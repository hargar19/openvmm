#!/bin/sh
# Minimal reproducible kexec test for Underhill / VTL2 flow.
# Only what is needed to reproduce the original VTL2 protection (AccessDenied)
# issue prior to the init.rs idempotent fix.
set -eu

echo "[KEXEC] Building minimal initramfs"
INITRD_DIR="/dev/shm/initrd"
rm -rf "$INITRD_DIR"
mkdir -p "$INITRD_DIR" \
    "$INITRD_DIR/bin" \
    "$INITRD_DIR/dev" \
    "$INITRD_DIR/etc" \
    "$INITRD_DIR/proc" \
    "$INITRD_DIR/run" \
    "$INITRD_DIR/sys" \
    "$INITRD_DIR/tmp"

# Busybox + init
cp /bin/busybox "$INITRD_DIR/bin/busybox"; chmod 755 "$INITRD_DIR/bin/busybox"
for l in sh mount mkdir echo mknod dd od hexdump printf; do ln -sf busybox "$INITRD_DIR/bin/$l"; done
cp /bin/busybox "$INITRD_DIR/init" && chmod 755 "$INITRD_DIR/init"

# Devices
mknod "$INITRD_DIR/dev/console" c 5 1
mknod "$INITRD_DIR/dev/ttyS2" c 4 66
mknod "$INITRD_DIR/dev/null" c 1 3
mknod "$INITRD_DIR/dev/kmsg" c 1 11
mknod "$INITRD_DIR/dev/mem" c 1 1
mknod "$INITRD_DIR/dev/ttyprintk" c 5 3 || true
chmod 600 "$INITRD_DIR/dev/console" "$INITRD_DIR/dev/ttyS2" "$INITRD_DIR/dev/mem"
chmod 666 "$INITRD_DIR/dev/null" "$INITRD_DIR/dev/kmsg"
[ -e "$INITRD_DIR/dev/ttyprintk" ] || ln -sf kmsg "$INITRD_DIR/dev/ttyprintk"

# Underhill binary
TARGET=$(readlink -f /underhill-init 2>/dev/null || echo /usr/bin/openvmm_hcl)
[ -f "$TARGET" ] || { echo "[KEXEC] ERROR: underhill binary not found at $TARGET" >&2; exit 1; }
cp "$TARGET" "$INITRD_DIR/underhill-init"
cp "$TARGET" "$INITRD_DIR/bin/openvmm_hcl"
chmod 755 "$INITRD_DIR/underhill-init" "$INITRD_DIR/bin/openvmm_hcl"

# Module staging (updated layout: modules are flat under /boot/modules)
echo "[KEXEC] Staging required kernel modules"
mkdir -p "$INITRD_DIR/lib/modules"
stage_mod() {
    m=$1; src="/boot/modules/$m"; dst="$INITRD_DIR/lib/modules/$m";
    if [ -f "$src" ]; then
        cp "$src" "$dst"
    else
        echo "[KEXEC] WARN missing $src" >&2
    fi
}
stage_mod pci-hyperv-intf.ko
stage_mod pci-hyperv.ko
stage_mod hv_storvsc.ko

# Initramfs
IMG_PATH="/tmp/initramfs.gz"
CPIO_TMP="/tmp/initramfs.cpio"
rm -f "$IMG_PATH" "$CPIO_TMP"
( cd "$INITRD_DIR" && find . -print0 | cpio --null -o -H newc > "$CPIO_TMP" )
gzip -1 -c "$CPIO_TMP" > "$IMG_PATH"
rm -f "$CPIO_TMP"

KEXEC_BIN="/sbin/kexec"
KERNEL_IMAGE="/boot/bzImage"
[ -x "$KEXEC_BIN" ] || { echo "[KEXEC] ERROR: $KEXEC_BIN not found/executable" >&2; exit 1; }
[ -f "$KERNEL_IMAGE" ] || { echo "[KEXEC] ERROR: $KERNEL_IMAGE not found" >&2; exit 1; }

# Kernel cmdline (original reproduction set)
CMDLINE="loglevel=8 log_buf_len=128K printk.time=1 console_msg_format=syslog"
CMDLINE="$CMDLINE uio_hv_generic.no_mask=1 coredump_filter=0x33"
CMDLINE="$CMDLINE cpufreq.off=1 cpuidle.off=1 cryptomgr.notests idle=halt"
CMDLINE="$CMDLINE initcall_blacklist=init_real_mode,sbf_init lpj=3000000"
CMDLINE="$CMDLINE no_timer_check noxsave oops=panic panic_on_warn=0 panic_print=0 panic=-1"
CMDLINE="$CMDLINE printk.devkmsg=on reboot=t rootfstype=tmpfs tsc=reliable unknown_nmi_panic=1"
CMDLINE="$CMDLINE vfio_pci.ids=1414:00ba vfio.enable_unsafe_noiommu_mode=1"
CMDLINE="$CMDLINE hv_storvsc.storvsc_vcpus_per_sub_channel=2048 hv_storvsc.storvsc_max_hw_queues=2 hv_storvsc.storvsc_ringbuffer_size=0x8000"
CMDLINE="$CMDLINE MIMALLOC_ARENA_EAGER_COMMIT=0"
CMDLINE="$CMDLINE clearcpuid=pcid iommu=off pci=off swiotlb=1,1"
CMDLINE="$CMDLINE console=ttyS2,115200 boot_cpus=0 hv_vmbus.message_connection_id=0x800074"
CMDLINE="$CMDLINE rdinit=/underhill-init UNDERHILL_DIAG=1 HVLITE_LOG=debug RUST_BACKTRACE=full OPENHCL_NVME_VFIO=1"
CMDLINE="$CMDLINE OPENHCL_KEXEC_SERVICING=1"
[ -n "${EXTRA_CMDLINE:-}" ] && CMDLINE="$CMDLINE $EXTRA_CMDLINE"

# Kexec
echo "[KEXEC] Loading kernel with cmdline: $CMDLINE"
$KEXEC_BIN -l "$KERNEL_IMAGE" --initrd="$IMG_PATH" --command-line="$CMDLINE" --reset-vga
echo "[KEXEC] Executing kexec"
exec "$KEXEC_BIN" -e