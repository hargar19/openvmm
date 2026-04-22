# Plan: Try vmlinux (uncompressed kernel) with kexec

## Goal
Eliminate the ~110ms purgatory decompression cost during the kexec blackout window by using an uncompressed `vmlinux` ELF instead of `bzImage`.

## Prerequisites
- Confirm `vmlinux` exists in the kernel package (it does — at `build/native/bin/x64/vmlinux`)
- Confirm `kexec -l` accepts ELF vmlinux (it does — kexec-tools auto-detects format)

## Changes Required

### 1. Add vmlinux to the rootfs — `openhcl/rootfs.config` (line 61)

Change the kernel image line from bzImage to vmlinux:
```
# Current:
file /boot/bzImage ${OPENHCL_MODULES_PATH}/../../build/linux/arch/x86/boot/bzImage 0644 0 0

# New:
file /boot/vmlinux ${OPENHCL_KERNEL_PATH}/build/native/bin/${OPENHCL_KERNEL_ARCH}/vmlinux 0644 0 0
```
Note: the vmlinux path uses a different directory than bzImage. It lives at `build/native/bin/x64/vmlinux`, not under `build/linux/arch/x86/boot/`.

### 2. Update the Rust kexec prepare code — `openhcl/underhill_core/src/dispatch/kexec_prepare.rs` (line 26)

```rust
// Current:
const KERNEL_IMAGE: &str = "/boot/bzImage";

// New:
const KERNEL_IMAGE: &str = "/boot/vmlinux";
```

### 3. Update the shell scripts (if still used for testing)
- `kexec/kexec_test.sh` (line 66): Change `KERNEL_IMAGE="/boot/bzImage"` to `"/boot/vmlinux"`
- `kexec/kexec_prepare.sh`: Same change if it has a hardcoded path

### 4. No changes needed to kexec flags
`kexec -l` auto-detects ELF vs bzImage. The `--reset-vga` flag is still valid for both formats.

## Risks and Measurements

| Risk | Mitigation |
|------|-----------|
| Rootfs size increase (~30-70MB larger) | Measure final IGVM size; may need `strip` on vmlinux or accept the trade-off |
| kexec staging memory increase | Measure with `dmesg | grep kexec` — segments will be larger but no decompression buffer needed |
| Boot behavior differences | ELF entry point differs from bzImage startup_64; verify kernel boots identically |
| `--reset-vga` compatibility | May not apply to ELF path in kexec-tools; test and remove if warnings appear |

## How to Measure Success

1. **Baseline**: Run current bzImage kexec path and capture Hyper-V reference time delta across the blackout window (should show ~110ms purgatory cost)
2. **After change**: Same measurement — purgatory cost should drop to near zero (just segment relocation, no decompression)
3. **Compare**:
   - Blackout time reduction (target: ~110ms savings)
   - `kexec -l` staging time (may change slightly — ELF parsing vs bzImage parsing)
   - Total rootfs/IGVM size delta
   - Total memory footprint of staged kexec segments

## Quick Smoke Test (on a running OpenHCL VM)

If you have shell access to VTL2 and both files available:
```bash
# Check if vmlinux exists in the kernel package
ls -lh /boot/vmlinux   # (after adding to rootfs)
file /boot/vmlinux      # Should show: ELF 64-bit LSB executable

# Stage with vmlinux instead
kexec -l /boot/vmlinux --initrd=/tmp/initramfs.gz --command-line="$(cat /proc/cmdline)" --reset-vga

# If staging succeeds, execute
kexec -e
```

## Optional Follow-up: Strip vmlinux

If the size increase is unacceptable, you can strip debug symbols:
```bash
# In the kernel build or rootfs packaging step
strip --strip-debug vmlinux   # Typically reduces from ~70MB to ~30MB
```
Or add `CONFIG_DEBUG_INFO=n` in the kernel config (if you control the OHCL-Linux-Kernel build).
