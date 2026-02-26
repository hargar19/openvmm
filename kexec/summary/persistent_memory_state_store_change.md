## Summary of All Changes — Kexec Servicing State Persistence

### Problem Statement

OpenHCL's current servicing flow (VM live update) requires the **host** to orchestrate everything: save VM state → host receives it → host reloads VTL2 IGVM → new VTL2 fetches state from host → restore. This creates a hard dependency on the host for every step. The goal is to enable **kexec-based servicing** where VTL2 replaces itself via Linux kexec, persisting VM state in memory without host involvement — achieving faster, self-contained servicing.

### What Was Built

**6 commits, 9 files changed, +400/-64 lines** across the `user/hargar/kexec-2026` branch:

---

#### Commit 1: `loader/boot: reserve persisted memory region for servicing state blob`
**5 files** — Memory infrastructure for persisting state across kexec.

| File | Change |
|------|--------|
| shim.rs | Added `VTL2_PERSISTED_SERVICING_STATE = 13` memory type + 3 header fields (`servicing_state_base`, `servicing_state_region_len`, `servicing_state_payload_len`) |
| memory.rs | Added `PersistedServicingState` reserved type, 3-way split of persisted region (header 1 page, protobuf half, servicing half), bumped `MAX_RESERVED_MEM_RANGES` |
| mod.rs | Increased persisted region from 20→64 pages (256KB), added servicing region to restore path |
| main.rs | Added new memory type to `E820_RESERVED` mapping, updated 4 tests |
| lib.rs | Parse `vtl2_persisted_servicing_state` from device tree, with validation |

---

#### Commit 2: `underhill: fix config crash when measured config pages are zeroed after kexec`
**1 file** — Runtime crash fix.

| File | Change |
|------|--------|
| mod.rs | When `magic == 0` (pages zeroed by `Vtl2ParamsMap` drop), log info and use defaults instead of asserting |

**Problem**: `Vtl2ParamsMap` has `zero_on_drop: true`. After kexec, old VTL2 drops mappings → config pages zeroed → new VTL2 panics on `assert_eq!(magic, MAGIC)`.

---

#### Commit 3: `underhill: add read/write helpers for persisted servicing state`
**1 file** — Core persistence API.

| File | Change |
|------|--------|
| mod.rs | `write_servicing_state_to_persisted()` — writes state + updates header. `read_servicing_state_from_persisted()` — reads state + clears header + zeroes region (one-shot read). |

---

#### Commit 4: `underhill: persist servicing state before kexec exec`
**1 file** — Hook into the kexec path.

| File | Change |
|------|--------|
| mod.rs | `try_kexec_after_servicing` now receives `state_buf: &[u8]`, calls `write_servicing_state_to_persisted()` before exec'ing kexec. Falls back to host restart on persist failure. |

---

#### Commit 5: `underhill: skip sidecar and online CPUs during kexec servicing`
**2 files** — Match original servicing behavior.

| File | Change |
|------|--------|
| lib.rs | Added `skip_sidecar: bool` to `UhPartitionNewParams`. When true, returns `None` instead of calling `SidecarClient::new()` |
| worker.rs | Sets `skip_sidecar: true` for kexec. Onlines all CPUs before `spawn_vps` so all VPs go through `spawn_main_vp()` |

**Problem**: In kexec, `openhcl_boot` doesn't re-run (no `SidecarOptions::DisabledServicing`). The sidecar kernel from first boot is still running on APs. Without this fix, `SidecarClient::new()` succeeds → VPs incorrectly spawned as sidecar VPs.

---

#### Commit 6: `underhill: restore servicing state from persisted memory after kexec`
**1 file** — The restore path.

| File | Change |
|------|--------|
| worker.rs | When `OPENHCL_KEXEC_SERVICING` is set: parses device tree to find persisted region, calls `read_servicing_state_from_persisted()`, uses the blob for restore. Falls back to host if read fails. |

---

### Current State

- **Build**: Compiles cleanly, all 19 tests pass (17 pass + 2 ignored)
- **Deployment verified**: State persisted (0x12893 bytes / ~75KB), restored after kexec, VTL0 boots successfully
- **No uncommitted changes** — all 6 commits are clean

### Next Issue: Host Servicing Command Hangs

The host-side servicing PowerShell command (`Save-VM` or equivalent) **hangs indefinitely** after kexec. Root cause:

- In original servicing, `send_servicing_state()` sends the state blob to the host via GET protocol → host receives it → host reloads VTL2 → restore → host sees VM running → command completes
- In kexec, `try_kexec_after_servicing` fires **before** `send_servicing_state()` — the exec replaces the process, so the host never receives any acknowledgment
- Multiple approaches were tried and **all reverted**:
  - Sending an empty blob → host interprets it as "reload VTL2" → triggers second reload → `AccessDenied` crash
  - Adding a `KEXEC_SERVICING_COMPLETE` GET notification → vmwp.exe doesn't understand custom notifications
  - Host-side orchestration changes → reverted due to triggering reload
- The VM itself is fully operational after kexec; only the host command is stuck

**Resolution options** (require protocol/host changes):
1. A new GET protocol message the host understands (requires vmwp.exe changes)
2. Host-side timeout/detection that VTL2 restarted itself
3. Alternative signaling mechanism outside the GET protocol 
