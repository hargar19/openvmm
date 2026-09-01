## 1. Why vmlinux didn't work

On x86_64, `kexec_file_load` only has one registered format handler: `kexec_bzImage64_ops`. When we passed vmlinux (an ELF), the kernel checked the magic bytes, found it wasn't a bzImage, and returned `ENOEXEC` ("Exec format error"):

```
kexec-bzImage64: Not a bzImage
```

This is a fundamental x86_64 kernel limitation — unlike ARM64 (which has `kexec_image_ops` for raw ELF), x86_64's `kexec_file_load` **only** accepts bzImage format. There's no ELF loader registered for this syscall on x86.

The older `kexec_load` syscall *does* understand ELF (userspace kexec-tools parses it), but it can't relocate segments. vmlinux has `PT_LOAD` at `PhysAddr 0x8000000` which overlaps the running kernel's reserved memory → "Invalid memory segment."

**Result**: vmlinux is a dead end on x86_64 kexec. bzImage is required.

---

## 2. `kexec_load` vs `kexec_file_load` (old vs new syscall)

| | `kexec_load` (old, syscall 246) | `kexec_file_load` (new, syscall 320) |
|---|---|---|
| **Kernel config** | `CONFIG_KEXEC=y` | `CONFIG_KEXEC_FILE=y` |
| **Input** | Pre-parsed memory segments + entry point | File descriptors (kernel fd + initrd fd) |
| **Who parses the image** | **Userspace** (kexec-tools) | **Kernel** |
| **Format support (x86)** | bzImage + ELF (via userspace) | bzImage only (kernel handler) |
| **Signature verification** | Not possible | Supported (`CONFIG_KEXEC_VERIFY_SIG`) |
| **KHO support** | No | Yes (KHO hooks are in this path) |
| **Relocation** | Userspace must set correct addresses | Kernel handles it |
| **kexec-tools flag** | `kexec -l` (default) | `kexec -l -s` |

`kexec_load` is older and more flexible in format support (userspace can parse anything), but it has no kernel-side verification and no KHO path. `kexec_file_load` is the modern path — it's where all new features (KHO, secure boot signing) are built.

---

## 3. Userspace kexec-tools vs direct syscall

| | Userspace `kexec -l` / `kexec -l -s` | Direct `kexec_sys::kexec_file_load()` |
|---|---|---|
| **Binary needed** | kexec (~800KB in rootfs) | None (syscall from Rust) |
| **Validation** | kexec-tools validates ELF segments against iomem **before** syscall | No userspace validation — kernel validates |
| **vmlinux result** | `kexec -l`: "Invalid memory segment" (address overlap check). `kexec -l -s`: same error (userspace check runs first) | Kernel returns `ENOEXEC` (no ELF handler on x86) — different, more honest error |
| **Process overhead** | Fork + exec of kexec binary | Single syscall, no subprocess |
| **For `kexec -e` (reboot)** | Fork + exec `/sbin/kexec -e` → calls `reboot()` | Direct `reboot(LINUX_REBOOT_CMD_KEXEC)` |
| **KHO future** | Would need updated kexec-tools with `--kho` flag | Can pass `KEXEC_FILE_ON_CRASH` or KHO flags directly |
| **Rootfs impact** | Needs kexec-tools binary + its dependencies | Zero — no binary needed |

The key insight from our debugging: even with `-s`, the userspace kexec binary runs its own segment validation *before* making the syscall. By calling the syscall directly, we skip that layer entirely and let the kernel decide what's valid. This also eliminates the kexec binary from the rootfs, saving space and reducing the attack surface.

---

## 4. DTB (Device Tree Blob) in OpenHCL kexec

### What is the DTB?

OpenHCL boots with `acpi=off` — there's no ACPI. Instead, the hypervisor provides a **Flattened Device Tree (FDT)** describing the hardware. This is the paravisor's entire hardware description.

### DTB flow

```
Hypervisor provides Host FDT
        ↓
openhcl_boot parses it (host_fdt_parser)
        ↓
openhcl_boot constructs a new Boot FDT (dt.rs)
        ↓
Linux kernel receives it as /sys/firmware/fdt
        ↓
underhill_core reads it (bootloader_fdt_parser)
```

### What's in the DTB

**From the hypervisor (Host FDT):**

| Node/Property | Purpose |
|---|---|
| `/cpus` — `reg`, `numa-node-id` per CPU | CPU topology (APIC IDs on x86, MPIDR on ARM) |
| `/memory@*` — `reg`, `numa-node-id`, `igvm-type` | Physical memory ranges + NUMA mapping |
| `/vmbus-vtl0@*` — `connection-id`, `ranges` | VTL0 VMBus connection + MMIO windows |
| `/vmbus-vtl2@*` — `connection-id`, `ranges` | VTL2 VMBus connection + MMIO windows |
| `/openhcl/entropy` — `reg` | Up to 256 bytes of host-provided entropy |
| `/openhcl` — `memory-allocation-mode`, `vtl0-alias-map` | Memory allocation strategy, VTL0 alias mapping |
| `/openhcl/device-dma` — `total-pages` | DMA pool size for device passthrough |
| `/openhcl/keep-alive` — `device-types` | NVMe keep-alive hint |
| `/bus/com3` | COM3 serial presence (x86) |
| `/intc@*` (ARM64) | GIC-v3 configuration |
| `/pmu` (ARM64) | PMU interrupt info |

**Transformed by openhcl_boot into Boot FDT with typed memory regions:**

| Memory Type | Purpose |
|---|---|
| `VTL2_RAM` | Normal usable VTL2 memory |
| `VTL2_CONFIG` | Config pages for underhill_core |
| `VTL2_RESERVED` | Reserved range |
| `VTL2_GPA_POOL` | Private pool memory |
| `VTL2_PERSISTED_STATE_HEADER` | 4KB servicing state header |
| `VTL2_PERSISTED_STATE_PROTOBUF` | Serialized partition state |
| `VTL2_PERSISTED_SERVICING_STATE` | Kexec servicing handover state |
| `VTL0_MMIO`, `VTL2_MMIO` | MMIO ranges |

### Why DTB must be preserved across kexec

After kexec, `openhcl_boot` does **not** re-run — the kernel jumps straight to the new kernel. Without the DTB, the new kernel and underhill_core have no way to discover:

| Lost Data | Consequence |
|---|---|
| Memory map + typed regions | Can't find persisted state, can't set up address space |
| VMBUS connection IDs | Can't talk to host or VTL0 devices |
| CPU topology | SMP initialization fails |
| Entropy | No host-provided randomness |
| Isolation type | Wrong security posture (SNP/TDX/VBS) |
| VTL0 alias map | Memory virtualization broken |

### OOT kernel patch for DTB preservation

Mainline x86 Linux does **not** preserve DTB across kexec (x86 uses ACPI, not device trees). We have an OOT patch in `arch/x86/kernel/kexec-bzimage64.c` (commit `647dcc7`) that:

1. Reads `initial_boot_params` (the current DTB) during `kexec_file_load`
2. Copies it into a `SETUP_DTB` entry in the boot params `setup_data` chain
3. The kexec'd kernel receives the same DTB at `/sys/firmware/fdt`

This produces the log line:
```
kexec-bzImage64: kexec: preserving current DTB (size=9148 bytes)
```

### All OOT kernel changes required for kexec servicing

| Commit | File | Purpose |
|---|---|---|
| `9860343` | `Microsoft/hcl-x64.config` | Enable `CONFIG_KEXEC=y` and `CONFIG_KEXEC_FILE=y` |
| `fac33d9` | `Microsoft/build-hcl-kernel.sh` | Add `bzImage` to build targets (was only `vmlinux`) |
| `647dcc7` | `arch/x86/kernel/kexec-bzimage64.c` | Preserve DTB across kexec via `setup_data` chain (61 lines) |

The first two are config/build changes. The third is the only actual kernel code change — without it, kexec works but the new kernel boots with no device tree.

---

## 5. Binary sizes

| Component | Size | Status |
|---|---|---|
| **bzImage** (compressed kernel in rootfs) | **3.5 MB** (3,621,888 bytes) | Included at `/boot/bzImage` |
| **vmlinux** (uncompressed kernel) | ~12 MB | Not used — dead end on x86_64 kexec |
| **kexec binary** (userspace kexec-tools) | ~800 KB | Removed — replaced by direct `kexec_file_load` syscall via `kexec_sys` crate |
