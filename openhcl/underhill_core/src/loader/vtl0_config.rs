// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Implements reading fixed-at-IGVM-build-time measured config. This is
//! configuration information that is deposited into the guest address space that
//! is measured as part of the partition's launch.

use super::LoadKind;
use super::PV_CONFIG_BASE_PAGE;
use super::VpContext;
use super::memory_range_from_page_region;
use guestmem::GuestMemory;
use guestmem::ranges::PagedRange;
use hvdef::HV_PAGE_SIZE;
use igvm::registers::UnsupportedRegister;
use loader_defs::paravisor::ParavisorMeasuredVtl0Config;
use loader_defs::shim::save_restore::MeasuredVtl0ConfigState;
use loader_defs::shim::save_restore::MeasuredVtl0LinuxInitrd;
use loader_defs::shim::save_restore::MeasuredVtl0LinuxState;
use loader_defs::shim::save_restore::MeasuredVtl0UefiState;
use memory_range::MemoryRange;
use std::ffi::CString;
use std::io::{Cursor, Read};
use thiserror::Error;
use tracing::instrument;
use zerocopy::FromZeros;
use zerocopy::IntoBytes;

/// Errors returned when reading measured config.
#[derive(Debug, Error)]
pub enum Error {
    #[error("accessing guest memory failed")]
    GuestMemoryAccess(#[source] guestmem::GuestMemoryError),
    #[error("invalid vtl0 vp context")]
    InvalidVtl0VpContext,
    #[error("uefi info did not contain firmware region")]
    UefiFirmwareRegion,
    #[error("linux info did not contain linux kernel region")]
    LinuxKernelRegion,
    #[error("measured vtl0 config magic mismatch: expected {expected:#x}, got {found:#x}")]
    MissingMagic { expected: u64, found: u64 },
    #[error("persisted measured vtl0 command line invalid")]
    InvalidPersistedCommandLine,
    #[cfg(guest_arch = "x86_64")]
    #[error("unsupported x64 register")]
    UnsupportedRegister(#[source] UnsupportedRegister<igvm::hv_defs::HvX64RegisterName>),
    #[cfg(guest_arch = "aarch64")]
    #[error("unsupported aarch64 register")]
    UnsupportedRegister(#[source] UnsupportedRegister<igvm::hv_defs::HvArm64RegisterName>),
}

#[derive(Debug)]
pub struct UefiInfo {
    pub firmware_memory: MemoryRange,
    pub vp_context: VpContext,
    pub vp_context_page: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct LinuxInfo {
    /// The region of memory used by the kernel.
    pub kernel_range: MemoryRange,
    /// The entrypoint of the kernel.
    pub kernel_entrypoint: u64,
    /// The (base address, size in bytes) of the initrd.
    pub initrd: Option<(u64, u64)>,
    /// The command line to pass to the kernel.
    pub command_line: Option<CString>,
}

#[derive(Debug)]
pub struct MeasuredVtl0Info {
    pub supports_pcat: bool,
    pub supports_uefi: Option<UefiInfo>,
    pub supports_linux: Option<LinuxInfo>,
    persisted_state: Option<MeasuredVtl0ConfigState>,
}

impl MeasuredVtl0Info {
    /// Read measured VTL0 load information from guest memory.
    #[instrument(skip_all, fields(CVM_ALLOWED))]
    pub fn read_from_memory(gm: &GuestMemory) -> Result<Self, Error> {
        // Read the measured config, noting which pages the config was stored
        // in. These pages will need to be zeroed out afterwards (the memory
        // used here is part of VTL0's memory).
        let mut config_pages = Vec::new();

        let measured_config = gm
            .read_plain::<ParavisorMeasuredVtl0Config>(PV_CONFIG_BASE_PAGE * HV_PAGE_SIZE)
            .map_err(Error::GuestMemoryAccess)?;
        config_pages.push(PV_CONFIG_BASE_PAGE);

        // Verify the magic field is set.
        if measured_config.magic != ParavisorMeasuredVtl0Config::MAGIC {
            return Err(Error::MissingMagic {
                expected: ParavisorMeasuredVtl0Config::MAGIC,
                found: measured_config.magic,
            });
        }

        let supports_pcat = measured_config.supported_vtl0.pcat_supported();

        let supports_uefi = if measured_config.supported_vtl0.uefi_supported() {
            let uefi = &measured_config.uefi_info;
            let mut persisted_vp_context = None;
            let vp_context = match uefi.vtl0_vp_context.pages() {
                Some((vtl0_vp_context_page_base, vtl0_vp_context_page_count)) => {
                    assert!(vtl0_vp_context_page_base != 0);
                    assert_eq!(vtl0_vp_context_page_count, 1);

                    let mut vtl0_vp_context_raw: Vec<u8> = vec![0; HV_PAGE_SIZE as usize];
                    gm.read_at(
                        vtl0_vp_context_page_base * HV_PAGE_SIZE,
                        vtl0_vp_context_raw.as_mut_slice(),
                    )
                    .map_err(Error::GuestMemoryAccess)?;
                    config_pages.push(vtl0_vp_context_page_base);

                    persisted_vp_context = Some(vtl0_vp_context_raw.clone());
                    parse_vtl0_vp_context(&vtl0_vp_context_raw)?
                }
                None => VpContext::Vbs(Vec::new()),
            };

            Some(UefiInfo {
                firmware_memory: memory_range_from_page_region(&uefi.firmware)
                    .ok_or(Error::UefiFirmwareRegion)?,
                vp_context,
                vp_context_page: persisted_vp_context,
            })
        } else {
            None
        };

        let supports_linux = if measured_config.supported_vtl0.linux_direct_supported() {
            let linux = &measured_config.linux_info;

            let command_line = match linux.command_line.pages() {
                Some((page_base, page_count)) => {
                    // TODO: only single page for command line supported
                    assert_eq!(page_count, 1);
                    let mut raw = vec![0; HV_PAGE_SIZE as usize];
                    gm.read_at(page_base * HV_PAGE_SIZE, raw.as_mut_slice())
                        .map_err(Error::GuestMemoryAccess)?;
                    config_pages.push(page_base);

                    // Find the first null byte, and remove the rest of the bytes
                    // after that.
                    let nul = raw
                        .iter()
                        .position(|c| *c == 0)
                        .expect("command line is measured and should be valid");
                    raw.truncate(nul);
                    Some(CString::new(raw).expect("cstring should be valid"))
                }
                None => None,
            };

            let initrd = if let Some((initrd_base, _)) = linux.initrd_region.pages() {
                Some((initrd_base * HV_PAGE_SIZE, linux.initrd_size))
            } else {
                None
            };

            Some(LinuxInfo {
                kernel_range: memory_range_from_page_region(&linux.kernel_region)
                    .ok_or(Error::LinuxKernelRegion)?,
                kernel_entrypoint: linux.kernel_entrypoint,
                initrd,
                command_line,
            })
        } else {
            None
        };

        // Clear measured info from VTL0 memory.
        gm.zero_range(
            &PagedRange::new(0, config_pages.len() * HV_PAGE_SIZE as usize, &config_pages)
                .expect("page range is valid"),
        )
        .map_err(Error::GuestMemoryAccess)?;

        let persisted_state = Some(MeasuredVtl0ConfigState {
            supports_pcat,
            supports_uefi: supports_uefi.as_ref().map(|uefi| MeasuredVtl0UefiState {
                firmware_memory: uefi.firmware_memory,
                vp_context_page: uefi.vp_context_page.clone(),
            }),
            supports_linux: supports_linux.as_ref().map(|linux| MeasuredVtl0LinuxState {
                kernel_range: linux.kernel_range,
                kernel_entrypoint: linux.kernel_entrypoint,
                initrd: linux
                    .initrd
                    .map(|(base, len)| MeasuredVtl0LinuxInitrd { base, len }),
                command_line: linux
                    .command_line
                    .as_ref()
                    .map(|cmd| cmd.as_bytes().to_vec()),
            }),
        });

        Ok(Self {
            supports_pcat,
            supports_uefi,
            supports_linux,
            persisted_state,
        })
    }

    /// Clear other VTL0 info from guest memory that were not loaded.
    pub fn finalize_load(&self, gm: &GuestMemory, load_kind: LoadKind) -> Result<(), Error> {
        if let Some(uefi_info) = &self.supports_uefi {
            if load_kind != LoadKind::Uefi {
                // Clear out UEFI firmware and misc pages.
                gm.fill_at(
                    uefi_info.firmware_memory.start(),
                    0,
                    uefi_info.firmware_memory.len() as usize,
                )
                .map_err(Error::GuestMemoryAccess)?;
            }
        }

        if let Some(linux_info) = &self.supports_linux {
            if load_kind != LoadKind::Linux {
                // Clear out Linux kernel and initrd if set.
                gm.fill_at(
                    linux_info.kernel_range.start(),
                    0,
                    linux_info.kernel_range.len() as usize,
                )
                .map_err(Error::GuestMemoryAccess)?;

                if let Some((initrd_base, initrd_len)) = linux_info.initrd {
                    gm.fill_at(initrd_base, 0, initrd_len as usize)
                        .map_err(Error::GuestMemoryAccess)?;
                }
            }
        }

        Ok(())
    }

    /// Return the persisted representation of this measured config, if available.
    pub fn persisted_state(&self) -> Option<MeasuredVtl0ConfigState> {
        self.persisted_state.clone()
    }

    /// Reconstruct measured VTL0 info from persisted servicing state.
    pub fn from_persisted(state: &MeasuredVtl0ConfigState) -> Result<Self, Error> {
        let supports_pcat = state.supports_pcat;

        let supports_uefi = if let Some(uefi_state) = &state.supports_uefi {
            let vp_context_page = uefi_state.vp_context_page.clone();
            let vp_context = if let Some(page) = &vp_context_page {
                parse_vtl0_vp_context(page.as_slice())?
            } else {
                VpContext::Vbs(Vec::new())
            };

            Some(UefiInfo {
                firmware_memory: uefi_state.firmware_memory,
                vp_context,
                vp_context_page,
            })
        } else {
            None
        };

        let supports_linux = if let Some(linux_state) = &state.supports_linux {
            let command_line = match &linux_state.command_line {
                Some(bytes) => Some(
                    CString::new(bytes.clone()).map_err(|_| Error::InvalidPersistedCommandLine)?,
                ),
                None => None,
            };

            Some(LinuxInfo {
                kernel_range: linux_state.kernel_range,
                kernel_entrypoint: linux_state.kernel_entrypoint,
                initrd: linux_state
                    .initrd
                    .as_ref()
                    .map(|initrd| (initrd.base, initrd.len)),
                command_line,
            })
        } else {
            None
        };

        Ok(Self {
            supports_pcat,
            supports_uefi,
            supports_linux,
            persisted_state: Some(state.clone()),
        })
    }
}

/// Parse and validate the raw byte VTL0 VP context into expected format
/// depending on isolation architecture.
fn parse_vtl0_vp_context(raw: &[u8]) -> Result<VpContext, Error> {
    // VBS format is a VbsVpContextHeader followed by VbsVpContextRegister.
    let mut header = igvm_defs::VbsVpContextHeader::new_zeroed();
    let mut reader = Cursor::new(raw);
    let mut registers = Vec::new();
    reader
        .read_exact(header.as_mut_bytes())
        .map_err(|_| Error::InvalidVtl0VpContext)?;

    for _ in 0..header.register_count {
        let mut reg = igvm_defs::VbsVpContextRegister::new_zeroed();
        reader
            .read_exact(reg.as_mut_bytes())
            .map_err(|_| Error::InvalidVtl0VpContext)?;

        #[cfg(guest_arch = "x86_64")]
        let igvm_reg: igvm::registers::X86Register =
            reg.try_into().map_err(Error::UnsupportedRegister)?;

        #[cfg(guest_arch = "aarch64")]
        let igvm_reg: igvm::registers::AArch64Register =
            reg.try_into().map_err(Error::UnsupportedRegister)?;

        registers.push(igvm_reg.into());
    }

    Ok(VpContext::Vbs(registers))
}
