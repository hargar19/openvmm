# Plan: Kexec Servicing with Rust Stub + Transient IGVM Mapping

## Problem

bzImage (~3.5 MB) is currently packaged in the rootfs at `/boot/bzImage`. The rootfs is the initramfs loaded into VTL2 memory. Adding bzImage to rootfs increases VTL2 memory requirements. We want to avoid that.

## Current IGVM Memory Layout (x86_64)

From [paravisor.rs](../../vm/loader/src/paravisor.rs):

```
--- Low memory, 2MB aligned ---
persisted state region (2MB)
bounce buffer (2MB, CVM only)
kernel (vmlinux ELF, ~12MB)
sidecar (optional)
openhcl_boot (boot shim)
initrd (rootfs)          ← bzImage currently lives here, inside the CPIO
GDT (1 page)
boot params (1 page)
command line (1 page)
reserved VTL2 region
IGVM parameters (SLIT, PPTT, DTB)
bootshim logs (8KB)
bootshim heap (16 pages)
page tables
free space
--- High memory, 2MB aligned ---
```

Each component is placed sequentially, with offsets recorded in `ShimParamsRaw` so `openhcl_boot` can find them.

## Chosen Approach: Rust Stub + Transient IGVM Mapping

Instead of permanently embedding the kexec kernel in VTL2 GPA space, use a two-stage kexec with a small Rust trampoline stub. The host maps the **new** IGVM file into GPA space only during servicing — the kernel data is transient, not permanently resident.

### How It Works

1. **Steady state**: Only the stub (~64 KB) lives in VTL2 memory. No kernel copy is resident.
2. **Servicing starts**: The host maps the **new** IGVM file into VTL2 GPA space (transient mapping).
3. **kexec stage 1**: `underhill_core` calls `kexec_file_load` with the **stub** (not the real kernel).
4. **kexec stage 2**: The stub runs bare-metal, reads vmlinux + initrd from the mapped IGVM region, sets up boot params, and jumps to the kernel entry point.
5. **After boot**: The transient IGVM mapping is released — memory reclaimed.

### Key Benefits

| Benefit | Explanation |
|---------|-------------|
| **Minimal memory overhead** | Only ~64 KB stub is permanently resident, vs ~3.5 MB for bzImage or ~12 MB for vmlinux |
| **Uncompressed kernel support** | vmlinux (~12 MB) can be used directly since it's transient — no need for bzImage compression to save space |
| **Version-correct servicing** | The host maps the **new** IGVM file, so we always boot the new kernel — no stale data from VM creation time |
| **Smaller rootfs** | bzImage removed from rootfs CPIO entirely |

---

## Design Details

### The Stub

The stub is a small bare-metal Rust program (~64 KB) that:

1. Receives control after kexec (bare machine — no OS, no allocator)
2. Finds the transient IGVM mapping at a known GPA range (communicated via boot params or a fixed address)
3. Parses the IGVM to locate vmlinux and initrd sections (or the host pre-parses and maps components at known offsets)
4. Sets up Linux boot_params: command line, initrd pointer, memory map
5. Builds page tables for the kernel
6. Handles CVM page acceptance if needed (SNP/TDX)
7. Jumps to the vmlinux 64-bit entry point

The stub is essentially a minimal bootloader, similar in spirit to `openhcl_boot` but focused solely on the kexec-to-new-kernel transition.

### Transient IGVM Mapping

The host maps the new IGVM file into VTL2 GPA space at servicing time:

- **Non-CVM**: Straightforward — the host maps pages directly into the GPA range.
- **CVM (SNP/TDX)**: The host cannot inject data without guest cooperation. The stub must accept pages from the host mapping. This may require a protocol (e.g., the host signals the mapping location via a register or shared page, and the stub issues acceptance hypercalls).

### Discovery: How Does the Stub Find the IGVM Data?

Options:

| Option | How | Pro | Con |
|--------|-----|-----|-----|
| **A: Fixed well-known GPA** | Host always maps IGVM at a predetermined address | Simple — no discovery needed | Inflexible, may conflict with existing layout |
| **B: Boot params / cmdline** | `underhill_core` passes the GPA range when setting up kexec | Flexible | Stub must parse boot params |
| **C: Shared register / page** | Host writes mapping location to a register or shared memory page before servicing kexec | Decoupled from Linux boot protocol | Needs host-side coordination |

### What Format Does the Stub Parse?

Options:

| Option | How | Pro | Con |
|--------|-----|-----|-----|
| **A: Raw IGVM** | Stub contains an IGVM parser, finds kernel/initrd sections | Self-contained — host just maps the file | Stub needs IGVM parsing logic |
| **B: Pre-parsed by host** | Host maps vmlinux at offset X, initrd at offset Y, writes a simple header | Stub is minimal — just reads a header | More host-side logic, tighter coupling |

---

## Open Questions (with findings)

> **Scope**: This experiment targets x86 non-CVM only. CVM (SNP/TDX) support is out of scope.

### 1. Host-side IGVM mapping mechanism

**Status: Does not exist yet — needs to be built.**

The closest existing mechanisms:

- **`CREATE_RAM_GPA_RANGE`** (GET protocol message ID 28, defined in `vm/devices/get/get_protocol/src/lib.rs`): VTL2 asks the host to map RAM at a GPA range. Used only by i440bx PCI bridge for BIOS PAM emulation. The OpenVMM GED handler **always returns `FAILED`** — only works with real Hyper-V host.
- **`complete_reload_igvm()`** in `openvmm/openvmm_core/src/worker/dispatch.rs`: The host-driven reload path. The host parses the IGVM, stops VPs, resets VTL2 VMBus, calls `load_firmware(true)` to re-import all IGVM pages into VTL2 GPA space, then resumes VPs. This is the standard servicing path but it's **host-driven** — VTL2 is not running during the reload.
- **Hypercalls**: `HvCallModifySparseGpaPageHostVisibility` (0x00DB) and `HvCallAcceptGpaPages` (0x00D9) exist but are for CVM page visibility, not data injection.

**Conclusion**: A new mechanism is needed for the host to map the new IGVM's contents into VTL2 GPA space while VTL2 is still running (before kexec). Options:
- Extend `CREATE_RAM_GPA_RANGE` to support IGVM-backed mappings
- Add a new GET protocol message
- Use a shared memory region where the host places the data
- Have the host pre-parse the IGVM and map individual components (kernel, initrd) at well-known GPAs

### 2. Stub delivery

**Answer: Embed in the IGVM at creation time, like openhcl_boot.**

`openhcl_boot` is already embedded as an ELF loaded into VTL2 GPA space via `crate::elf::load_static_elf()` in `vm/loader/src/paravisor.rs`. Its offset/size is recorded in `ShimParamsRaw`. The stub would follow the exact same pattern — a small ELF embedded in the IGVM layout.

The stub is version-locked to VM creation time, but this is acceptable: the stub's ABI (how it discovers the IGVM mapping and boots a vmlinux) is stable. The actual kernel and initrd come from the **new** IGVM at servicing time.

Resources are specified via `ResourceType` in `vm/loader/igvmfilegen_config/src/lib.rs` and wired through manifest JSON files in `vm/loader/manifests/`.

### 3. IGVM parsing in stub vs. host pre-parsing

**Recommendation: Host pre-parses (Option B).**

The IGVM parsing crate (`igvm` v0.4.0) is available but pulls in dependencies unsuitable for a bare-metal stub. `openhcl_boot` does **not** parse IGVM — it reads parameters from the device tree (FDT). The host/hypervisor processes the IGVM and places data at the right GPAs before the shim runs.

For the stub approach, **host pre-parsing is simpler**: the host already parses the IGVM in `complete_reload_igvm()` → `load_firmware()`. It can extract vmlinux and initrd and map them at known GPA offsets, then communicate the layout to VTL2 via a simple header or shared page. The stub just reads "kernel at GPA X (size N), initrd at GPA Y (size M)" and boots.

### 4. Initrd handling

**Answer: The initrd must come from the new IGVM.**

Currently, `prepare_kexec()` in `openhcl/underhill_core/src/dispatch/kexec_prepare.rs` builds a **fresh initramfs** at servicing time by:
1. Reading the running `openvmm_hcl` binary from `/usr/bin/openvmm_hcl`
2. Reading kernel modules from `/boot/modules/` (`pci-hyperv-intf.ko`, `pci-hyperv.ko`, `hv_storvsc.ko`)
3. Building a cpio newc archive in memory, compressing with gzip (flate2, level 1), writing to `/tmp/initramfs.gz`
4. Passing the fd to `kexec_file_load()`

For version-correct servicing, the **new** initrd (or at least the new `openvmm_hcl` binary) must come from the new IGVM. The host should map both the new vmlinux **and** the new initrd from the IGVM into VTL2 GPA space. The stub then passes both to the kernel at boot.

### 5. Fallback

**Answer: Robust fallback already exists — kexec failure falls through to host-driven servicing.**

The current kexec flow has layered fallbacks at every step:

- **`prepare_kexec_if_enabled()`** (`openhcl/underhill_core/src/dispatch/mod.rs`): On any error, logs `"kexec prepare failed; will fall back to host restart after save"` and returns false. Servicing continues via the host path.
- **`try_kexec_after_servicing()`**: If persisted state write fails, or kexec exec fails, or `kexec_reboot()` returns (meaning it failed), the code falls through to `send_servicing_state()` which sends state to the host for standard host-driven reload.
- **Restore side** (`openhcl/underhill_core/src/worker.rs`): On kexec boot, first tries persisted state from memory, then retries after GET completes, then falls back to `get_saved_state_from_host()`.

**Key gap**: Once `reboot(LINUX_REBOOT_CMD_KEXEC)` succeeds, there's no going back — the old kernel is gone. If the stub or new kernel fails to boot, the VM is dead. A host-side health-check/watchdog would be needed for production.

---

## Risks

| Risk | Mitigation |
|------|-----------|
| Stub is a bare-metal bootloader — significant complexity | Keep it minimal; reuse code from `openhcl_boot` where possible |
| CVM page acceptance adds complexity to the stub | Start with non-CVM support; add CVM as a follow-up |
| Double kexec adds latency (kexec → stub → kernel) | Stub should be fast (~milliseconds) since it's just copying memory and jumping |
| Stub must correctly implement the Linux x86 boot protocol | Leverage existing `openhcl_boot` code; test thoroughly |
| Host-side IGVM mapping mechanism may not exist yet | Coordinate with host team on the mapping API |
| Stub in IGVM is version-locked to creation time | Stub ABI must be stable; kernel format (vmlinux ELF) is stable |

---

## Implementation Order

1. **Design stub ABI** — Define how the stub receives control (entry point, register state, stack) and how it discovers the IGVM mapping location
2. **Implement minimal stub** — Bare-metal Rust binary that can boot a vmlinux from a known GPA (hardcoded for testing)
3. **Host-side IGVM mapping** — Implement or wire up the mechanism for the host to map the new IGVM into VTL2 GPA space at servicing time
4. **Stub IGVM/component parsing** — Stub locates vmlinux + initrd from the mapped region (either IGVM parsing or reading a header)
5. **Embed stub in IGVM** — Add the stub as a new region in the IGVM layout, with offset/size in `ShimParamsRaw`
6. **Wire kexec to use stub** — `underhill_core` kexecs into the stub instead of bzImage
7. **Remove bzImage from rootfs** — [openhcl/rootfs.config](../../openhcl/rootfs.config)
8. **CVM support** — Add page acceptance to the stub for SNP/TDX
9. **Build pipeline integration** — Wire stub build into flowey pipeline
10. **Test**: End-to-end kexec servicing with stub + transient IGVM mapping

---

## Rejected Alternative: bzImage as Permanent IGVM PageData + DTB Discovery

An earlier approach was prototyped and removed: embed bzImage as PageData in the IGVM file at creation time, expose its location via a new `VTL2_KEXEC_KERNEL` memory type in the DTB, and have `underhill_core` mmap it via `/dev/mem` and pass it to `kexec_file_load`. The DTB changes (VTL2_KEXEC_KERNEL memory type, open_kexec_kernel in kexec_prepare.rs, ShimParams fields, etc.) have been reverted.

This was rejected because:

- **Permanent memory cost**: bzImage (~3.5 MB) would be permanently resident in VTL2 GPA space, even when not servicing.
- **Version-locked**: The bzImage baked into the IGVM at creation time is the **old** kernel. Servicing to a new version would require a separate mechanism to provide the new kernel.
- **Forces bzImage format**: To keep memory overhead down, we'd need the compressed bzImage (~3.5 MB) instead of the uncompressed vmlinux (~12 MB). The transient mapping approach allows using vmlinux directly since the memory is only used during the servicing transition.

The stub + transient IGVM mapping approach solves all three issues: minimal permanent memory overhead (~64 KB stub only), always loads the new version's kernel, and supports uncompressed vmlinux.
