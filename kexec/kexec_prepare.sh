#!/bin/sh
# Pre-build the initramfs and stage the kernel for kexec.
# This script should be run in the background after Underhill is fully
# operational.  It performs the slow work (file copies, cpio archive,
# gzip compression, kexec --load) once so that the servicing path only
# needs to call `kexec -e`.
#
# A sentinel file /run/kexec-ready is created on success so the servicing
# code can check whether pre-loading completed.
set -eu

READY_FLAG="/run/kexec-ready"
rm -f "$READY_FLAG"

KEXEC_BIN="/sbin/kexec"
KERNEL_IMAGE="/boot/bzImage"
TARGET=$(readlink -f /underhill-init 2>/dev/null || echo /usr/bin/openvmm_hcl)

# Build kernel command line early — pure string work, no I/O.
#
# After kexec, openhcl_boot does NOT run (we load bzImage directly), so the
# command line from /proc/cmdline is what the kexec'd kernel receives.  Most
# parameters carry over correctly, but some need adjustment:
#
#   boot_cpus=  — MUST be stripped.  On the original boot sidecar was active
#                 and openhcl_boot set boot_cpus=0 to limit Linux to one CPU
#                 while sidecar managed the rest.  After kexec there is no
#                 sidecar, so the kernel must SMP-boot all CPUs.
CMDLINE=$(sed 's/boot_cpus=[^ ]* *//' /proc/cmdline)
case "$CMDLINE" in
    *OPENHCL_KEXEC_SERVICING=*) ;;
    *) CMDLINE="$CMDLINE OPENHCL_KEXEC_SERVICING=1" ;;
esac
[ -n "${EXTRA_CMDLINE:-}" ] && CMDLINE="$CMDLINE $EXTRA_CMDLINE"

# --- Build initramfs tree ------------------------------------------------
echo "[KEXEC-PREPARE] Building minimal initramfs"
INITRD_DIR="/dev/shm/initrd"
rm -rf "$INITRD_DIR"
mkdir -p "$INITRD_DIR/bin" \
    "$INITRD_DIR/dev" \
    "$INITRD_DIR/etc" \
    "$INITRD_DIR/proc" \
    "$INITRD_DIR/run" \
    "$INITRD_DIR/sys" \
    "$INITRD_DIR/tmp" \
    "$INITRD_DIR/lib/modules/000" \
    "$INITRD_DIR/lib/modules/001" \
    "$INITRD_DIR/lib/modules/999"

# Pre-mount device nodes — underhill_init's init_logging() opens these
# BEFORE devtmpfs is mounted on /dev.  After the mount, the kernel
# auto-populates all other device nodes.
mknod "$INITRD_DIR/dev/null" c 1 3
mknod "$INITRD_DIR/dev/kmsg" c 1 11
mknod "$INITRD_DIR/dev/ttyprintk" c 5 3 || true
[ -e "$INITRD_DIR/dev/ttyprintk" ] || ln -sf kmsg "$INITRD_DIR/dev/ttyprintk"
ln -sf ttyprintk "$INITRD_DIR/dev/console"
chmod 666 "$INITRD_DIR/dev/null" "$INITRD_DIR/dev/kmsg"

# Underhill binary — must be copied (data has to go into cpio).
# /underhill-init is a symlink so the cpio stores it once, matching rootfs.config.
cp "$TARGET" "$INITRD_DIR/bin/openvmm_hcl"
chmod 755 "$INITRD_DIR/bin/openvmm_hcl"
ln -s /bin/openvmm_hcl "$INITRD_DIR/underhill-init"

# Kernel modules — copy into numbered subdirectories to preserve load
# order (underhill_init walks /lib/modules/ sorted by name).
# 000 = pci-hyperv-intf (infrastructure, must load first)
# 001 = pci-hyperv       (depends on intf)
# 999 = hv_storvsc       (storage, slow to probe, loads last)
cp_module() {
    src="/boot/modules/$2"
    if [ -f "$src" ]; then
        cp "$src" "$INITRD_DIR/lib/modules/$1/$2"
    else
        echo "[KEXEC-PREPARE] WARN missing $src" >&2
    fi
}
cp_module 000 pci-hyperv-intf.ko
cp_module 001 pci-hyperv.ko
cp_module 999 hv_storvsc.ko

# --- Build compressed initramfs ------------------------------------------
# Use gzip -1 (fast) to keep the initrd small — the kernel needs memory for
# both the compressed archive and the extracted rootfs during early boot.
IMG_PATH="/tmp/initramfs.gz"
rm -f "$IMG_PATH"
( cd "$INITRD_DIR" && find . -print0 | cpio --null -o -H newc 2>/dev/null | gzip -1 ) > "$IMG_PATH"
rm -rf "$INITRD_DIR"

# --- Load the kernel into kexec staging memory ---------------------------
# After this call, only `kexec -e` is needed to jump to the new kernel.
echo "[KEXEC-PREPARE] Loading kernel with kexec -l"
$KEXEC_BIN -l "$KERNEL_IMAGE" --initrd="$IMG_PATH" --command-line="$CMDLINE" --reset-vga

# Clean up — the kernel image is now staged in kernel memory by the kexec
# subsystem, so the temp file is no longer needed.
rm -f "$IMG_PATH"

# Signal that kexec is pre-loaded and ready.
touch "$READY_FLAG"
echo "[KEXEC-PREPARE] kexec pre-loaded successfully"
