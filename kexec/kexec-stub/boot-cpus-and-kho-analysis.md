# boot_cpus=0 vs All-CPUs Boot & KHO VP Path Analysis

## Flow Comparison: `boot_cpus=0` vs All-CPUs Boot

### With `boot_cpus=0` (old bzImage kexec path)

```
Timeline (32 VP VM):

 kexec_core: Starting new kernel             [t=0]
 │
 ├── Kernel boot (1 CPU only)
 │   ├── smp: Bringing up secondary CPUs...  [+0.63s]
 │   ├── smp: Brought up 1 node, 1 CPU      [+0.63s]  ← instant, no APs started
 │   ├── Driver init, initramfs unpack       [+0.97s]
 │   ├── /underhill-init starts              [+0.97s]
 │   └── mshv_vtl_init: cpuhp brings up     [+1.0s]
 │       31 secondary VPs (via HVCALL_START_VP, sequential)
 │                                            [~1.3s total to bring up all 32]
 │
 ├── Underhill init + GET negotiation        [+1.2s]
 ├── NVMe VFIO + device restore             [+2.9s]
 ├── Partition restore (VP registers)        [+2.97s → 4.40s]  ← 1.43s!
 ├── State start (vmtime, devices)           [+4.40s → 4.83s]
 └── VM resumes                              [+5.8s?]
     (old logs cut off, but ~5.8s based on boot_time)
```

### Without `boot_cpus=0` (stub kexec path — current)

```
Timeline (32 VP VM):

 kexec_core: Starting new kernel             [t=0]
 │
 ├── Kernel boot (all 32 CPUs)
 │   ├── smp: Bringing up secondary CPUs...  [+0.63s]
 │   ├── 31 APs started via HVCALL_START_VP  [+0.63s → 0.65s]  ← 11ms!
 │   ├── smp: Brought up 1 node, 32 CPUs    [+0.65s]
 │   ├── Driver init, initramfs unpack       [+0.97s]
 │   └── /underhill-init starts              [+0.97s]
 │       (mshv_vtl_init: APs already up, just configure SynIC)
 │
 ├── Underhill init + GET negotiation        [+1.16s]
 ├── NVMe VFIO + device restore             [+1.85s → 2.86s]
 ├── Partition restore (VP registers)        [+2.97s → 4.40s]  ← 1.43s!
 ├── State start (vmtime, devices)           [+4.43s → 4.90s]
 └── VM resumes                              [+4.91s]
     blackout_time = 6.56s
```

## Does Removing `boot_cpus=0` Save Blackout Time?

**No meaningful difference for total blackout time.** Here's why:

The SMP bringup is ~11ms for all 32 CPUs (parallel HVCALL_START_VP). With `boot_cpus=0`, those same VPs get started later by `mshv_vtl_init` anyway, taking similar time. Moving it earlier doesn't save wall-clock time because the **bottleneck is elsewhere**.

## The Real Bottleneck: Partition Restore (VP Registers) — 1.43s

From the stub-kexec logs:
```
device="partition" restore duration = 1.42633002s   (22% of total blackout)
```

This is the `partition` state unit restoring all 32 VPs' register sets. In the kexec path, VP registers are persisted in the VTL2 memory region and replayed via **sequential `set_vp_registers` ioctls** — one per VP, ~33 register groups each. That's 32 × 33 = ~1056 kernel ioctls sequentially.

**This 1.43s is present regardless of `boot_cpus=0` or not.** It happens after all VPs are up, during Underhill's device restore phase. The APs being up earlier doesn't help because the VP register restore is single-threaded from the Underhill dispatcher.

## Blackout Breakdown (6.56s total, stub path)

| Phase | Time | % |
|---|---|---|
| VM stop (pre-kexec, old kernel) | ~965ms | 15% |
| kexec + stub + kernel boot | ~975ms | 15% |
| Underhill init → NVMe restore | ~1.9s | 29% |
| **Partition restore (VP registers)** | **1.43s** | **22%** |
| State start (vmtime + devices) | ~475ms | 7% |
| Misc | ~0.8s | 12% |

## What WOULD Save Blackout Time

1. **Sidecar for VP register restore** (~32x speedup → ~45ms instead of 1.4s) — but sidecar is disabled during kexec servicing because it requires OOT kernel modules that aren't loaded yet
2. **Batch `set_vp_registers` ioctl** — restore multiple VPs in one syscall instead of sequential per-VP calls
3. **Parallel VP restore** — spread the 32 VP register restores across multiple threads (currently single-threaded)
4. **KHO (Kexec Handover)** — preserve VP state across kexec without save/restore entirely

---

## KHO: Different VP Bringup Path

### Without KHO (current)

```
Old kernel stops APs → kexec → new kernel calls HVCALL_START_VP for each AP
→ VP starts FRESH (registers reset) → Underhill must replay 1.43s of set_vp_registers ioctls
```

### With KHO

```
Old kernel marks VP memory (register pages, VP assist pages, SynIC pages) as preserved
→ kexec with handover → new kernel sees VPs still exist with valid state
→ SMP bringup reattaches to existing VPs (no HVCALL_START_VP needed)
→ mshv_vtl reattaches to preserved SynIC state
→ Underhill skips VP register restore entirely (state never lost)
```

### Key Differences

1. **VPs aren't destroyed/recreated** — the hypervisor never tore them down, and their backing memory is preserved across kexec, so the new kernel just resumes them
2. **No stale register page problem** — register pages are in preserved KHO memory, so they're still valid when APs wake up (our kernel fix becomes unnecessary)
3. **Eliminates the 1.43s partition restore** — VP registers were never lost, no need to replay them via ioctls
4. **SynIC state persists** — no need to re-configure SINT0/interrupt routing

### Impact on Blackout Time

KHO doesn't just change how VPs are brought up — it eliminates the biggest single bottleneck (partition restore) by keeping VP state alive across the transition:

- Partition restore: 1.43s → ~0ms
- Stale state cleanup: not needed (register pages preserved)
- SynIC reconfiguration: not needed (state preserved)

**Estimated blackout reduction: 6.56s → ~5.1s** from KHO alone (eliminating the 1.43s partition restore).

Combined with other improvements (sidecar for remaining device restore, NVMe fast-path), KHO is the foundation for getting blackout under 2s.
