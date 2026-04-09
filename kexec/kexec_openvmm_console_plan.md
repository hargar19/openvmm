# Plan: Hook Kexec into OpenVMM Console `service-vtl2`

## Current State of Affairs

### The Problem
On Hyper-V, `Restart-Underhill` performs an **atomic save+reload**: once VTL2 sends saved state back to the host, Hyper-V immediately proceeds to tear down VTL2 (stop VPs, reset VMBus, reload firmware). With kexec, VTL2 never sends state back because it execs into a new kernel — so the command **hangs forever** waiting for state.

### What the Branch Does Today (Guest-Side Kexec)
The branch hooks into **underhill's servicing flow** (`handle_servicing_request` in `openhcl/underhill_core/src/dispatch/mod.rs`). When the host sends a "save VTL2 state" notification via the GET protocol:

1. Underhill serializes its state (`handle_servicing_inner` → state blob)
2. **Kexec hook** (`try_kexec_after_servicing`) runs **before** sending state to the host:
   - Writes the state blob to a reserved memory region (`VTL2_PERSISTED_SERVICING_STATE`)
   - Execs `kexec -e` (or full `kexec_test.sh` if not pre-loaded)
3. If kexec succeeds, the process never returns. The new VTL2 boots, reads persisted state from memory, and restores.
4. If kexec fails, falls through to normal host-driven path (sends state to host).

**But this requires Hyper-V to initiate the servicing request first** — which is the `Restart-Underhill` command that hangs.

### What the OpenVMM Team Suggested
Use the **OpenVMM console** (the `openvmm>` interactive CLI, available via Ctrl-Q) instead of Hyper-V's `Restart-Underhill`. OpenVMM has its own implementation of the entire save+reload flow that we can modify.

---

## How OpenVMM Console Servicing Works Today

### Chain
`openvmm>` console → `service-vtl2` command → `openvmm/openvmm_entry/src/lib.rs` (line ~2856)

### Flow
```
save_underhill()    ← openvmm/openvmm_helpers/src/underhill.rs
  ├─ VmRpc::StartReloadIgvm(file)     → stages new IGVM in memory
  └─ GuestEmulationRequest::SaveGuestVtl2State(flags)
       → GED sends SaveGuestVtl2StateNotification to guest over VMBus
       → Guest (underhill) does handle_servicing_inner() → serialize state
       → Guest sends state back in chunks (SaveGuestVtl2StateRequest)
       → GED accumulates chunks in save_restore_buf

restore_underhill()
  ├─ VmRpc::CompleteReloadIgvm(true)
  │    → Stop VPs
  │    → Reset VTL2 VMBus
  │    → Reload VTL2 firmware from staged IGVM
  │    → Resume VPs
  └─ GuestEmulationRequest::WaitForVtl0Start()
       → Wait for new VTL2 to report VTL0 started
```

### Key Files
| File | Role |
|------|------|
| `openvmm/openvmm_entry/src/lib.rs` | Console command definition + handler (~L1989, ~L2856) |
| `openvmm/openvmm_helpers/src/underhill.rs` | `save_underhill()` and `restore_underhill()` orchestration |
| `openvmm/openvmm_defs/src/rpc.rs` | `VmRpc::StartReloadIgvm`, `VmRpc::CompleteReloadIgvm` |
| `vm/devices/get/guest_emulation_device/src/lib.rs` | GED — host-side GET protocol (save/restore state relay) |
| `vm/devices/get/get_resources/src/lib.rs` | `GuestEmulationRequest` enum, `GuestServicingFlags` |
| `openvmm/openvmm_core/src/worker/dispatch.rs` | `start_reload_igvm()`, `complete_reload_igvm()` (~L2992, ~L3004) |
| `openhcl/underhill_core/src/dispatch/mod.rs` | Guest-side `handle_servicing_request`, `try_kexec_after_servicing` |
| `openhcl/underhill_core/src/worker.rs` | Guest-side new-instance boot + state restore logic |

---

## Plan: Add a Kexec Mode to `service-vtl2`

### Phase 1: New Console Command Flag

Add a `--kexec` flag to the `ServiceVtl2` clap struct.

**File:** `openvmm/openvmm_entry/src/lib.rs` (~L1989)
- Add `kexec: bool` to the `ServiceVtl2` struct.

**File:** `openvmm/openvmm_entry/src/lib.rs` (~L2856)
- In the handler, if `kexec` is set, call a new `kexec_service_underhill()` instead of the normal `save_underhill()` + `restore_underhill()`.

### Phase 2: New Host-Side Flow (`kexec_service_underhill`)

**File:** `openvmm/openvmm_helpers/src/underhill.rs` (new function)

The kexec flow differs from normal servicing:

```
kexec_service_underhill()
  ├─ GuestEmulationRequest::SaveGuestVtl2State(flags)
  │    → Send save notification to guest (fire-and-forget, don't await)
  │    → Guest saves state to persisted memory, does kexec -e
  │    → Guest NEVER sends state back (kexec replaces process)
  │
  ├─ GuestEmulationRequest::WaitForVtl0Start()
  │    → Wait for the new VTL2 to boot and report VTL0 started
  │
  └─ Drop/cancel the pending GED save RPC

  KEY DIFFERENCES from normal servicing:
  - No StartReloadIgvm — kexec loads the new kernel from within VTL2
    (via kexec -l / kexec -e), no host-side IGVM staging needed.
  - No CompleteReloadIgvm — kexec already replaced VTL2; no need
    to stop VPs / reset VMBus / reload firmware from the host.
  - No state flows through the host — persisted memory region is
    the sole transfer mechanism between old and new VTL2.
  - This is why we use the OpenVMM console path: to bypass
    Hyper-V's atomic save+reload protocol entirely.
```

### Phase 3: Two Design Options

#### Option A: "Skip Reload" — Simplest (RECOMMENDED for prototype)

1. Console sends `SaveGuestVtl2State` notification to guest (fire-and-forget, don't await the RPC)
2. Guest (underhill) saves state → persists to reserved memory → `kexec -e`
3. New VTL2 kernel boots, underhill starts, reads persisted state directly from reserved memory, restores
4. **No** `StartReloadIgvm` — kexec loads the kernel from within VTL2
5. **No** `CompleteReloadIgvm` — kexec already replaced VTL2
6. Call `WaitForVtl0Start()` to confirm the new VTL2 is up
7. Cancel/drop the pending GED save RPC (it will never complete since the guest kexec'd away)

**Key insight:** No state flows through the host at all. The persisted memory
region (`VTL2_PERSISTED_SERVICING_STATE`) is the transfer mechanism. The new
VTL2 instance reads directly from it — no `get_saved_state_from_host()` and
no `send_servicing_state()` needed for this path. No IGVM staging needed
either — kexec handles its own kernel loading.

**Pros:** Zero new RPC types, no host involvement in state transfer or kernel loading, fastest possible kexec.
**Required changes:**
- Host side: fire-and-forget the save notification, then just wait for VTL0 start.
- Guest side: already works — reads persisted state from memory (existing code in `worker.rs`).

#### Option B: "New RPC" — Most Correct

1. **New RPC: `VmRpc::KexecServiceVtl2`** — Sends save notification but does NOT stage IGVM or reload
2. Guest saves state → persists to memory → `kexec -e`
3. Save notification times out (expected — guest kexec'd away)
4. **`GuestEmulationRequest::WaitForVtl0Start`** — Wait for new VTL2 to boot
5. Done. No `StartReloadIgvm` or `CompleteReloadIgvm` needed.

### Phase 4: Guest-Side Changes

The guest side (underhill) **already does most of what's needed**:

- `handle_servicing_request` already calls `try_kexec_after_servicing`
- State is persisted to reserved memory
- New instance reads it back from persisted memory and restores

**No additional guest-side changes needed for the OpenVMM console path.**
The new VTL2 instance already reads persisted state from the reserved memory
region (existing code in `worker.rs` under `kexec_servicing` branch). It does
not need to send state to the host or fetch it from the host — the persisted
memory region is the sole transfer mechanism.

### Phase 5: Concrete File Changes

| File | Change |
|------|--------|
| `openvmm/openvmm_entry/src/lib.rs` (~L1989) | Add `--kexec` flag to `ServiceVtl2` |
| `openvmm/openvmm_entry/src/lib.rs` (~L2856) | Branch on `kexec` flag to call new helper |
| `openvmm/openvmm_helpers/src/underhill.rs` | Add `kexec_service_underhill()` function |
| `openvmm/openvmm_defs/src/rpc.rs` | Possibly add new RPC variant (Option B only) |
| `vm/devices/get/guest_emulation_device/src/lib.rs` | May need cancel/drop logic for save RPC (guest kexec'd away) |
| `vm/devices/get/get_resources/src/lib.rs` | May need new `GuestEmulationRequest` variant (Option B only) |

---

## Recommended Starting Point (Option A Prototype)

1. Add `--kexec` to `ServiceVtl2` console command
2. In the kexec path: send `SaveGuestVtl2State` notification (fire-and-forget, don't await)
3. Guest saves state to persisted memory, does `kexec -e`
4. New VTL2 boots, reads persisted state directly from reserved memory, restores
5. **No** `StartReloadIgvm` — kexec loads the kernel itself
6. **No** `CompleteReloadIgvm` — kexec already replaced VTL2
7. Call `WaitForVtl0Start()` to confirm the new VTL2 is up
8. Drop/cancel the pending GED save RPC

**No state or kernel image goes through the host.** The persisted memory region
is the sole state transfer mechanism, and kexec handles kernel loading internally.
This is the entire reason for using the OpenVMM console path — to bypass
Hyper-V's atomic save+reload protocol.
