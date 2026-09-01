# Kexec Servicing Optimization Plan

Goal: Reduce kexec servicing blackout time to match or beat normal (host round-trip) servicing.

## Current Measurements (1 VP, OpenVMM console)

| Flow | Blackout | Build |
|---|---|---|
| Normal servicing | **1.862s** | main (`d2c5f72`) |
| Kexec servicing (all optimizations) | **1.836s** | `user/hargar/kexec-2026` (`556800e0`) |
| Kexec servicing (before optimizations) | 2.640s | `user/hargar/kexec-2026` (`6261fbe`) |

Kexec is now **26ms faster** than normal servicing on console (1 VP). The real test is Hyper-V with 32 VPs where heavier stop/save/restore phases dominate.

## Environment Flags

Two env vars control kexec servicing:

- **`OPENHCL_SERVICING_RESTART_VIA_KEXEC=1`** — Baked into boot cmdline by `openhcl_boot`. Tells the running VTL2: "use kexec instead of host IGVM reload when servicing is requested." Present from initial boot.
- **`OPENHCL_KEXEC_SERVICING=1`** — Added by `prepare_kexec()` to the kexec'd kernel cmdline. Tells the new VTL2 after reboot: "you booted via kexec — read state from persisted memory, skip sidecar, skip self test." Only present after an actual kexec reboot.

## State Transfer Comparison

| | Normal servicing | Kexec servicing |
|---|---|---|
| Method | Host round-trip via GET | Persisted VTL2 memory region |
| Flow | VTL2 → `send_servicing_state()` → Host holds state → IGVM reload → New VTL2 → `get_saved_state_from_host()` → Host returns state | VTL2 → `write_servicing_state_to_persisted()` → `kexec -e` → New VTL2 reads from persisted memory |
| State size | 0xc40 (3136 bytes) | 0xc42 (3138 bytes) |
| Transfer time | ~10ms (GET RPC to local openvmm process) | <1ms (memcpy from reserved region), overlapped with GET negotiation |

On Hyper-V the host round-trip goes through vmwp.exe and the host kernel stack, so the savings should be larger.

## Phase-by-Phase Breakdown (console, 1 VP)

Timestamps are kernel-relative (seconds from kexec'd kernel start = 0.000).

| Phase | Normal | Kexec | Delta | Notes |
|---|---|---|---|---|
| Console enabled | 0.038s → 0.533s | 0.008s → 0.503s | -30ms | |
| Run /underhill-init | 0.944s | 0.871s | -73ms | Kexec skips boot shim |
| Module loading (3 modules) | 1.010s → 1.092s | 0.940s → 1.025s | -67ms | Same ~82ms in both |
| GET version negotiated | 1.116s | 1.047s | -69ms | |
| Servicing state acquired | 1.133s | ~1.047s | -86ms | Normal: host round-trip; Kexec: persisted read overlapped with GET |
| Boot log replay (8 lines) | 1.157s → 1.258s | *skipped* | **-101ms** | Stale first-boot logs |
| Guest memory self test | 1.416s → 1.429s | *skipped* | **-13ms** | Partition not torn down |
| Restore state_change | 1.530s (869μs) | 1.289s (900μs) | -241ms | |
| VM worker started | 1.555s | 1.309s | -246ms | |
| Start state_change | 1.616s (54.8ms) | 1.371s (57.4ms) | -245ms | |
| **Resuming VM** | **1.626s** | **1.377s** | **-249ms** | |

Note: Both flows share a fixed ~503ms serial 8250 init gap (console→APIC) that dominates the timeline.

The kexec blackout (1.836s) includes time before the kernel starts (kexec -e → first kernel log) which doesn't appear in the per-phase kernel timestamps above.

## Completed Optimizations

### 1. Reduce VTL2 memory size (128 MB)
- **Commit**: `5df196e7`
- **Change**: Increased `memory_page_count` to 32768 (128 MB) to accommodate kexec staging, down from 640 MB prototype.
- **Impact**: ~150ms vs 640 MB. Kernel memory init scales linearly with VTL2 size.

### 2. Skip background kexec pre-load after kexec boot
- **Commit**: `5df196e7`
- **Change**: Added `OPENHCL_KEXEC_SERVICING` guard so pre-load is skipped after kexec boot (initramfs lacks modules/kexec binary).
- **Impact**: Eliminates noisy warning lines from logs. No critical-path savings (pre-load is async).

### 3. Overlap GET negotiation with persisted state read
- **Commit**: `556800e0`
- **Change**: In `Worker::new()`, start GET negotiation as a future, then read persisted state in parallel. Await GET after state is already decoded.
- **Impact**: Absorbs the persisted memory read (~10ms) entirely behind the GET VMBUS round-trip.
- **Files**: `openhcl/underhill_core/src/worker.rs`

### 4. Reduce openhcl_boot log replay chatter
- **Commit**: `f4cec938`
- **Change**: Downgraded stale first-boot log replay from `info!` to `debug!` during kexec servicing.
- **Impact**: ~50ms saved from reduced serial I/O.
- **Files**: `openhcl/underhill_core/src/loader/vtl2_config.rs`

### 5. Skip guest memory self test on kexec boot
- **Commit**: `fe5f2152`
- **Change**: Skip self test when `OPENHCL_KEXEC_SERVICING` is set — partition not torn down, mappings unchanged.
- **Impact**: ~13ms.
- **Files**: `openhcl/underhill_core/src/worker.rs`

### 6. Move kexec pre-load before VM stop
- **Commit**: `f7b949dd`
- **Change**: `kexec -l` (initramfs build + kernel staging, ~1,420ms) now runs while the guest is still running, before `handle_servicing_inner()` stops the VM.
- **Impact**: ~800ms moved off the blackout critical path.
- **Files**: `openhcl/underhill_core/src/dispatch/mod.rs`, `openhcl/underhill_core/src/dispatch/kexec_prepare.rs`

### 7. Downgrade kexec-specific info! to debug!
- **Commit**: `57a878ec`
- **Change**: Downgraded 4 kexec-specific `info!` log lines to `debug!` to reduce serial I/O during blackout.
- **Impact**: ~10ms.

## Removed

### ~~Persist topology for openhcl_boot~~ — Not applicable
- After kexec, `openhcl_boot` does **not run**. `kexec -l` loads the Linux kernel directly. The DTB is preserved by kexec itself (`kexec: preserving current DTB`).

## Planned

### 8. Build kernel modules into the kernel (built-in instead of loadable)
- **Estimated savings**: ~82ms
- **Difficulty**: Easy (kernel config change)
- **Details**: The kexec'd kernel reloads `pci-hyperv-intf.ko`, `pci-hyperv.ko`, `hv_storvsc.ko` from initramfs (~82ms). If compiled built-in (`=y` instead of `=m`), module loading is eliminated.
- **Files**: Kernel `.config`, `openhcl/underhill_init/src/main.rs`

## Summary

| # | Optimization | Savings | Status |
|---|---|---|---|
| 1 | Reduce VTL2 memory (128 MB) | ~150ms | Done (`5df196e7`) |
| 2 | Skip background pre-load after kexec boot | Logs cleanup | Done (`5df196e7`) |
| 3 | Overlap GET negotiation with persisted state read | ~10ms | Done (`556800e0`) |
| 4 | Reduce boot log replay chatter | ~50ms | Done (`f4cec938`) |
| 5 | Skip guest memory self test | ~13ms | Done (`fe5f2152`) |
| 6 | Move kexec pre-load before VM stop | ~800ms | Done (`f7b949dd`) |
| 7 | Downgrade kexec-specific info! to debug! | ~10ms | Done (`57a878ec`) |
| 8 | Built-in kernel modules | ~82ms | Planned |

## Log Files

- Normal servicing (console, 1 VP): `kexec/openvmm-console/servicing-logs`
- Kexec servicing (console, 1 VP, all optimizations): `kexec/openvmm-console/kexec-logs`
- Kexec servicing (console, 1 VP, before preload move): `kexec/openvmm-console/kexec-logs-withoutPreload`
- Hyper-V servicing (32 VPs): `kexec/servicing_logs`
