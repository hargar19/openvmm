// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Code to read and validate runtime parameters. These come from a variety of
//! sources, such as the host or openhcl_boot.
//!
//! Note that host provided IGVM parameters are untrusted and dynamic at
//! runtime, unlike measured config. Parameters provided by openhcl_boot are
//! expected to be already validated by the bootloader.

use anyhow::{Context, ensure};
use bootloader_fdt_parser::IsolationType;
use bootloader_fdt_parser::ParsedBootDtInfo;
use cvm_tracing::CVM_ALLOWED;
use hvdef::HV_PAGE_SIZE;
use inspect::Inspect;
use loader_defs::paravisor::PARAVISOR_CONFIG_PPTT_PAGE_INDEX;
use loader_defs::paravisor::PARAVISOR_CONFIG_SLIT_PAGE_INDEX;
use loader_defs::paravisor::PARAVISOR_MEASURED_VTL2_CONFIG_PAGE_INDEX;
use loader_defs::paravisor::PARAVISOR_RESERVED_VTL2_SNP_CPUID_PAGE_INDEX;
use loader_defs::paravisor::PARAVISOR_RESERVED_VTL2_SNP_CPUID_SIZE_PAGES;
use loader_defs::paravisor::PARAVISOR_RESERVED_VTL2_SNP_SECRETS_PAGE_INDEX;
use loader_defs::paravisor::PARAVISOR_RESERVED_VTL2_SNP_SECRETS_SIZE_PAGES;
use loader_defs::paravisor::ParavisorMeasuredVtl2Config;
use loader_defs::shim::MemoryVtlType;
use loader_defs::shim::PersistedStateHeader;
use loader_defs::shim::save_restore::MeasuredVtl0ConfigState;
use loader_defs::shim::save_restore::MeasuredVtl2ConfigState;
use loader_defs::shim::save_restore::MemoryEntry;
use loader_defs::shim::save_restore::MmioEntry;
use loader_defs::shim::save_restore::SavedState;
use memory_range::MemoryRange;
use sparse_mmap::SparseMapping;
use std::convert::TryFrom;
use string_page_buf::StringBuffer;
use vm_topology::memory::MemoryRangeWithNode;
use zerocopy::Immutable;
use zerocopy::IntoBytes;
use zerocopy::KnownLayout;

/// Structure that holds parameters provided at runtime. Some are read from the
/// guest address space, and others from openhcl_boot provided via devicetree.
#[derive(Debug, Inspect)]
pub struct RuntimeParameters {
    parsed_openhcl_boot: ParsedBootDtInfo,
    slit: Option<Vec<u8>>,
    pptt: Option<Vec<u8>>,
    cvm_cpuid_info: Option<Vec<u8>>,
    snp_secrets: Option<Vec<u8>>,
    #[inspect(iter_by_index)]
    bootshim_logs: Vec<String>,
    bootshim_log_dropped: u16,
    #[inspect(skip)]
    persisted_saved_state: Option<SavedState>,
    #[inspect(skip)]
    measured_vtl2_config: ParavisorMeasuredVtl2Config,
}

impl RuntimeParameters {
    /// The overall memory map of the partition provided by the bootloader,
    /// including VTL2.
    pub fn partition_memory_map(&self) -> &[bootloader_fdt_parser::AddressRange] {
        &self.parsed_openhcl_boot.partition_memory_map
    }

    /// The parsed settings from device tree provided by openhcl_boot.
    pub fn parsed_openhcl_boot(&self) -> &ParsedBootDtInfo {
        &self.parsed_openhcl_boot
    }

    /// A sorted slice representing the memory used by VTL2.
    pub fn vtl2_memory_map(&self) -> &[MemoryRangeWithNode] {
        &self.parsed_openhcl_boot.vtl2_memory
    }

    /// The VM's ACPI SLIT table provided by the host.
    pub fn slit(&self) -> Option<&[u8]> {
        self.slit.as_deref()
    }

    /// The VM's ACPI PPTT table provided by the host.
    pub fn pptt(&self) -> Option<&[u8]> {
        self.pptt.as_deref()
    }

    /// The hardware supplied cpuid information for a CVM.
    pub fn cvm_cpuid_info(&self) -> Option<&[u8]> {
        self.cvm_cpuid_info.as_deref()
    }
    pub fn snp_secrets(&self) -> Option<&[u8]> {
        self.snp_secrets.as_deref()
    }

    /// The memory ranges to use for the private pool
    pub fn private_pool_ranges(&self) -> &[MemoryRangeWithNode] {
        &self.parsed_openhcl_boot.private_pool_ranges
    }

    /// The persisted saved state captured before this boot, if available.
    pub fn persisted_saved_state(&self) -> Option<&SavedState> {
        self.persisted_saved_state.as_ref()
    }

    /// The measured VTL2 config captured from bootshim.
    pub fn measured_vtl2_config(&self) -> &ParavisorMeasuredVtl2Config {
        &self.measured_vtl2_config
    }
}

/// Structure that holds the read IGVM parameters from the guest address space.
#[derive(Debug, Inspect)]
pub struct MeasuredVtl2Info {
    #[inspect(with = "inspect_helpers::accepted_regions")]
    accepted_regions: Vec<MemoryRange>,
    pub vtom_offset_bit: Option<u8>,
}

impl MeasuredVtl2Info {
    pub fn accepted_regions(&self) -> &[MemoryRange] {
        &self.accepted_regions
    }
}

/// Map of the portion of memory that contains the VTL2 parameters to read.
///
/// If configured, on drop this mapping zeroes out the specified config ranges.
struct Vtl2ParamsMap<'a> {
    mapping: SparseMapping,
    zero_on_drop: bool,
    ranges: &'a [MemoryRange],
}

impl<'a> Vtl2ParamsMap<'a> {
    fn new_internal(
        ranges: &'a [MemoryRange],
        writeable: bool,
        zero_on_drop: bool,
    ) -> anyhow::Result<Self> {
        // No overlaps.
        if let Some((l, r)) = ranges
            .iter()
            .zip(ranges.iter().skip(1))
            .find(|(l, r)| r.start() < l.end())
        {
            anyhow::bail!("range {r} overlaps {l}");
        }

        tracing::trace!("requested mapping ranges {:x?}", ranges);

        let base = ranges.first().context("no ranges")?.start();
        let size = ranges.last().unwrap().end() - base;

        let mapping = SparseMapping::new(size as usize)
            .context("failed to create a sparse mapping for vtl2params")?;

        let writeable = writeable || zero_on_drop;
        let dev_mem = fs_err::OpenOptions::new()
            .read(true)
            .write(writeable)
            .open("/dev/mem")?;
        for range in ranges {
            mapping
                .map_file(
                    (range.start() - base) as usize,
                    range.len() as usize,
                    dev_mem.file(),
                    range.start(),
                    writeable,
                )
                .context("failed to memory map igvm parameters")?;
        }

        Ok(Self {
            mapping,
            ranges,
            zero_on_drop,
        })
    }

    fn new(config_ranges: &'a [MemoryRange], zero_on_drop: bool) -> anyhow::Result<Self> {
        Self::new_internal(config_ranges, false, zero_on_drop)
    }

    // TODO: Consider not using /dev/mem and instead using mshv_vtl_low, which
    // would require not describing the memory to the kernel in the E820 map.
    fn new_writeable(ranges: &'a [MemoryRange]) -> anyhow::Result<Self> {
        Self::new_internal(ranges, true, false)
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> anyhow::Result<()> {
        Ok(self.mapping.write_at(offset, buf)?)
    }

    fn read_at(&self, offset: usize, buf: &mut [u8]) -> anyhow::Result<()> {
        Ok(self.mapping.read_at(offset, buf)?)
    }

    fn read_plain<T: IntoBytes + zerocopy::FromBytes + Immutable + KnownLayout>(
        &self,
        offset: usize,
    ) -> anyhow::Result<T> {
        Ok(self.mapping.read_plain(offset)?)
    }
}

impl Drop for Vtl2ParamsMap<'_> {
    fn drop(&mut self) {
        if self.zero_on_drop {
            let base = self
                .ranges
                .first()
                .expect("already checked that there is at least one range")
                .start();

            for range in self.ranges {
                self.mapping
                    .fill_at((range.start() - base) as usize, 0, range.len() as usize)
                    .unwrap();
            }
        }
    }
}

pub(crate) fn is_kexec_servicing_enabled() -> bool {
    std::env::var("OPENHCL_KEXEC_SERVICING")
        .map(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return false;
            }

            let lowered = trimmed.to_ascii_lowercase();
            !matches!(lowered.as_str(), "0" | "false" | "off" | "no")
        })
        .unwrap_or(false)
}

fn load_persisted_saved_state(parsed: &ParsedBootDtInfo) -> anyhow::Result<Option<SavedState>> {
    let header_range = [parsed.vtl2_persisted_header];
    let header_mapping =
        Vtl2ParamsMap::new(&header_range, false).context("failed to map persisted state header")?;
    let header: PersistedStateHeader = header_mapping
        .read_plain(0)
        .context("failed to read persisted state header")?;
    drop(header_mapping);

    if header.magic != PersistedStateHeader::MAGIC || header.protobuf_payload_len == 0 {
        return Ok(None);
    }

    if header.protobuf_base != parsed.vtl2_persisted_protobuf_region.start() {
        tracing::warn!(
            CVM_ALLOWED,
            persisted_base = header.protobuf_base,
            expected_base = parsed.vtl2_persisted_protobuf_region.start(),
            "persisted protobuf base mismatch"
        );
        return Ok(None);
    }

    if header.protobuf_region_len > parsed.vtl2_persisted_protobuf_region.len() {
        tracing::warn!(
            CVM_ALLOWED,
            persisted_len = header.protobuf_region_len,
            region_len = parsed.vtl2_persisted_protobuf_region.len(),
            "persisted protobuf region length exceeds reserved buffer"
        );
        return Ok(None);
    }

    if header.protobuf_payload_len > header.protobuf_region_len {
        tracing::warn!(
            CVM_ALLOWED,
            payload_len = header.protobuf_payload_len,
            region_len = header.protobuf_region_len,
            "persisted protobuf payload length exceeds region length"
        );
        return Ok(None);
    }

    let payload_len = usize::try_from(header.protobuf_payload_len)
        .context("persisted protobuf payload length does not fit usize")?;
    if payload_len == 0 {
        return Ok(None);
    }

    let region_len = usize::try_from(parsed.vtl2_persisted_protobuf_region.len())
        .context("persisted protobuf region length does not fit usize")?;
    if payload_len > region_len {
        tracing::warn!(
            CVM_ALLOWED,
            payload_len,
            region_len,
            "persisted protobuf payload longer than mapped region"
        );
        return Ok(None);
    }

    let protobuf_range = [parsed.vtl2_persisted_protobuf_region];
    let proto_mapping = Vtl2ParamsMap::new(&protobuf_range, false)
        .context("failed to map persisted protobuf region")?;
    let mut buf = vec![0u8; payload_len];
    proto_mapping
        .read_at(0, buf.as_mut_slice())
        .context("failed to read persisted state protobuf")?;
    drop(proto_mapping);

    let saved_state: SavedState =
        mesh_protobuf::decode(&buf).context("failed to decode persisted state protobuf")?;

    Ok(Some(saved_state))
}

/// Write persisted info into the bootshim described persisted region.
pub fn persist_state(
    parsed: &ParsedBootDtInfo,
    measured_config: &ParavisorMeasuredVtl2Config,
    measured_vtl0: Option<&MeasuredVtl0ConfigState>,
) -> anyhow::Result<()> {
    tracing::trace!(
        protobuf_region = ?parsed.vtl2_persisted_protobuf_region,
        "writing persisted protobuf"
    );

    let ranges = [parsed.vtl2_persisted_protobuf_region];
    let mapping =
        Vtl2ParamsMap::new_writeable(&ranges).context("failed to map persisted protobuf region")?;

    let measured_config_for_persist = if measured_config.magic == ParavisorMeasuredVtl2Config::MAGIC
    {
        Some(MeasuredVtl2ConfigState {
            magic: measured_config.magic,
            vtom_offset_bit: u32::from(measured_config.vtom_offset_bit),
        })
    } else {
        None
    };

    // Create the serialized data to write.
    let state = SavedState {
        partition_memory: parsed
            .partition_memory_map
            .iter()
            .filter_map(|r| match r {
                bootloader_fdt_parser::AddressRange::Memory(memory) => Some(MemoryEntry {
                    range: memory.range.range,
                    vnode: memory.range.vnode,
                    vtl_type: memory.vtl_usage,
                    igvm_type: memory.igvm_type.into(),
                }),
                bootloader_fdt_parser::AddressRange::Mmio(_) => None,
            })
            .collect(),
        partition_mmio: parsed
            .partition_memory_map
            .iter()
            .filter_map(|r| match r {
                bootloader_fdt_parser::AddressRange::Mmio(mmio) => Some(MmioEntry {
                    range: mmio.range,
                    vtl_type: match mmio.vtl {
                        bootloader_fdt_parser::Vtl::Vtl0 => MemoryVtlType::VTL0_MMIO,
                        bootloader_fdt_parser::Vtl::Vtl2 => MemoryVtlType::VTL2_MMIO,
                    },
                }),
                bootloader_fdt_parser::AddressRange::Memory(_) => None,
            })
            .collect(),
        measured_vtl2_config: measured_config_for_persist,
        measured_vtl0_config: measured_vtl0.cloned(),
    };

    let protobuf = mesh_protobuf::encode(state);
    tracing::trace!(len = protobuf.len(), "persisted protobuf len");

    mapping
        .write_at(0, protobuf.as_bytes())
        .context("failed to write persisted state protobuf")?;

    tracing::trace!(
        header_region = ?parsed.vtl2_persisted_header,
        "writing persisted header"
    );

    let ranges = [parsed.vtl2_persisted_header];
    let mapping =
        Vtl2ParamsMap::new_writeable(&ranges).context("unable to map persisted header")?;

    let header = PersistedStateHeader {
        magic: PersistedStateHeader::MAGIC,
        protobuf_base: parsed.vtl2_persisted_protobuf_region.start(),
        protobuf_region_len: parsed.vtl2_persisted_protobuf_region.len(),
        protobuf_payload_len: protobuf.len() as u64,
    };

    mapping.write_at(0, header.as_bytes())?;

    Ok(())
}

/// Reads the VTL 2 parameters from the config region and VTL2 reserved region.
pub fn read_vtl2_params() -> anyhow::Result<(RuntimeParameters, MeasuredVtl2Info)> {
    let parsed_openhcl_boot = ParsedBootDtInfo::new().context("failed to parse openhcl_boot dt")?;

    let mapping = Vtl2ParamsMap::new(&parsed_openhcl_boot.config_ranges, true)
        .context("failed to map igvm parameters")?;

    // For the various ACPI tables, read the header to see how big the table
    // is, then read the exact table.

    let slit = {
        let table_header: acpi_spec::Header = mapping
            .read_plain((PARAVISOR_CONFIG_SLIT_PAGE_INDEX * HV_PAGE_SIZE) as usize)
            .context("failed to read slit header")?;
        tracing::trace!(?table_header, "Read SLIT ACPI header");

        if table_header.length.get() == 0 {
            None
        } else {
            let mut slit: Vec<u8> = vec![0; table_header.length.get() as usize];
            mapping
                .read_at(
                    (PARAVISOR_CONFIG_SLIT_PAGE_INDEX * HV_PAGE_SIZE) as usize,
                    slit.as_mut_slice(),
                )
                .context("failed to read slit")?;
            Some(slit)
        }
    };

    let pptt = {
        let table_header: acpi_spec::Header = mapping
            .read_plain((PARAVISOR_CONFIG_PPTT_PAGE_INDEX * HV_PAGE_SIZE) as usize)
            .context("failed to read pptt header")?;
        tracing::trace!(?table_header, "Read PPTT ACPI header");

        if table_header.length.get() == 0 {
            None
        } else {
            let mut pptt: Vec<u8> = vec![0; table_header.length.get() as usize];
            mapping
                .read_at(
                    (PARAVISOR_CONFIG_PPTT_PAGE_INDEX * HV_PAGE_SIZE) as usize,
                    pptt.as_mut_slice(),
                )
                .context("failed to read pptt")?;
            Some(pptt)
        }
    };

    // Read SNP specific information from the reserved region.
    let (cvm_cpuid_info, snp_secrets) = {
        if parsed_openhcl_boot.isolation == IsolationType::Snp {
            let ranges = &[parsed_openhcl_boot.vtl2_reserved_range];
            let reserved_mapping =
                Vtl2ParamsMap::new(ranges, false).context("failed to map vtl2 reserved region")?;

            let mut cpuid_pages: Vec<u8> =
                vec![0; (PARAVISOR_RESERVED_VTL2_SNP_CPUID_SIZE_PAGES * HV_PAGE_SIZE) as usize];
            reserved_mapping
                .read_at(
                    (PARAVISOR_RESERVED_VTL2_SNP_CPUID_PAGE_INDEX * HV_PAGE_SIZE) as usize,
                    cpuid_pages.as_mut_slice(),
                )
                .context("failed to read cpuid pages")?;
            let mut secrets =
                vec![0; (PARAVISOR_RESERVED_VTL2_SNP_SECRETS_SIZE_PAGES * HV_PAGE_SIZE) as usize];
            reserved_mapping
                .read_at(
                    (PARAVISOR_RESERVED_VTL2_SNP_SECRETS_PAGE_INDEX * HV_PAGE_SIZE) as usize,
                    secrets.as_mut_slice(),
                )
                .context("failed to read secrets page")?;

            (Some(cpuid_pages), Some(secrets))
        } else {
            (None, None)
        }
    };

    // Read bootshim logs.
    let (bootshim_logs, bootshim_log_dropped) = {
        let range = *parsed_openhcl_boot
            .partition_memory_map
            .iter()
            .find(|range| range.vtl_usage() == MemoryVtlType::VTL2_BOOTSHIM_LOG_BUFFER)
            .context("no bootshim log buffer found")?
            .range();
        let ranges = &[range];
        let mapping =
            Vtl2ParamsMap::new(ranges, false).context("failed to map bootshim log buffer")?;

        let mut raw = vec![0; range.len() as usize];
        mapping
            .read_at(0, raw.as_mut_slice())
            .context("unable to read raw bootshim logs")?;

        let buf = StringBuffer::from_existing(raw.as_mut_slice())
            .context("bootshim buffer contents invalid")?;

        let bootshim_log_dropped = buf.dropped_messages();
        if bootshim_log_dropped != 0 {
            tracing::warn!(
                CVM_ALLOWED,
                bootshim_log_dropped,
                "bootshim logger dropped messages"
            );
        }

        (
            buf.contents().lines().map(|s| s.to_string()).collect(),
            bootshim_log_dropped,
        )
    };

    for line in &bootshim_logs {
        tracing::info!(CVM_ALLOWED, line, "openhcl_boot log");
    }

    let accepted_regions = if parsed_openhcl_boot.isolation != IsolationType::None {
        parsed_openhcl_boot.accepted_ranges.clone()
    } else {
        Vec::new()
    };

    let persisted_saved_state = load_persisted_saved_state(&parsed_openhcl_boot)
        .context("failed to load persisted saved state")?;

    let mut measured_config = mapping
        .read_plain::<ParavisorMeasuredVtl2Config>(
            (PARAVISOR_MEASURED_VTL2_CONFIG_PAGE_INDEX * HV_PAGE_SIZE) as usize,
        )
        .context("failed to read measured vtl2 config")?;

    drop(mapping);

    if measured_config.magic != ParavisorMeasuredVtl2Config::MAGIC {
        if is_kexec_servicing_enabled() {
            tracing::info!(
                CVM_ALLOWED,
                "measured VTL2 config magic missing, attempting persisted fallback"
            );

            match persisted_saved_state
                .as_ref()
                .and_then(|state| state.measured_vtl2_config.as_ref())
            {
                Some(persisted_config) => {
                    let vtom_offset_bit = u8::try_from(persisted_config.vtom_offset_bit)
                        .context("persisted vtom offset bit does not fit in u8")?;
                    tracing::info!(
                        CVM_ALLOWED,
                        vtom_offset_bit,
                        "restored measured VTL2 config from persisted state"
                    );
                    measured_config = ParavisorMeasuredVtl2Config {
                        magic: persisted_config.magic,
                        vtom_offset_bit,
                        padding: [0; 7],
                    };
                }
                None => {
                    tracing::warn!(
                        CVM_ALLOWED,
                        "persisted measured VTL2 config unavailable during kexec servicing boot; proceeding without vTOM offset"
                    );
                    measured_config = ParavisorMeasuredVtl2Config {
                        magic: ParavisorMeasuredVtl2Config::MAGIC,
                        vtom_offset_bit: 0,
                        padding: [0; 7],
                    };
                }
            }
        } else {
            ensure!(
                false,
                "invalid measured vtl2 config magic: expected {:#x}, got {:#x}",
                ParavisorMeasuredVtl2Config::MAGIC,
                measured_config.magic
            );
        }
    }

    ensure!(
        measured_config.magic == ParavisorMeasuredVtl2Config::MAGIC,
        "invalid measured vtl2 config magic after kexec fallback: expected {:#x}, got {:#x}",
        ParavisorMeasuredVtl2Config::MAGIC,
        measured_config.magic
    );

    let vtom_offset_bit = if measured_config.vtom_offset_bit == 0 {
        None
    } else {
        Some(measured_config.vtom_offset_bit)
    };

    let runtime_params = RuntimeParameters {
        parsed_openhcl_boot,
        slit,
        pptt,
        cvm_cpuid_info,
        snp_secrets,
        bootshim_logs,
        bootshim_log_dropped,
        persisted_saved_state,
        measured_vtl2_config: measured_config,
    };

    let measured_vtl2_info = MeasuredVtl2Info {
        accepted_regions,
        vtom_offset_bit,
    };

    Ok((runtime_params, measured_vtl2_info))
}

mod inspect_helpers {
    use super::*;

    pub(super) fn accepted_regions(regions: &[MemoryRange]) -> impl Inspect + '_ {
        inspect::iter_by_key(
            regions
                .iter()
                .map(|region| (region, inspect::AsDebug(region))), // TODO ??
        )
    }
}
