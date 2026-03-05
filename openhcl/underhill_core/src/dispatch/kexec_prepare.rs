// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Native kexec pre-load: builds initramfs in-process and stages kernel
//! via `kexec -l`.
//!
//! This replaces the `kexec_prepare.sh` shell script with native Rust code
//! that builds the cpio newc archive directly in memory, streaming it through
//! a `gzip -1` subprocess into a temp file. The only subprocesses spawned are
//! `gzip` (compression) and `kexec` (kernel staging) — down from ~11 process
//! invocations in the shell script approach.
//!
//! The cpio format implementation follows the "newc" (SVR4 with no CRC)
//! specification as documented in the Linux kernel source:
//! `Documentation/driver-api/early-userspace/buffer-format.rst`

use anyhow::Context;
use cvm_tracing::CVM_ALLOWED;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

const READY_FLAG: &str = "/run/kexec-ready";
const KEXEC_BIN: &str = "/sbin/kexec";
const KERNEL_IMAGE: &str = "/boot/bzImage";

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
/// `kexec -l` stages the kernel for a future `kexec -e`.
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

    tracing::info!(
        CVM_ALLOWED,
        initramfs_size,
        "loading kernel with kexec -l"
    );

    // Stage the kernel in kexec memory. After this, only `kexec -e` is
    // needed to jump to the new kernel.
    let status = std::process::Command::new(KEXEC_BIN)
        .arg("-l")
        .arg(KERNEL_IMAGE)
        .arg(format!("--initrd={}", img_path))
        .arg(format!("--command-line={}", cmdline))
        .arg("--reset-vga")
        .status()
        .context("failed to execute kexec -l")?;

    // The kernel image is now staged in kernel memory by the kexec
    // subsystem, so the temp file is no longer needed.
    let _ = std::fs::remove_file(img_path);

    if !status.success() {
        anyhow::bail!("kexec -l exited with status: {}", status);
    }

    // Signal that kexec is pre-loaded and ready.
    std::fs::write(READY_FLAG, b"").context("failed to write kexec-ready flag")?;

    Ok(())
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
