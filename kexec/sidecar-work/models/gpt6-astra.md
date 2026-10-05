Yes. The idea is to **make OpenVMM construct an ordinary ELF that boots through a small bootstrap**, instead of teaching the Linux kernel about a Microsoft-specific bundle.

**Proposed Flow**
```text
Incoming IGVM
    │
    ├─ Linux ELF
    ├─ Sidecar ELF
    ├─ OpenHCL bootstrap
    └─ Initrd
           │
           ▼
OpenVMM builds one composite ELF
           │
           ▼
kexec_file_load(composite_elf_memfd, initrd_memfd)
           │
           ▼
Existing kernel ELF loader
           │
           ▼
Bootstrap → initialize sidecar → enter Linux
           │
           ▼
OpenHCL restores saved servicing state
```

**Implementation Steps**
1. **Keep the extraction work already implemented.** Extract Linux, sidecar, and bootstrap from the incoming IGVM. This keeps them matched to the incoming build and requires no rootfs addition.

2. **Replace the custom bundle with a standard ELF.** In `kexec_prepare.rs`, arrange the components into compatible `PT_LOAD` regions, including space for BSS, stack, handoff data, and sidecar runtime memory. Preserve each component’s internal offsets and satisfy the existing loader’s alignment requirements.

3. **Set the ELF entry point to a small position-independent stub.** The kernel loads the ELF and jumps to that entry using its existing convention. It does not know that some segments contain sidecar.

4. **Build the handoff inside the stub.** The stub receives Linux’s boot-parameter pointer through the existing `RSI` argument. Using its actual address and embedded relative offsets, it finds the relocated components and constructs the bootstrap’s handoff. This replaces the handoff currently constructed by my kernel patch.

5. **Reuse servicing’s sidecar initialization in the boot shim.** Read the final persisted topology and NVMe CPU exclusions, choose Linux versus sidecar CPUs, allocate sidecar memory, and start sidecar. Generate matching `boot_cpus=`, DT nodes, and memory reservations while preserving the saved-state and live DMA regions.

6. **Enter Linux and restore normally.** Preserve `OPENHCL_KEXEC_SERVICING=1` and the persisted state. Existing userspace restoration resumes VTL0 instead of reloading its firmware.

**What This Removes**
The kernel would no longer need the new bundle parser, companion-specific loading functions, or special bootstrap handoff construction. Those responsibilities move into OpenVMM and the boot shim.

**What It Does Not Solve**
- Old CPUs must still stop using their previous code, stacks, and control pages before those pages are reused.
- Fresh sidecar CPU assignment and safe memory retirement still need implementation and live testing.
- Signature enforcement must authenticate the complete composite image. Generating and signing it at build time may be preferable to assembling it dynamically.

**Current Status**
This remains a proposed alternative, not implemented. The current code still uses the custom bundle and kernel patches.

The narrower promise is **no new sidecar-specific kernel loader changes**. It still depends on this branch’s existing ELF/DTB/VTL2 kexec support; it does not imply compatibility with an unmodified upstream kernel.


/////////////////////////////////////////////
**My recommendation: a fresh sidecar from the incoming IGVM, started by a minimal bootstrap carried in a standard composite ELF.** Prepare everything possible before stopping VTL0, and keep Linux’s initial CPU set small.

This fits your servicing-equivalence goal while avoiding a sidecar-specific kernel loading ABI. **The composite format itself is not a proven performance improvement**; the expected benefit comes from avoiding unnecessary Linux CPU and per-CPU initialization.

**Implementation Plan**
1. **Prove the bootstrap preserves servicing first.** Require the servicing marker, restored saved state, unchanged VTL0 uptime, and a valid blackout event. The latest fresh-boot log does not pass this gate.

2. **Finish all static preparation before blackout.** Extract and validate incoming components, compose the ELF, allocate staging memory, and call `kexec_file_load` while VTL0 runs. The `mod.rs:703` already stages before stopping the VM. Prefer build-time composition/signing for production.

3. **Reuse the existing stop/save and NVMe keepalive path.** Read the final saved CPU exclusions after save; don’t use a potentially stale staging-time snapshot. Keep DMA and saved-state memory protected.

4. **Perform a minimal pre-Linux handoff.** Safely retire old CPU execution without first bringing every sidecar CPU into Linux. Start the fresh sidecar using the existing fan-out mechanism, then produce matching CPU masks, DT nodes, and memory reservations. Do not replay cold-boot initialization.

5. **Boot Linux only where needed.** Start sidecar-node base CPUs plus CPUs required for restored device interrupts. With the current 32-VP node policy, 64 VPs on one NUMA node would normally need **two Linux base CPUs**, before exclusions. Ensure VM/threadpool initialization does not immediately online the other 62.

6. **Restore and resume using existing servicing code.** Defer optional initialization and CPU hotplug until needed, but never defer work required for correct device operation.

**Why Not Retain The Old Sidecar?**
That could have lower transition overhead when most CPUs still run in sidecar. But CPUs already hotplugged into Linux stay Linux, and the sidecar cannot be upgraded. Its benefit becomes workload-dependent. I would consider it later as a compatible fast path, not the default design.

**Measure The Right Savings**
Your six sidecar-off kexec samples averaged approximately **436 ms**, ranging from **368–531 ms**. Direct SMP startup was only about **10–18 ms**. Therefore, removing AP startup alone will not produce a dramatic reduction.

Measure:
- `handoff_kernel_init`, `vm_build`, restore, and total blackout.
- Linux CPU count at guest resume.
- Median, p95, and maximum across matched runs.
- First-I/O latency after resume, to detect costs merely deferred beyond the blackout measurement.

**Next concrete milestone:** demonstrate reliable bootstrap-only servicing through the composite ELF, then enable fresh sidecar on a small VM before scaling to 64 VPs and active NVMe. CPU retirement remains a correctness gate; avoiding a custom loader ABI does not eliminate that work.
