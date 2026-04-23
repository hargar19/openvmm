// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Thin wrapper around the Linux `kexec_file_load` syscall.
//!
//! This crate exists because `underhill_core` uses `#![forbid(unsafe_code)]`.
//! The unsafe syscall is isolated here behind a safe API.

#![allow(unsafe_code)]

use std::ffi::CStr;
use std::io;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::OwnedFd;
use std::os::unix::io::RawFd;

/// `kexec_file_load` syscall number on x86_64.
const SYS_KEXEC_FILE_LOAD: libc::c_long = 320;

/// Skip reading an initrd from the initrd_fd.
pub const KEXEC_FILE_NO_INITRAMFS: u64 = 0x04;

/// Call the `kexec_file_load(2)` syscall to stage a kernel for kexec.
///
/// This bypasses userspace kexec-tools entirely. The kernel handles ELF
/// parsing and relocation, which is required for vmlinux images whose
/// `PT_LOAD` segments overlap the running kernel's reserved memory.
///
/// # Arguments
/// * `kernel_fd` - File descriptor for the kernel image (vmlinux).
/// * `initrd_fd` - File descriptor for the initrd/initramfs (-1 if none).
/// * `cmdline`   - Null-terminated kernel command line.
/// * `flags`     - Flags (e.g. `KEXEC_FILE_NO_INITRAMFS`).
///
/// # Returns
/// `Ok(())` on success, or an `io::Error` on failure.
pub fn kexec_file_load(kernel_fd: RawFd, initrd_fd: RawFd, cmdline: &CStr, flags: u64) -> io::Result<()> {
    let cmdline_bytes = cmdline.to_bytes_with_nul();
    // SAFETY: kexec_file_load is a Linux syscall. The file descriptors must
    // be valid for the duration of this call (guaranteed by the caller
    // keeping the File handles alive). The cmdline pointer is valid for
    // the CStr's lifetime which exceeds this call.
    let ret = unsafe {
        libc::syscall(
            SYS_KEXEC_FILE_LOAD,
            kernel_fd as libc::c_long,
            initrd_fd as libc::c_long,
            cmdline_bytes.len() as libc::c_ulong,
            cmdline_bytes.as_ptr() as libc::c_long,
            flags as libc::c_ulong,
        )
    };
    if ret != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Trigger a kexec reboot into the previously staged kernel.
///
/// This is equivalent to `kexec -e` / `reboot(LINUX_REBOOT_CMD_KEXEC)`.
/// If successful, this function does not return — the current kernel is
/// replaced by the staged one.
pub fn kexec_reboot() -> io::Error {
    // SAFETY: reboot(2) with LINUX_REBOOT_CMD_KEXEC triggers the kexec
    // jump. The magic values are required by the kernel ABI.
    unsafe {
        libc::reboot(libc::LINUX_REBOOT_CMD_KEXEC);
    }
    // If we get here, the reboot syscall failed.
    io::Error::last_os_error()
}

/// Create a `memfd` — an anonymous in-memory file descriptor.
///
/// Returns an `OwnedFd` that behaves like a regular file but is not backed
/// by any filesystem. Useful for passing data to `kexec_file_load` without
/// consuming tmpfs space.
pub fn memfd_create(name: &CStr) -> io::Result<OwnedFd> {
    // SAFETY: memfd_create is a Linux syscall that creates an anonymous file.
    // The name pointer is valid for the CStr's lifetime which exceeds this call.
    let raw_fd = unsafe { libc::memfd_create(name.as_ptr(), libc::MFD_CLOEXEC) };
    if raw_fd < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: raw_fd is a valid fd just returned by memfd_create.
    Ok(unsafe { OwnedFd::from_raw_fd(raw_fd) })
}
