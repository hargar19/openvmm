// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Native kexec pre-load: builds initramfs in-process and stages kernel
//! via the `kexec_file_load` syscall.
//!
//! This replaces the `kexec_prepare.sh` shell script with native Rust code
//! that builds the cpio newc archive directly in memory, streaming it through
//! a `gzip -1` subprocess into a temp file. The only subprocess spawned is
//! `gzip` (compression) — the kernel staging uses the `kexec_file_load`
//! syscall directly via the `kexec_sys` crate, bypassing userspace
//! kexec-tools entirely.
//!
//! Using `kexec_file_load` (rather than `kexec_load` via the userspace
//! `kexec -l` binary) is required for the future KHO (Kexec Handover)
//! support, which hooks into the `kexec_file_load` kernel path.
//!
//! The cpio format implementation follows the "newc" (SVR4 with no CRC)
//! specification as documented in the Linux kernel source:
//! `Documentation/driver-api/early-userspace/buffer-format.rst`

use anyhow::Context;
use cvm_tracing::CVM_ALLOWED;
use std::io;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::path::PathBuf;

const READY_FLAG: &str = "/run/kexec-ready";
const KERNEL_IMAGE: &str = "/boot/bzImage";
const VMLINUX_IMAGE: &str = "/boot/vmlinux";
const KEXEC_STUB_BIN: &str = "/boot/kexec_stub.bin";

/// Magic bytes at the start of the packed blob (must match kexec_stub).
const PACK_MAGIC: &[u8; 8] = b"KXSTUB\x01\x00";

// File type bits (from POSIX stat.h)
const S_IFDIR: u32 = 0o040_000;
const S_IFREG: u32 = 0o100_000;
const S_IFLNK: u32 = 0o120_000;
const S_IFCHR: u32 = 0o020_000;

/// Kernel modules to include in the initramfs.
///
/// Each entry is (directory_prefix, filename). The prefix controls load
/// order: `underhill_init` walks `/lib/modules/` sorted by filename, so
/// `000/` loads first, `999/` loads last.  This matches `rootfs.config`:
///   000/pci-hyperv-intf.ko  (PCI infrastructure, must load first)
///   001/pci-hyperv.ko       (PCI host bridge, depends on intf)
///   999/hv_storvsc.ko       (storage, slow to probe, should not block others)
const KERNEL_MODULES: &[(&str, &str)] = &[
    ("000", "pci-hyperv-intf.ko"),
    ("001", "pci-hyperv.ko"),
    ("999", "hv_storvsc.ko"),
];

/// Build the initramfs and stage the kernel for kexec, entirely in Rust.
///
/// This is functionally equivalent to `kexec_prepare.sh` but eliminates
/// the staging directory and most process spawning. The cpio archive is
/// built in memory and streamed through `gzip -1` to a temp file, then
/// the `kexec_file_load` syscall stages the kernel for a future
/// `reboot(LINUX_REBOOT_CMD_KEXEC)`.
pub fn prepare_kexec() -> anyhow::Result<()> {
    // Clean up any stale sentinel from a previous run.
    let _ = std::fs::remove_file(READY_FLAG);

    let binary_path = resolve_binary_path();
    let cmdline = build_cmdline().context("failed to build kernel command line")?;

    // Build cpio archive, compress via gzip, write to temp file.
    let img_path = "/tmp/initramfs.gz";
    build_and_compress_initramfs(&binary_path, img_path)
        .context("failed to build initramfs")?;

    let initramfs_size = std::fs::metadata(img_path)
        .map(|m| m.len())
        .unwrap_or(0);

    // Try the stub-based kexec path (loads uncompressed vmlinux via a bare-metal
    // stub). Falls back to the direct bzImage path if the stub or vmlinux are
    // not present.
    if Path::new(KEXEC_STUB_BIN).exists()
        && Path::new(VMLINUX_IMAGE).exists()
    {
        tracing::info!(
            CVM_ALLOWED,
            initramfs_size,
            "using kexec stub path with vmlinux"
        );
        prepare_kexec_stub(img_path, &cmdline)
            .context("kexec stub path failed")?;
    } else {
        tracing::info!(
            CVM_ALLOWED,
            initramfs_size,
            "using direct bzImage kexec path"
        );
        prepare_kexec_bzimage(img_path, &cmdline)
            .context("direct bzImage kexec path failed")?;
    }

    // The kernel image is now staged in kernel memory by the kexec
    // subsystem, so the temp file is no longer needed.
    let _ = std::fs::remove_file(img_path);

    // Signal that kexec is pre-loaded and ready.
    std::fs::write(READY_FLAG, b"").context("failed to write kexec-ready flag")?;

    Ok(())
}

/// Stage kexec via the stub: build a single bzImage kernel file that contains
/// both the stub binary and the packed blob (vmlinux + initrd), then call
/// kexec_file_load with no separate initrd.
///
/// The packed blob is embedded directly in the PM kernel file (after the stub
/// binary + BSS padding) so that it resides in VTL2 memory. This avoids the
/// issue where kexec places a separate initrd at a low GPA that belongs to
/// VTL0 and is inaccessible to VTL2 after kexec.
///
/// Uses memfd for the kernel file to avoid consuming tmpfs space. Streams
/// vmlinux and initrd directly from their files to minimize peak memory.
fn prepare_kexec_stub(initrd_path: &str, cmdline: &str) -> anyhow::Result<()> {
    // Read the stub flat binary (tiny, ~14 KB).
    let mut stub_bin = std::fs::read(KEXEC_STUB_BIN)
        .with_context(|| format!("failed to read kexec stub: {}", KEXEC_STUB_BIN))?;

    // Get file sizes for the pack header (without reading files into memory).
    let vmlinux_size = std::fs::metadata(VMLINUX_IMAGE)
        .with_context(|| format!("failed to stat vmlinux: {}", VMLINUX_IMAGE))?
        .len();
    let initrd_size = std::fs::metadata(initrd_path)
        .with_context(|| format!("failed to stat initrd: {}", initrd_path))?
        .len();

    // The packed blob (magic + sizes + vmlinux + padding + initrd) is embedded
    // directly in the bzImage "kernel" file, after the stub binary + BSS padding.
    // The initrd must start at a page-aligned offset within the packed blob
    // so the kernel's free_initrd_mem() gets a page-aligned address.
    let pack_header_size = PACK_MAGIC.len() + 8 + 8; // magic + vmlinux_size + initrd_size
    let vmlinux_padded_end = (pack_header_size + vmlinux_size as usize + 0xFFF) & !0xFFF;
    let packed_blob_size = vmlinux_padded_end + initrd_size as usize;

    // pack_start: offset in PM kernel where packed blob begins.
    // Must be past stub file data + BSS + page tables (allocated at runtime
    // from memory past _end). 64 KB headroom past stub binary covers BSS,
    // the runtime page tables, and alignment.
    let pack_start = (0x200 + stub_bin.len() + 64 * 1024 + 0xFFF) & !0xFFF;

    // init_size: total memory the kexec loader must allocate for the PM kernel.
    let init_size = (pack_start + packed_blob_size + 0xFFF) & !0xFFF;

    // Patch the stub binary with pack offset and size.
    // entry.S places pack_offset at byte 8 and pack_size at byte 16 within
    // the flat binary (after a jmp instruction that skips over them).
    anyhow::ensure!(stub_bin.len() >= 24, "stub binary too small for pack info header");
    stub_bin[8..16].copy_from_slice(&(pack_start as u64).to_le_bytes());
    stub_bin[16..24].copy_from_slice(&(packed_blob_size as u64).to_le_bytes());

    tracing::info!(
        CVM_ALLOWED,
        stub_size = stub_bin.len(),
        vmlinux_size,
        initrd_size,
        pack_start,
        init_size,
        "constructing kexec stub bzImage with embedded packed blob"
    );

    // Build the 1024-byte bzImage header.
    let header = construct_bzimage_header(init_size);

    // Stream everything into a single memfd:
    //   [header:1024][startup_32:0x200][stub_bin][zero padding to pack_start][packed blob]
    let cname = std::ffi::CString::new("kexec_bzimage").unwrap();
    let fd = kexec_sys::memfd_create(&cname)
        .context("memfd_create failed for bzImage")?;
    let mut memfd = std::fs::File::from(fd);

    // 1. Write the 1024-byte bzImage header.
    memfd.write_all(&header).context("failed to write bzImage header")?;

    // 2. Write 0x200 bytes startup_32 padding.
    memfd.write_all(&[0u8; 0x200]).context("failed to write startup_32 padding")?;

    // 3. Write the stub binary.
    memfd.write_all(&stub_bin).context("failed to write stub binary")?;

    // 4. Zero-pad from current PM offset to pack_start.
    let current_pm_offset = 0x200 + stub_bin.len();
    let padding_size = pack_start - current_pm_offset;
    let zero_buf = [0u8; 4096];
    let mut remaining = padding_size;
    while remaining > 0 {
        let chunk = remaining.min(zero_buf.len());
        memfd.write_all(&zero_buf[..chunk]).context("failed to write padding")?;
        remaining -= chunk;
    }

    // 5. Write pack header: [magic:8][vmlinux_size:8][initrd_size:8]
    memfd.write_all(PACK_MAGIC).context("failed to write pack magic")?;
    memfd.write_all(&vmlinux_size.to_le_bytes()).context("failed to write vmlinux size")?;
    memfd.write_all(&initrd_size.to_le_bytes()).context("failed to write initrd size")?;

    // 6. Stream vmlinux from file to memfd.
    let mut vmlinux_file = std::fs::File::open(VMLINUX_IMAGE)
        .with_context(|| format!("failed to open vmlinux: {}", VMLINUX_IMAGE))?;
    io::copy(&mut vmlinux_file, &mut memfd).context("failed to copy vmlinux to memfd")?;
    drop(vmlinux_file);

    // 7. Pad after vmlinux to page-align the initrd start.
    let vmlinux_end_offset = pack_header_size + vmlinux_size as usize;
    let initrd_pad = vmlinux_padded_end - vmlinux_end_offset;
    if initrd_pad > 0 {
        let pad_buf = [0u8; 4096];
        let mut pad_remaining = initrd_pad;
        while pad_remaining > 0 {
            let chunk = pad_remaining.min(pad_buf.len());
            memfd.write_all(&pad_buf[..chunk]).context("failed to write initrd padding")?;
            pad_remaining -= chunk;
        }
    }

    // 8. Stream initrd from file to memfd.
    let mut initrd_file = std::fs::File::open(initrd_path)
        .with_context(|| format!("failed to open initrd: {}", initrd_path))?;
    io::copy(&mut initrd_file, &mut memfd).context("failed to copy initrd to memfd")?;
    drop(initrd_file);

    // Seek back to start for kexec_file_load.
    io::Seek::seek(&mut memfd, io::SeekFrom::Start(0))
        .context("failed to seek bzImage memfd")?;
    let bzimage_fd = std::os::unix::io::OwnedFd::from(memfd);

    let cmdline_cstr = std::ffi::CString::new(cmdline.to_owned())
        .context("kernel command line contains null byte")?;

    // Stage via kexec_file_load. No separate initrd — the packed blob is
    // embedded in the kernel file and the stub finds it via payload_offset.
    kexec_sys::kexec_file_load(
        bzimage_fd.as_raw_fd(),
        -1,
        &cmdline_cstr,
        kexec_sys::KEXEC_FILE_NO_INITRAMFS,
    )
    .context("kexec_file_load syscall failed (stub path)")?;

    Ok(())
}

/// Stage kexec via the direct bzImage path (original approach).
fn prepare_kexec_bzimage(initrd_path: &str, cmdline: &str) -> anyhow::Result<()> {
    let kernel_file = std::fs::File::open(KERNEL_IMAGE)
        .with_context(|| format!("failed to open kernel image: {}", KERNEL_IMAGE))?;
    let initrd_file = std::fs::File::open(initrd_path)
        .with_context(|| format!("failed to open initrd: {}", initrd_path))?;

    let cmdline_cstr = std::ffi::CString::new(cmdline.to_owned())
        .context("kernel command line contains null byte")?;

    kexec_sys::kexec_file_load(
        kernel_file.as_raw_fd(),
        initrd_file.as_raw_fd(),
        &cmdline_cstr,
        0,
    )
    .context("kexec_file_load syscall failed")?;

    Ok(())
}

/// Construct a minimal bzImage header for the stub kernel.
///
/// The bzImage header is 1024 bytes (setup_sects=1, so 2 sectors).
/// After the header, the PM kernel contains: 0x200 bytes startup_32 padding,
/// the stub binary, zero padding for BSS, and the packed blob.
fn construct_bzimage_header(init_size: usize) -> [u8; 1024] {
    let mut header = [0u8; 1024];

    // setup_sects = 1 (header is boot sector + 1 setup sector = 1024 bytes)
    header[0x1F1] = 1;

    // boot_flag = 0xAA55
    header[0x1FE] = 0x55;
    header[0x1FF] = 0xAA;

    // header magic = "HdrS" (0x53726448 LE)
    header[0x202] = 0x48; // 'H'
    header[0x203] = 0x64; // 'd'
    header[0x204] = 0x72; // 'r'
    header[0x205] = 0x53; // 'S'

    // version = 0x020F (boot protocol 2.15)
    header[0x206] = 0x0F;
    header[0x207] = 0x02;

    // loadflags = LOADED_HIGH (0x01)
    header[0x211] = 0x01;

    // cmdline_size = 0xFFFF (64 KB max) — at absolute offset 0x238
    // (setup_header starts at 0x1F1, cmdline_size is at struct offset 0x47)
    header[0x238..0x23C].copy_from_slice(&0xFFFFu32.to_le_bytes());

    // kernel_alignment = 0x1000 (4 KB)
    header[0x230..0x234].copy_from_slice(&0x1000u32.to_le_bytes());

    // relocatable_kernel = 1
    header[0x234] = 1;

    // xloadflags: XLF_KERNEL_64 (0x01) | XLF_CAN_BE_LOADED_ABOVE_4G (0x02) | XLF_5LEVEL (0x10)
    header[0x236..0x238].copy_from_slice(&0x0013u16.to_le_bytes());

    // init_size: total memory for PM kernel (stub + BSS + packed blob).
    header[0x260..0x264].copy_from_slice(&(init_size as u32).to_le_bytes());

    header
}

/// Resolve the path to the underhill binary on disk.
///
/// Follows the `/underhill-init` symlink to find the actual binary,
/// falling back to `/usr/bin/openvmm_hcl` if the link doesn't exist.
fn resolve_binary_path() -> PathBuf {
    std::fs::read_link("/underhill-init")
        .map(|p| {
            if p.is_absolute() {
                p
            } else {
                PathBuf::from("/").join(p)
            }
        })
        .unwrap_or_else(|_| PathBuf::from("/usr/bin/openvmm_hcl"))
}

/// Build the kernel command line for the kexec'd kernel.
///
/// Reads `/proc/cmdline`, strips `boot_cpus=` (so all CPUs SMP-boot after
/// kexec when sidecar is no longer active), and adds `OPENHCL_KEXEC_SERVICING=1`
/// to tell the new `underhill_core` instance to read persisted state instead
/// of fetching it from the host.
fn build_cmdline() -> anyhow::Result<String> {
    let raw = std::fs::read_to_string("/proc/cmdline")
        .context("failed to read /proc/cmdline")?;

    let mut cmdline: String = raw
        .split_whitespace()
        .filter(|w| !w.starts_with("boot_cpus="))
        .collect::<Vec<_>>()
        .join(" ");

    if !cmdline.contains("OPENHCL_KEXEC_SERVICING=") {
        cmdline.push_str(" OPENHCL_KEXEC_SERVICING=1");
    }

    if let Some(extra) = std::env::var_os("EXTRA_CMDLINE") {
        cmdline.push(' ');
        cmdline.push_str(&extra.to_string_lossy());
    }

    Ok(cmdline)
}

/// Build a gzip-compressed cpio newc archive and write it to `output_path`.
///
/// The cpio archive is built in memory and compressed in-process using
/// `flate2::GzEncoder`. This avoids creating a staging directory, spawning
/// shell utilities, and eliminates the pipe overhead of the gzip subprocess
/// that dominated the previous ~1.3s build time.
fn build_and_compress_initramfs(binary_path: &Path, output_path: &str) -> anyhow::Result<()> {
    let output_file = std::fs::File::create(output_path)
        .with_context(|| format!("failed to create {}", output_path))?;

    // Compress in-process: GzEncoder wraps the output file and compresses
    // all writes on the fly. Level 1 (fast) matches the previous gzip -1.
    // This eliminates the gzip subprocess and all pipe I/O overhead.
    let gz = flate2::GzBuilder::new()
        .write(output_file, flate2::Compression::fast());

    // BufWriter batches the many small cpio header writes into fewer
    // compress/write calls.
    {
        let mut out = io::BufWriter::with_capacity(64 * 1024, gz);
        let mut inode = 1u32;

        // Directories
        for dir in &[
            ".", "bin", "dev", "etc", "proc", "run", "sys", "tmp", "lib",
            "lib/modules", "lib/modules/000", "lib/modules/001", "lib/modules/999",
        ] {
            write_cpio_entry(&mut out, &mut inode, dir, S_IFDIR | 0o755, 2, &[], 0, 0)?;
        }

        // Pre-mount device nodes — underhill_init's init_logging() opens
        // these BEFORE devtmpfs is mounted on /dev.
        write_cpio_entry(
            &mut out,
            &mut inode,
            "dev/null",
            S_IFCHR | 0o666,
            1,
            &[],
            1,
            3,
        )?;
        write_cpio_entry(
            &mut out,
            &mut inode,
            "dev/kmsg",
            S_IFCHR | 0o666,
            1,
            &[],
            1,
            11,
        )?;
        write_cpio_entry(
            &mut out,
            &mut inode,
            "dev/ttyprintk",
            S_IFCHR | 0o644,
            1,
            &[],
            5,
            3,
        )?;

        // Symlinks
        write_cpio_entry(
            &mut out,
            &mut inode,
            "dev/console",
            S_IFLNK | 0o777,
            1,
            b"ttyprintk",
            0,
            0,
        )?;
        write_cpio_entry(
            &mut out,
            &mut inode,
            "underhill-init",
            S_IFLNK | 0o777,
            1,
            b"/bin/openvmm_hcl",
            0,
            0,
        )?;

        // Underhill binary — read directly from disk, no staging copy.
        let binary_data = std::fs::read(binary_path)
            .with_context(|| format!("failed to read {}", binary_path.display()))?;

        tracing::info!(
            CVM_ALLOWED,
            binary_size = binary_data.len(),
            "read underhill binary for initramfs"
        );

        write_cpio_entry(
            &mut out,
            &mut inode,
            "bin/openvmm_hcl",
            S_IFREG | 0o755,
            1,
            &binary_data,
            0,
            0,
        )?;

        // Kernel modules — placed in numbered subdirectories to control
        // load order (underhill_init walks /lib/modules/ sorted by name).
        for &(prefix, module) in KERNEL_MODULES {
            let src = format!("/boot/modules/{}", module);
            match std::fs::read(&src) {
                Ok(data) => {
                    let name = format!("lib/modules/{}/{}", prefix, module);
                    write_cpio_entry(
                        &mut out,
                        &mut inode,
                        &name,
                        S_IFREG | 0o644,
                        1,
                        &data,
                        0,
                        0,
                    )?;
                }
                Err(e) => {
                    tracing::warn!(
                        CVM_ALLOWED,
                        module,
                        error = %e,
                        "missing kernel module, skipping"
                    );
                }
            }
        }

        // TRAILER marks end of archive.
        write_cpio_trailer(&mut out)?;

        out.flush().context("failed to flush cpio data")?;

        // Finish the gzip stream: flush compressed data and write the
        // gzip footer (CRC32 + length). into_inner() returns the
        // GzEncoder, then finish() returns the underlying File.
        let gz = out.into_inner().context("failed to flush BufWriter")?;
        gz.finish().context("failed to finalize gzip stream")?;
    }

    Ok(())
}

/// Write a single cpio "newc" (SVR4 no CRC) entry to the output stream.
///
/// The newc header is 110 bytes of ASCII hex:
///   magic(6) + inode(8) + mode(8) + uid(8) + gid(8) + nlink(8) +
///   mtime(8) + filesize(8) + devmajor(8) + devminor(8) + rdevmajor(8) +
///   rdevminor(8) + namesize(8) + checksum(8) = 110 bytes
///
/// The filename (with NUL terminator) is padded to a 4-byte boundary.
/// The file data (if any) is also padded to a 4-byte boundary.
fn write_cpio_entry(
    out: &mut impl Write,
    inode: &mut u32,
    name: &str,
    mode: u32,
    nlink: u32,
    data: &[u8],
    rdev_major: u32,
    rdev_minor: u32,
) -> io::Result<()> {
    let ino = *inode;
    *inode += 1;

    let namesize = name.len() + 1; // including NUL terminator
    let filesize = data.len();

    // 110-byte ASCII header
    write!(
        out,
        "070701\
         {:08X}{:08X}{:08X}{:08X}{:08X}{:08X}\
         {:08X}{:08X}{:08X}{:08X}{:08X}\
         {:08X}{:08X}",
        ino,            // inode
        mode,           // mode
        0u32,           // uid
        0u32,           // gid
        nlink,          // nlink
        0u32,           // mtime
        filesize,       // filesize
        0u32,           // devmajor (device containing file)
        0u32,           // devminor
        rdev_major,     // rdevmajor (device node major)
        rdev_minor,     // rdevminor (device node minor)
        namesize,       // namesize (including NUL)
        0u32,           // checksum (unused in newc)
    )?;

    // Name + NUL, padded to 4-byte boundary
    out.write_all(name.as_bytes())?;
    out.write_all(&[0])?;
    let header_plus_name = 110 + namesize;
    let name_padding = (4 - (header_plus_name % 4)) % 4;
    if name_padding > 0 {
        out.write_all(&[0u8; 3][..name_padding])?;
    }

    // File data, padded to 4-byte boundary
    if !data.is_empty() {
        out.write_all(data)?;
        let data_padding = (4 - (filesize % 4)) % 4;
        if data_padding > 0 {
            out.write_all(&[0u8; 3][..data_padding])?;
        }
    }

    Ok(())
}

/// Write the cpio TRAILER entry that marks end of archive.
fn write_cpio_trailer(out: &mut impl Write) -> io::Result<()> {
    let name = "TRAILER!!!";
    let namesize = name.len() + 1; // 11

    write!(
        out,
        "070701\
         {:08X}{:08X}{:08X}{:08X}{:08X}{:08X}\
         {:08X}{:08X}{:08X}{:08X}{:08X}\
         {:08X}{:08X}",
        0u32,           // inode
        0u32,           // mode
        0u32,           // uid
        0u32,           // gid
        1u32,           // nlink
        0u32,           // mtime
        0u32,           // filesize
        0u32,           // devmajor
        0u32,           // devminor
        0u32,           // rdevmajor
        0u32,           // rdevminor
        namesize,       // namesize
        0u32,           // checksum
    )?;

    out.write_all(name.as_bytes())?;
    out.write_all(&[0])?;

    // Pad header+name to 4-byte boundary
    let header_plus_name = 110 + namesize;
    let padding = (4 - (header_plus_name % 4)) % 4;
    if padding > 0 {
        out.write_all(&[0u8; 3][..padding])?;
    }

    Ok(())
}
