# Servicing flow (OpenHCL/Underhill) — walkthrough with log anchors

This document explains the **servicing** flow as implemented/observed in the openvmm repo, using [kexec/servicing_logs](kexec/servicing_logs) as the canonical example.

Servicing is a **VTL2 (Underhill/OpenHCL) reload** with continuity: the old VTL2 instance quiesces the VM and **saves a servicing-state blob** to the host; then a new VTL2 instance boots, **fetches the blob back**, restores internal/device state, and finally **resumes the VM**.

## Block diagram

```text
                          +------------------------+
                          | Trigger servicing-save |
                          +-----------+------------+
                                      |
                                      v
  +-----------------------------------------------------------------------+
  |                   Blackout window (VPs paused)                         |
  |                                                                       |
  |  +---------------------------+                                        |
  |  | Stop graph (quiesce VM)   |  Stop VPs + stop device/unit graph     |
  |  +-------------+-------------+                                        |
  |                |                                                      |
  |                v                                                      |
  |  +---------------------------+                                        |
  |  | Save graph                |  Serialize unit/device state           |
  |  +-------------+-------------+                                        |
  |                |                                                      |
  |                v                                                      |
  |  +---------------------------+                                        |
  |  | Stream blob to host       |  GET/GED, then await host response     |
  |  +-------------+-------------+                                        |
  |                |                                                      |
  |                v                                                      |
  |  +---------------------------+                                        |
  |  | Restart boundary          |  VTL2 jumps to new kernel              |
  |  +-------------+-------------+                                        |
  |                |                                                      |
  |                v                                                      |
  |  +---------------------------+                                        |
  |  | New VTL2 boots            |  kernel + Underhill init/userspace     |
  |  +-------------+-------------+                                        |
  |                |                                                      |
  |                v                                                      |
  |  +---------------------------+                                        |
  |  | Fetch blob from host      |  new VTL2 requests saved state         |
  |  +-------------+-------------+                                        |
  |                |                                                      |
  |                v                                                      |
  |  +---------------------------+                                        |
  |  | Restore graph             |  Apply blob to units/devices           |
  |  +-------------+-------------+                                        |
  |                |                                                      |
  |                v                                                      |
  |  +---------------------------+                                        |
  |  | Start graph               |  Start units/devices                   |
  |  +-------------+-------------+                                        |
  +-----------------------------------------------------------------------+
                                      |
                                      v
                          +------------------------+
                          | Resume VM              |
                          | End blackout           |
                          +------------------------+
```

Notes:
- The 4 **graph transitions** are: `stop` → `save` → `restore` → `start`.
- The blackout window includes more than those graphs (notably the blob streaming, the restart boundary, and parts of new-kernel bring-up).

## Key milestones (log snapshots)

The milestones below correspond to the anchor points in the servicing logs.

### 1) Stop complete (stop graph finished)

Anchor: [kexec/servicing_logs](kexec/servicing_logs#L120)

Meaning:
- Underhill has finished dependency-ordered stopping of its unit/device graph.
- VPs are stopped during this window (you’ll see many `stopping VP` lines earlier).

Snapshot:
```text
<30>[   23.923103] state_unit:  INFO servicing_save_vtl2{ correlation_id=2f92653f-e6e0-43e9-948f-b86bda745813}:state_change{ operation="stop"}:  state change complete duration=944.577613ms
```

### 2) Save complete (save graph finished)

Anchor: [kexec/servicing_logs](kexec/servicing_logs#L151)

Meaning:
- Units/devices finished serializing their saveable state into the servicing-state payload.

Snapshot:
```text
<30>[   24.294573] state_unit:  INFO servicing_save_vtl2{ correlation_id=2f92653f-e6e0-43e9-948f-b86bda745813}:save_units:state_change{ operation="save"}:  state change complete duration=332.842463ms
```

### 3) Blob sent to host (GET/GED stream complete)

Anchor: [kexec/servicing_logs](kexec/servicing_logs#L182)

Meaning:
- VTL2 finished streaming the saved-state blob to the host.
- The old VTL2 instance now waits for the host to proceed (typically: perform the VTL2 reload boundary).

Snapshot:
```text
<31>[   24.529047] guest_emulation_transport::process_loop: DEBUG  More data? SUCCESS saved_state_bytes_written 73728 saved_state_size 75793, payload_len 2065
<31>[   24.536855] guest_emulation_transport::process_loop: DEBUG  Done writing saved state, awaiting host response
```

### 4) Restart boundary + new kernel

Anchor: [kexec/servicing_logs](kexec/servicing_logs#L192-L193)

Meaning:
- Servicing crosses the reboot boundary for VTL2.
- In this log, the transition is visible as the “jump to kernel” line followed by a fresh kernel boot at timestamp 0.0.

Snapshot:
```text
uninitializing hypercalls, about to jump to kernel
<5>[    0.000000] Linux version 6.12.52-microsoft-hcl ...
```

Also visible during this early boot:
- Sidecar is disabled because this is a servicing restore:
  - [kexec/servicing_logs](kexec/servicing_logs#L189-L191)

### 5) Fetch blob (new VTL2 gets servicing state from host)

Anchor: [kexec/servicing_logs](kexec/servicing_logs#L428)

Meaning:
- The new Underhill instance is up far enough to establish transport and request the saved blob.

Snapshot:
```text
<30>[    1.131261] underhill_core::worker:  INFO worker_new{ name="UnderhillWorker" action="new"}:init:  VTL2 restart, getting servicing state from the host
<30>[    1.131670] underhill_core::worker:  INFO worker_new{ name="UnderhillWorker" action="new"}:init:  received servicing state from host saved_state_len=0x12811
```

### 6) Restore complete (restore graph finished)

Anchor: [kexec/servicing_logs](kexec/servicing_logs#L737)

Meaning:
- The new Underhill instance has applied the saved blob to rebuild internal/device state.

Snapshot:
```text
<30>[    4.576248] state_unit:  INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/restore{ correlation_id=2f92653f-e6e0-43e9-948f-b86bda745813}:restore_units:state_change{ operation="restore"}:  state change complete duration=1.144605085s
```

### 7) Start complete (start graph finished)

Anchor: [kexec/servicing_logs](kexec/servicing_logs#L798)

Meaning:
- Units/devices have been started and the system is ready to resume execution.

Snapshot:
```text
<30>[    5.025364] state_unit:  INFO state_change{ operation="start"}:  state change complete duration=436.10605ms
```

### 8) Resume + blackout time

Anchor: [kexec/servicing_logs](kexec/servicing_logs#L799)

Meaning:
- Underhill resumes the VM (VPs run again).
- `blackout_time` is the end-to-end pause duration as tracked by Underhill, spanning more than just the 4 state graphs.

Snapshot:
```text
<30>[    5.030935] underhill_core::dispatch:  INFO  resuming VM correlation_id=2f92653f-e6e0-43e9-948f-b86bda745813 blackout_time="6.7179477s"
```

## Why blackout_time is larger than (stop+save+restore+start)

In this run:
- stop + save + restore + start ≈ 2.86s total (sum of the four “graph complete” durations)
- blackout_time ≈ 6.72s

The difference is expected because blackout includes:
- time to stream the blob (after save completes, before the restart boundary)
- the restart boundary itself
- new kernel boot + early init + Underhill userspace bring-up required before restore can even start
- transport re-establishment before fetching the blob

## Quick grep for these anchors

```bash
grep -n -E "state_change complete duration=944\.577613ms|state change complete duration=332\.842463ms|Done writing saved state, awaiting host response|uninitializing hypercalls, about to jump to kernel|Linux version|VTL2 restart, getting servicing state from the host|state change complete duration=1\.144605085s|state change complete duration=436\.10605ms|resuming VM" kexec/servicing_logs
```
