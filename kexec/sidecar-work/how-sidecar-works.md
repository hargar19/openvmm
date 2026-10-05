## Plan: Sidecar From IGVM to VTL2

**Sidecar accelerates VTL2 startup by booting most virtual processors into a minimal kernel instead of Linux.** Those processors can execute the guest immediately; they become Linux CPUs later when needed.

I checked upstream `main` at [commit 9d1e9bf77899](https://github.com/microsoft/openvmm/commit/9d1e9bf7789945dda74894efa43f501fe96f32a0). The links below navigate corresponding workspace sources; local kexec extensions are excluded from this explanation.

**Components**
- **Sidecar kernel:** a freestanding Rust `no_std` ELF, not a Linux module. Runs in VTL2, normally at ring 0.
- **Linux sidecar driver:** manages sidecar CPUs and exposes `/dev/mshv_vtl_sidecarN`.
- **Sidecar client:** an OpenHCL userspace library using that driver.
- **Shared definitions:** boot parameters, command pages, control pages, and CPU state in sidecar_defs.

Both sidecar and OpenHCL Linux run **in VTL2**, on different CPUs. They are not separate VMs or separate trust levels.

**1. Build And Package**
`build_sidecar.rs:60` builds the Rust binary for the `minimal_rt` target, producing an ELF and separate debug symbols. The IGVM recipe supplies that ELF as `ResourceType::UnderhillSidecar`.

`main.rs:1445` opens Linux, initrd, boot shim, and optional sidecar binaries. It calls `load_openhcl_x64` to construct their memory image.

**2. Place Inside IGVM**
`paravisor.rs:341` places Linux first, aligns the next address to **2 MiB**, loads sidecar, then loads `openhcl_boot` and initrd. There is **one shared sidecar image**, not an image per CPU.

`elf.rs:103` imports ELF `PT_LOAD` segments, preserving their physical offsets and accounting for their memory sizes. `file_loader.rs:1144` turns these into `PageData` directives. It does **not** apply sidecar’s dynamic ELF relocations.

The loader records `sidecar_offset`, `sidecar_size`, and `sidecar_entry_offset` relative to the shim base. Relative offsets remain valid if the host relocates the whole VTL2 image.

**3. Host Loads The Image**
The host’s IGVM loader imports those pages into guest physical memory and installs the initial VTL2 BSP context. OpenVMM’s reference implementation is `igvm.rs:138`.

Initial execution targets **the boot shim**, not sidecar: RIP points to the shim entry, RSI identifies its parameter block, and CR3 points to startup page tables. Hyper-V has its own loader implementing this handoff.

Consequently, **the guest does not extract a sidecar ELF from IGVM at boot**. Its executable segments are already in memory; `shim_params.rs:187` reconstructs their actual addresses.

**4. Shim Assigns CPUs**
`sidecar.rs:95` checks eligibility, then partitions contiguous NUMA CPU runs into balanced groups of at most **32 VPs**. Each group’s base VP boots Linux; the remaining VPs normally boot sidecar. A sidecar group is not necessarily an entire NUMA node.

For example, 64 VPs within one NUMA run produce Linux bases 0 and 32, with `boot_cpus=0,32`. Servicing overrides can assign additional CPUs to Linux.

Each group reserves $4096(1+13n)$ bytes for $n$ VPs: one control page, two shared pages per VP, and eleven private pages per VP. Private pages hold page tables, globals, assist/hypercall pages, and a three-page stack. Allocation prefers local NUMA memory, then falls back elsewhere.

**5. Call The Binary**
The shim constructs page-aligned `SidecarParams`: hypercall-page GPA, logging flag, group memory/VP assignments, and optional per-CPU overrides. `SidecarOutput` returns control/shared-page addresses or an error.

It calls the resolved entry address with the C ABI signature `extern "C" fn(&SidecarParams, &mut SidecarOutput) -> bool`. This is a **direct call on the BSP before Linux starts**, not an `exec`, module load, or ioctl.

`entry.S:9` saves the shim’s stack and paging state, clears BSS, applies its own relocations to virtual base `0xffffff8000000000`, fixes page-table pointers to physical addresses, installs its mappings, switches stacks, and calls Rust initialization. It later restores the shim’s state and returns.

**6. Start APs In Parallel**
`init.rs:154` copies the hypercall page because the shim’s original will not remain available. It initializes descriptors, the request interrupt vector `0x20`, VTL-return information, and group control pages.

`init.rs:412` uses atomic per-group counters to claim CPUs. BSP and newly started APs all help start more APs. Each AP receives private page tables, stack, and long-mode context with RIP targeting `ap_init`; startup uses **`HvCallEnableVpVtl` and `HvCallStartVirtualProcessor`, targeting VTL2**.

`vp.rs:58` enables x2APIC, initializes shared pages, publishes `IDLE`, and enters its dispatch loop. The BSP waits for readiness before returning to the shim. This is VTL2 CPU startup, separate from the guest’s later INIT/SIPI startup.

**7. Hand Off To Linux**
The shim preserves sidecar image/runtime memory as **E820 reserved**, so Linux cannot allocate it. It appends `boot_cpus=` and publishes `dt.rs:502` with compatible string `microsoft,openhcl-sidecar` and `ctrl`/`shmem` resources.

`main.rs:745`. Sidecar APs remain running while Linux initializes its selected CPUs.

The [companion Linux driver](https://github.com/microsoft/OHCL-Linux-Kernel/blob/product/hcl-main/6.12/drivers/hv/mshv_vtl_sidecar.c) discovers those nodes, maps control pages, configures response IPIs to each group’s Linux base CPU, and registers its device. **The driver does not boot the sidecar image.**

**8. Dispatch Runtime Commands**
`lib.rs:136` opens devices, uses `INFO` to obtain layout, and maps per-VP command/register pages. Userspace controls command contents; the Linux driver controls CPU status and IPIs.

The protocol supports `NONE`, `RUN_VP`, `GET_VP_REGISTERS`, `SET_VP_REGISTERS`, and `TRANSLATE_GVA`. Async guest execution uses **`START` → completion IPI → `poll`/four-byte CPU-index `read`**. Synchronous commands use `RUN` directly. `STOP` requests cancellation but still requires waiting for completion.

CPU states are `REMOVED`, `IDLE`, `RUN`, `STOP`, and `REMOVE`. Command results, guest context, intercept messages, and errors remain in mapped shared pages.

**9. Execute The Guest**
`vp.rs:256` lazily enables assist/register overlays, restores guest GPR/CR2/FPU state, and calls the hypercall-page VTL-return entry. **Guest code executes on that same VP at a lower VTL**, normally VTL0.

On return to VTL2, an ordinary interrupt allows local resumption unless stopping; an intercept copies its message into the command page and completes the command. OpenHCL userspace handles device emulation, CPUID, MSRs, and intercepted hypercalls. Sidecar itself is not a device-emulation kernel.

**10. Convert To Linux**
By default, `x64.rs:353` triggers Linux conversion to avoid repeated remote-exit jitter. Targeted Linux/device tasks can also trigger it; disabling first-exit hotplug does not prevent every conversion trigger.

`vp.rs:188` stops the remote runner, saves its state, serializes CPU onlining, and restores execution on an affinitized Linux thread. The kernel’s CPU-hotplug hook sends `REMOVE` and waits for `REMOVED`; sidecar disables its overlays/APIC operation, acknowledges, and parks. **Conversion is one-way within that boot; `STOP` alone does not remove sidecar.**

**Policy And Verification**
Sidecar requires x64, runtime non-isolation, x2APIC, an included image, useful CPU groups, and sufficient memory. `OPENHCL_SIDECAR=on/off/log` controls it. Upstream’s [release manifest](https://github.com/microsoft/openvmm/blob/9d1e9bf7789945dda74894efa43f501fe96f32a0/vm/loader/manifests/openhcl-x64-release.json) does **not** disable it; your local manifest does.

Servicing selectively boots CPUs with restored NVMe I/O/interrupt requirements into Linux. The **400-CPU bound applies to optional override storage**, not sidecar’s overall CPU capacity; the declared 100-CPU threshold is currently unused.

Research and the detailed source map are saved in the session plan. No code changes or live VM tests were performed; the companion driver was checked on its public 6.12 branch, not against a pinned deployed kernel package.
