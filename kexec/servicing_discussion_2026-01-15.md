# OpenVMM / Underhill (OpenHCL) Servicing — Discussion Notes (2026-01-15)

Highlevel: Servicing reboots VTL2 by creating a save/restore boundary around the VTL2 instance: VTL2 quiesces, saves its servicing state, and sends the blob to the host; then the host triggers a VTL2 reload (a reset boundary for the management VTL, not a full VM reboot). A fresh VTL2 instance boots, detects it’s a servicing restore, pulls the saved blob from the host, restores device/worker state, and resumes. 

These notes capture our discussion about “servicing” an Underhill/OpenHCL VM in this repo, including what the restart command means, how the save/reload/restore pipeline works (OpenVMM vs Hyper-V), and how your `kexec/servicing_logs` confirm the flow.

## Terminology

- **Underhill**: a codename used in this repo/ecosystem for **OpenHCL**, the VTL2 “paravisor” that runs alongside the guest.
- **VTL0**: the “normal” guest OS (what most people think of as “the VM”).
- **VTL2**: the management/paravisor environment (OpenHCL/Underhill).
- **Servicing**: a process to **reload/replace VTL2 (OpenHCL)** while preserving continuity by **saving state**, doing the reload, then **restoring state**.

Important nuance: servicing is **not necessarily a full VM reboot**. The system is primarily reloading **VTL2**, not rebooting VTL0.

## The PowerShell command and what it means

You asked:

```powershell
Restart-Underhill -Name $vmName -OverrideVersionChecks
```

Key point from repo inspection:

- The repo did not define a function named `Restart-Underhill` directly.
- The repo contains a related Hyper-V PowerShell cmdlet named **`Restart-OpenHCL`** (same conceptual operation: restart/reload the management VTL / OpenHCL).
- In the Hyper-V path, `-OverrideVersionChecks` maps to a flag bit in an **Options** field passed to a Hyper-V CIM method.

### Hyper-V backend (PowerShell/CIM)

In the Hyper-V backend (via Petri), the restart operation is implemented by calling the Hyper-V guest management service CIM method **`ReloadManagementVtl`** (class `Msvm_VirtualSystemGuestManagementService`).

- `-OverrideVersionChecks` sets an option bit (bit 0 / value 1) that tells the host to bypass version compatibility checks when reloading.
- The same Options bitmask is also used for other toggles (e.g., disabling NVMe keepalive in some scenarios).

This repo shows how the CIM call is *invoked*; the full implementation of `ReloadManagementVtl` itself lives in the Hyper-V host stack.

## Servicing flow in OpenVMM (Rust path)

This repo contains a full “OpenVMM backend” implementation of servicing, where the host-side orchestrator (Petri/OpenVMM worker) coordinates with the guest-side VTL2 (OpenHCL/Underhill).

### High-level sequence

1. **Stage the replacement image** (IGVM) on the host side.
2. Ask VTL2 to **save its servicing state** and send it to the host.
3. Host performs a VTL2 reload (firmware reload / VTL2 reset boundary).
4. New VTL2 instance boots and detects it is a **servicing restore**.
5. New VTL2 instance **fetches the saved state from host** and restores internal state and emulated devices.
6. Host/guest resume normal execution.

### What happens to VPs during servicing?

You asked whether VPs (virtual processors) remain stopped afterward.

Conclusion from the OpenVMM worker reload code path:

- VPs are **stopped temporarily** during the critical “reload VTL2” window.
- After the reload finishes, the worker **resumes VPs** (in code this is done by dropping a scoped “stop VPs” guard).
- So: **they do not remain stopped** once servicing completes.

## How your `kexec/servicing_logs` confirm the flow

Your log shows the expected “save → reboot boundary (VTL2) → restore → resume” pattern.

### A) Save phase (VTL2 prepares and streams state to host)

The log begins a servicing save with device/VP quiesce and device stop:

- `underhill_core::dispatch:  INFO servicing_save_vtl2 ... stopping VM`
- `vmm_core::partition_unit::vp_set: ... stopping VP`

Then you see GET/GED chunking of the saved-state payload:

- `guest_emulation_transport::process_loop: DEBUG  More data? ...`
- culminating in:

```text
[   24.536855] guest_emulation_transport::process_loop: DEBUG  Done writing saved state, awaiting host response
```

Interpretation:

- VTL2 has finished **sending the saved servicing state to the host**.
- It is now waiting for the host to respond (ack / proceed). In practice, the host often initiates the reload promptly.

### B) The “reload boundary” (new VTL2 boots)

Immediately after the “awaiting host response” line you see fresh boot logs starting at time ~0.0 (kernel boot), which indicates a **new VTL2 instance** is running.

This matches the core servicing design: the *old* VTL2 instance saved state and then the host reloaded VTL2.

### C) Restore phase (new VTL2 pulls state back from host)

Later, the new Underhill instance says:

- `VTL2 restart, getting servicing state from the host`
- `received servicing state from host saved_state_len=...`

That is the concrete evidence that the saved-state blob is being reused to restore continuity.

### D) VM resumes

After restore, your log includes:

- `underhill_core::dispatch:  INFO  resuming VM ... blackout_time=...`

This aligns with our earlier conclusion that VP stop/quiesce is scoped to the servicing window and does not persist.

## Answers to the key questions you asked

### 1) Does “awaiting host response” mean state is saved for later reuse?

Yes.

That line indicates VTL2 has finished transmitting the saved servicing-state to the host so that, after the VTL2 reload, the new VTL2 instance can restore from that same state.

In your log, this is corroborated by the later messages where the new VTL2 instance fetches `saved_state_len=...` from the host.

### 2) Does servicing restart the whole VM?

Not necessarily.

In this design, servicing is primarily a **VTL2 restart/reload** (OpenHCL/Underhill), while trying to keep the overall VM experience continuous.

### 3) Do VPs remain stopped if the VM isn’t “restarted”?

No.

In OpenVMM’s servicing pipeline, VP stopping is a temporary safety mechanism during reload; VPs are resumed afterward.

## Practical “mental model”

Think of servicing as:

- **Checkpoint VTL2** → **swap/boot new VTL2** → **restore checkpoint**

…and the host uses GET/GED to transport the checkpoint blob between the old VTL2 and the new VTL2 instance.

---

## Kexec-servicing parity checklist (anchored to `kexec/servicing_logs`)

Goal: replace only the **restart boundary** with in-guest Linux `kexec`, while keeping the **save-to-host / restore-from-host** semantics identical to classic servicing.

### Phase 0: Preconditions (before the save)

- [ ] **Payload staging is complete** (new kernel+initrd+cmdline are available inside VTL2 *before* quiesce).
- [ ] **Payload validation is complete** (hash/signature/version gating). Decide how `-OverrideVersionChecks` maps into this.
- [ ] **Host-side saved-state storage is provisioned** (where the blob lives across the restart boundary, retention policy, cleanup on success/failure).

What the baseline log shows:
- Servicing starts without needing to “download” anything during the blackout window.

### Phase 1: Quiesce + Stop (enter blackout window)

Baseline anchors:
- `servicing_save_vtl2 ... stopping VM` at line 1
- Device graph stops: `state_change{ operation="stop" } ...` near the beginning

Checklist:
- [ ] **Stop/hold VM execution deterministically** (VP stop + device stop in a well-defined order).
- [ ] **Drain/flush work queues** so no new device/VMBus work races with saving.
- [ ] **Define “blackout window” start** (same point across runs so you can compare blackout time).

Kexec equivalence requirement:
- The kexec path should enter the blackout window at the same point the classic path does (or earlier), otherwise device state may not be safe to serialize.

### Phase 2: Save units + Serialize servicing state

Baseline anchors:
- `save_units:state_change{ operation="save" } ...` around lines 124–151

Checklist:
- [ ] **Serialize all required device/unit state** (the set must match what restore expects).
- [ ] **Version the blob format** and include compatibility checks.
- [ ] **Make the save idempotent** (if retry is possible, blob must be well-defined).

### Phase 3: Transfer saved blob to host + handshake

Baseline anchor:
- `Done writing saved state, awaiting host response` at line 182

Checklist:
- [ ] **Complete GET/GED transfer** and verify integrity on the host side.
- [ ] **Host ACK/COMMIT** protocol: host explicitly confirms it has stored the blob and that VTL2 is allowed to proceed to the restart boundary.
- [ ] **Timeout and rollback policy** if host does not ACK (resume old VTL2? abort servicing? retry?).

Kexec equivalence requirement:
- Do not execute kexec until the host has committed the saved blob.

### Phase 4: Restart boundary (this is the part you replace with Linux kexec)

Baseline anchors:
- `uninitializing hypercalls, about to jump to kernel` at line 192
- Fresh boot begins: `Linux version ...` at line 193

Checklist:
- [ ] **Quiesce final low-level platform state** (hypercalls/interrupts/log flushing) equivalent to the classic “jump to kernel” path.
- [ ] **Perform Linux kexec** (`kexec -l`/`kexec -e` or syscalls) only after host ACK.
- [ ] **Preserve/construct correct boot params** (cmdline, initrd, DTB). Your separate kexec log confirms DTB preservation is happening in your current setup.
- [ ] **Failure behavior**: if `kexec -e` fails, you must be able to safely resume (or fail closed) without corrupting state.

### Phase 5: New VTL2 boot recognizes “servicing restore” mode

Baseline indicators (seen during the restore boot):
- `found persisted state header`
- `disabling sidecar, as we are restoring from persisted state`

Checklist:
- [ ] **Ensure the new kernel’s early boot can determine it is a servicing restore** (via DPS + persisted header + env/cmdline conventions).
- [ ] **Ensure sidecar policy matches baseline** (disabled during restore, then re-enabled if/when appropriate).

### Phase 6: New VTL2 fetches blob from host + restore

Baseline anchors:
- `VTL2 restart, getting servicing state from the host` at line 428
- `received servicing state from host saved_state_len=...` at line 429

Checklist:
- [ ] **Bring up transport early enough** that “get saved state from host” works reliably.
- [ ] **Validate blob integrity/version** before applying restore.
- [ ] **Restore device/VMBus state** to the same lifecycle point as baseline (channels move through restored states; devices rebind as needed).
- [ ] **Keep restore non-panicking** across trust boundaries (treat host inputs and blob inputs as untrusted).

### Phase 7: Start units + resume execution (exit blackout window)

Baseline anchors:
- `state_change{ operation="start" } ... state change complete` near the end
- `resuming VM ... blackout_time="6.7179477s"` at line 799

Checklist:
- [ ] **Start device graph** in the correct order.
- [ ] **Resume VPs** and verify the blackout window ends.
- [ ] **Emit a single “servicing complete” marker** (easy to grep) including blackout time.

### Operational parity items (the usual “last 20%”)

- [ ] **Crash/failure recovery**: policy for (a) kexec fails, (b) new kernel boots but restore fails, (c) host loses blob.
- [ ] **Cleanup**: host drops saved blob only after new VTL2 confirms restore success.
- [ ] **Observability**: stable correlation id across old/new VTL2; explicit phase markers.
- [ ] **Compatibility controls**: reproduce `OverrideVersionChecks` semantics (strict by default, override only when requested).

### Quick verification commands

To sanity-check a run matches the baseline phases:

```bash
grep -n -E "servicing_save_vtl2|Done writing saved state|uninitializing hypercalls, about to jump to kernel|Linux version|VTL2 restart, getting servicing state from the host|received servicing state from host|resuming VM" kexec/servicing_logs
```

---
