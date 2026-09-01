# Servicing vs Kexec: Blackout Time Deep Dive

## 1. How blackout_time Is Calculated

### Clock Source
The blackout time uses the **Hyper-V partition reference time** — a 64-bit counter
in 100ns units that starts at 0 on VM boot and ticks continuously as long as the
partition exists. It does **not** reset across kexec or servicing restarts (the
partition is never torn down). It may pause if the VM is paused by the host.

**Implementation**: `openhcl/underhill_core/src/reference_time.rs`

### Start Point
Captured in `dispatch/mod.rs` `stop()` method (line ~924):
```rust
self.last_state_unit_stop = Some(ReferenceTime::new(self.partition.reference_time()));
self.state_units.stop().await;
```
This records the hypervisor reference time at the moment VTL2 issues the VM stop,
just before `state_units.stop()` runs.

### End Point
Captured in `dispatch/mod.rs` `start()` method (line ~893), **after**
`state_units.start()` completes on the new instance:
```rust
self.state_units.start().await;
let reference_time = ReferenceTime::new(self.partition.reference_time());
let blackout_time = reference_time.since(stopped);
```

### Persistence Across Restart
- **Servicing**: `vm_stop_reference_time` is serialized into `ServicingState`, sent
  to the host via GET, and restored on the new instance (worker.rs line ~3669).
- **Kexec**: `vm_stop_reference_time` is written to the persistent memory region
  and read back by the new instance directly.

### Blackout Window Definition
```
blackout = time(state_units.start() completes on NEW instance)
         - time(state_units.stop() called on OLD instance)
```
This covers: save → shutdown → bootloader → kernel boot → userspace init → VMM
startup → state restore → VM resume.

---

## 2. Reported Blackout Times

| Path | blackout_time_ms | blackout_time |
|------|-----------------|---------------|
| **Servicing** | 0x746 | **1.8624289s** |
| **Kexec** | 0x72c | **1.8362337s** |
| **Delta** | -26ms | kexec is ~26ms faster |

---

## 3. Stage-by-Stage Comparison

All times are Linux kernel timestamps (seconds since kernel start) unless noted.
Pre-restart times are from the old instance's clock.

### 3.1 Pre-Restart (Old Instance)

| Stage | Servicing | Kexec | Notes |
|-------|-----------|-------|-------|
| VM stop initiated | 13.210s | 39.240s | Absolute time (irrelevant for blackout) |
| state_units.stop() complete | +60ms (13.270) | +62ms (39.302) | ~same |
| Save state (serialize) | +21ms (13.291) | +24ms (39.326) | ~same |
| NVMe shutdown + log flush | +30ms (13.321) | +38ms (39.364) | ~same |
| Write state to persist region | N/A | +19ms (39.383) | Kexec-only: writes to persistent memory |
| Trigger restart | Host-driven | +24ms (39.412) | Kexec: runs `kexec -e` |

### 3.2 Bootloader Phase

| Stage | Servicing | Kexec | Notes |
|-------|-----------|-------|-------|
| openhcl_boot total | **21ms** (elapsed=21.0346ms) | **SKIPPED** | Kexec jumps directly to kernel |
| - found persisted state header | ✓ | — | |
| - reading topology | ✓ | — | |
| - decoding protobuf (410 bytes) | ✓ | — | |
| - bump allocator | ✓ | — | |
| - reclaim device tree memory | ✓ | — | |
| - sidecar init | ✓ | — | |
| - uninitializing hypercalls | ✓ | — | |

### 3.3 Kernel Boot (New Instance)

| Stage | Servicing | Kexec | Delta | Notes |
|-------|-----------|-------|-------|-------|
| Early boot → console enabled | 0→0.037 (37ms) | 0→0.008 (8ms) | **-29ms** | x2apic already enabled in kexec |
| Console → APIC setup | 0.037→0.533 (496ms) | 0.008→0.503 (495ms) | ~same | **Dominant cost**: serial console init |
| APIC → SMP boot | 0.533→0.597 (64ms) | 0.503→0.560 (57ms) | -7ms | |
| SMP → devtmpfs | 0.597→0.603 (6ms) | 0.560→0.570 (10ms) | ~same | |
| VMBus init | 0.632→0.634 | 0.606→0.609 | ~same | |
| Serial 8250 driver | 0.674→0.766 (92ms) | 0.620→0.712 (92ms) | ~same | |
| Initramfs unpack start | 0.651 | 0.616 | — | |
| Freeing initrd memory | 0.914 | 0.830 | — | |
| → Run /underhill-init | **0.943s** | **0.870s** | **-73ms** | |

**Kernel boot total**: Servicing ~943ms, Kexec ~870ms

### 3.4 Userspace Init (underhill_init)

| Stage | Servicing | Kexec | Delta |
|-------|-----------|-------|-------|
| init start | 0.973 | 0.905 | — |
| Mount filesystems | 0.986 | 0.917 | ~same |
| Register vfio-pci | 1.005 | 0.934 | ~same |
| Load pci-hyperv-intf.ko | 1.010→1.023 | 0.940→0.955 | ~same |
| Start openvmm_hcl | 1.015 | 0.947 | ~same |
| Load pci-hyperv.ko | 1.029→1.049 | 0.964→0.989 | ~same |
| Load hv_storvsc.ko | 1.069→1.092 | 1.006→1.024 | ~same |

### 3.5 VMM Startup (openvmm_hcl)

| Stage | Servicing | Kexec | Delta | Notes |
|-------|-----------|-------|-------|-------|
| VMM process start | 1.039 | 0.971 | — | |
| Trace filter set | 1.059 | 0.995 | ~same | |
| Boot loader times logged | 1.063 | 0.998 | — | Svc: 21ms, Kexec: 131ms |
| Diag server start | 1.080 | 1.015 | ~same | |
| GET version negotiated | 1.116 | 1.047 | ~same | NICKEL_REV2 |
| **Get servicing state from host** | **1.123** | **SKIPPED** | **-17ms** | Servicing: host roundtrip |
| **Receive servicing state** | **1.133** | **—** | | saved_state_len=0xc40 |
| Kernel boot time logged | 1.144 | 1.054 | — | |
| **openhcl_boot log replay** | **1.156→1.257** | **SKIPPED** | **-101ms** | 8 log lines printed to console |
| Memory allocation mode | 1.272 | 1.068 | — | |
| VTL2 RAM setup | 1.283 | 1.080 | ~same | |
| Flush logs | 1.307 | 1.105 | ~same | |
| VTL0 RAM / MMIO setup | 1.319→1.333 | 1.118→1.128 | ~same | |
| Alias map enable | 1.357 | 1.151 | ~same | |
| Hypercall allow map | 1.370 | 1.166 | ~same | |
| Memory map creation | 1.382 | 1.178 | ~same | |
| DMA manager | 1.396 | 1.189 | ~same | |
| Guest memory self test | 1.416→1.429 | — | — | Kexec: not logged |
| VTL2 settings | 1.444 | 1.211 | ~same | |
| NVMe manager | 1.486 | 1.251 | ~same | |
| PM timer assist (warn) | 1.503 | 1.268 | ~same | |
| **State restore** | 1.530 (869µs) | 1.289 (899µs) | ~same | |
| VM worker started | 1.555 | 1.308 | — | |
| **state_units.start()** | 1.616 (54ms) | 1.371 (57ms) | ~same | |
| **Resuming VM** | **1.625** | **1.377** | **-248ms** | |

---

## 4. Time Budget Summary

| Category | Servicing | Kexec | Savings |
|----------|-----------|-------|---------|
| Pre-restart overhead (save, shutdown) | ~111ms | ~143ms | -32ms (kexec slower: persist write + kexec -e) |
| openhcl_boot bootloader | ~21ms | 0ms | +21ms |
| Kernel boot (0 → /underhill-init) | ~943ms | ~870ms | +73ms |
| Userspace init (init → openvmm_hcl start) | ~66ms | ~66ms | 0ms |
| GET negotiation | ~77ms | ~76ms | 0ms |
| Host state roundtrip | ~17ms | 0ms | +17ms |
| openhcl_boot log replay | ~101ms | 0ms | +101ms |
| VM setup (memory, devices, etc.) | ~258ms | ~221ms | +37ms |
| State restore + start | ~95ms | ~88ms | +7ms |
| **Sum from Linux timestamps** | **~1689ms** | **~1464ms** | **~225ms** |

### Discrepancy Explained

The measured blackout delta is only **~26ms**, not ~225ms. The Linux-clock-based
breakdown shows ~225ms of savings, but kexec has a hidden cost: the **kexec
purgatory phase** (kernel relocation + decompression) happens before the new
kernel's clock starts at 0.000000. This phase is invisible to Linux timestamps
but visible to the Hyper-V reference time.

Evidence: `boot loader times` reports kexec=**131ms** vs servicing=**21ms**
(difference: ~110ms). Combined with the pre-restart overhead difference (+32ms),
this accounts for ~142ms of lost savings, bringing the net delta close to the
observed ~26ms.

---

## 5. Biggest Optimization Targets

### 5.1 Serial Console Gap (~495ms — both paths)
The `printk: legacy console [ttyS2] enabled` → `APIC: Switch to symmetric I/O mode`
gap is ~495ms in both paths. This is the single largest time consumer.

**Potential**: Disable serial console during servicing restart, or defer UART
initialization. Adding `console=` (empty) or removing ttyS2 console for restart
could save ~495ms.

### 5.2 Kernel Boot (~870-943ms — both paths)
The full Linux kernel boot from 0 to `/underhill-init` dominates the blackout
window. Most of this is fixed init cost (memory zones, subsystem init, driver
registration).

**Potential**: Kernel config minimization, initcall parallelization, or skip kernel
reboot entirely (userspace-only restart).

### 5.3 Kexec Purgatory (~110ms — kexec only)
The kexec purgatory (kernel relocation + decompression) adds ~110ms to kexec that
servicing doesn't have. The kernel is already pre-loaded by `kexec_prepare.sh`, but
the bzImage still needs decompression in the blackout window.

**Potential**: Use an uncompressed kernel image (vmlinux) to eliminate decompression
time. Trade-off: larger memory footprint for staging.

### 5.4 openhcl_boot Log Replay (~101ms — servicing only)
Replaying 8 openhcl_boot log lines via `tracing::info!` costs ~101ms in servicing.
This is pure logging overhead during the critical path.

**Potential**: Defer log replay or batch-emit after VM resume.

### 5.5 Serial 8250 Probing (~92ms — both paths)
Probing 3 serial ports (ttyS0, ttyS1, ttyS2) takes ~92ms total.

**Potential**: Reduce to probing only the console port, or configure as built-in
with fixed addresses to avoid auto-detection.
