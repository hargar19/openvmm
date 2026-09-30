// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Extract and stage the matching kernel and initrd from a servicing IGVM.

use anyhow::Context;
use guest_emulation_transport::api::MAX_SERVICING_IGVM_SIZE;
use igvm::IgvmDirectiveHeader;
use igvm::IgvmFile;
use igvm::IsolationType;
use igvm_defs::IGVM_FIXED_HEADER;
use igvm_defs::IGVM_FIXED_HEADER_V2;
use igvm_defs::IGVM_VHS_PAGE_DATA;
use igvm_defs::IGVM_VHS_PAGE_TABLE_RELOCATION;
use igvm_defs::IGVM_VHS_PARAMETER_AREA;
use igvm_defs::IGVM_VHS_RELOCATABLE_REGION;
use igvm_defs::IGVM_VHS_SUPPORTED_PLATFORM;
use igvm_defs::IGVM_VHS_VARIABLE_HEADER;
use igvm_defs::IGVM_VHS_VP_CONTEXT;
use igvm_defs::IgvmArchitecture;
use igvm_defs::IgvmPageDataFlags;
use igvm_defs::IgvmPageDataType;
use igvm_defs::IgvmPlatformType;
use igvm_defs::IgvmVariableHeaderType;
use igvm_defs::VbsVpContextHeader;
use igvm_defs::VbsVpContextRegister;
use loader_defs::paravisor::KexecPayloadDescriptor;
use loader_defs::paravisor::PARAVISOR_MEASURED_VTL2_CONFIG_SIZE_PAGES;
use loader_defs::paravisor::ParavisorMeasuredVtl2Config;
use loader_defs::paravisor::kexec_payload_descriptor_offset;
use object::LittleEndian;
use object::read::elf::FileHeader;
use std::collections::TryReserveError;
use std::ffi::CString;
use std::fs::File;
use std::ops::Range;
use std::os::fd::AsRawFd;
use std::os::unix::fs::FileExt;
use zerocopy::FromBytes;
use zerocopy::Immutable;
use zerocopy::KnownLayout;

const PAGE_SIZE: usize = igvm_defs::PAGE_SIZE_4K as usize;
const CONFIG_SIZE: u64 = PARAVISOR_MEASURED_VTL2_CONFIG_SIZE_PAGES * igvm_defs::PAGE_SIZE_4K;

#[derive(Debug, thiserror::Error)]
enum ExtractError {
    #[error("malformed IGVM: {0}")]
    MalformedIgvm(&'static str),
    #[error(
        "servicing requires a single x64 VSM platform with highest VTL 2 and no shared boundary"
    )]
    UnsupportedPlatform,
    #[error("IGVM or expanded payload exceeds the servicing size limit")]
    TooLarge,
    #[error("invalid IGVM")]
    Igvm(#[from] igvm::Error),
    #[error("failed to allocate servicing payload")]
    Allocation(#[from] TryReserveError),
    #[error("invalid or overflowing GPA range")]
    InvalidRange,
    #[error("duplicate page at GPA {0:#x}")]
    DuplicatePage(u64),
    #[error("missing page at GPA {0:#x}")]
    MissingPage(u64),
    #[error("page at GPA {0:#x} is not normal private measured 4K data")]
    InvalidPage(u64),
    #[error("expected exactly one measured VTL2 config")]
    ConfigCount,
    #[error("invalid measured VTL2 config or product policy length")]
    InvalidConfig,
    #[error("missing or unsupported kexec payload descriptor")]
    InvalidDescriptor,
    #[error("kernel, initrd, and measured config ranges overlap")]
    OverlappingPayloads,
    #[error("invalid ELF kernel")]
    Elf(#[from] object::Error),
    #[error("kernel is not a supported little-endian x64 executable ELF with valid load segments")]
    InvalidKernel,
}

pub(super) fn prepare_kexec(igvm_data: &[u8]) -> anyhow::Result<String> {
    let payload = extract_payload(igvm_data).context("failed to extract servicing IGVM")?;
    let raw_cmdline = std::fs::read_to_string("/proc/cmdline")
        .context("failed to read current kernel command line")?;
    let cmdline = build_kexec_cmdline(&raw_cmdline)?;
    let online_cpus = online_cpu_mask()?;
    let kernel = stage_file(&payload.kernel, "openhcl-kexec-kernel")
        .context("failed to stage servicing kernel")?;
    let initrd = stage_file(&payload.initrd, "openhcl-kexec-initrd")
        .context("failed to stage servicing initrd")?;
    drop(payload);
    kexec_sys::kexec_file_load(
        kernel.as_raw_fd(),
        initrd.as_raw_fd(),
        &cmdline,
        kexec_sys::KEXEC_FILE_FORCE_DTB,
    )
    .context("kexec_file_load failed")?;
    Ok(online_cpus)
}

pub(super) fn online_cpu_mask() -> anyhow::Result<String> {
    Ok(std::fs::read_to_string("/sys/devices/system/cpu/online")
        .context("failed to read online CPU mask")?
        .trim()
        .to_owned())
}

fn build_kexec_cmdline(raw: &str) -> anyhow::Result<CString> {
    let raw = CString::new(raw).context("kernel command line contains a NUL byte")?;
    let words: Vec<_> = raw
        .to_str()?
        .split_whitespace()
        .filter(|word| {
            !word.starts_with("boot_cpus=") && !word.starts_with("OPENHCL_KEXEC_SERVICING=")
        })
        .chain(std::iter::once("OPENHCL_KEXEC_SERVICING=1"))
        .collect();
    CString::new(words.join(" ")).context("invalid kexec command line")
}

fn stage_file(data: &[u8], name: &str) -> anyhow::Result<File> {
    let file: File = sparse_mmap::alloc_shared_memory(data.len(), name)
        .context("failed to create kexec memfd")?
        .into();
    file.write_all_at(data, 0)
        .context("failed to write kexec memfd")?;
    Ok(file)
}

fn read_header<T: FromBytes + Immutable + KnownLayout>(data: &[u8]) -> Result<T, ExtractError> {
    T::read_from_bytes(data).map_err(|_| ExtractError::MalformedIgvm("invalid header length"))
}

fn account_expansion(total: &mut usize, size: usize) -> Result<(), ExtractError> {
    *total = total
        .checked_add(size)
        .filter(|size| *size <= MAX_SERVICING_IGVM_SIZE)
        .ok_or(ExtractError::TooLarge)?;
    Ok(())
}

fn check_file_data(
    data: &[u8],
    data_start: usize,
    file_offset: u32,
    size: usize,
) -> Result<(), ExtractError> {
    if file_offset != 0 {
        let start = file_offset as usize;
        let end = start
            .checked_add(size)
            .ok_or(ExtractError::MalformedIgvm("file data range overflow"))?;
        if start < data_start || end > data.len() {
            return Err(ExtractError::MalformedIgvm(
                "file data outside data section",
            ));
        }
    }
    Ok(())
}

fn checked_end(base: u64, size: u64) -> Result<u64, ExtractError> {
    if size == 0 {
        return Err(ExtractError::InvalidRange);
    }
    base.checked_add(size).ok_or(ExtractError::InvalidRange)
}

fn preflight_igvm(data: &[u8]) -> Result<u32, ExtractError> {
    if data.len() > MAX_SERVICING_IGVM_SIZE {
        return Err(ExtractError::TooLarge);
    }
    let (fixed, _) = IGVM_FIXED_HEADER::read_from_prefix(data)
        .map_err(|_| ExtractError::MalformedIgvm("truncated fixed header"))?;
    if fixed.magic != igvm_defs::IGVM_MAGIC_VALUE || fixed.total_file_size as usize != data.len() {
        return Err(ExtractError::MalformedIgvm("invalid magic or total size"));
    }
    let fixed_size = match fixed.format_version {
        igvm_defs::IGVM_FORMAT_VERSION_1 => size_of::<IGVM_FIXED_HEADER>(),
        igvm_defs::IGVM_FORMAT_VERSION_2 => {
            let (v2, _) = IGVM_FIXED_HEADER_V2::read_from_prefix(data)
                .map_err(|_| ExtractError::MalformedIgvm("truncated v2 fixed header"))?;
            if v2.architecture != IgvmArchitecture::X64 || v2.page_size as usize != PAGE_SIZE {
                return Err(ExtractError::UnsupportedPlatform);
            }
            size_of::<IGVM_FIXED_HEADER_V2>()
        }
        _ => return Err(ExtractError::UnsupportedPlatform),
    };
    let variable_start = fixed.variable_header_offset as usize;
    let data_start = fixed
        .variable_header_offset
        .checked_add(fixed.variable_header_size)
        .ok_or(ExtractError::MalformedIgvm(
            "variable header range overflow",
        ))? as usize;
    if variable_start < fixed_size
        || !variable_start.is_multiple_of(8)
        || data_start >= data.len()
        || fixed.variable_header_size % 8 != 0
    {
        return Err(ExtractError::MalformedIgvm("invalid variable header range"));
    }
    let mut headers = data
        .get(variable_start..data_start)
        .ok_or(ExtractError::MalformedIgvm("invalid variable header range"))?;
    let mut platform_mask = None;
    let mut expanded = 0;
    while !headers.is_empty() {
        let (header, remaining) = IGVM_VHS_VARIABLE_HEADER::read_from_prefix(headers)
            .map_err(|_| ExtractError::MalformedIgvm("truncated variable header"))?;
        let length = header.length as usize;
        let aligned_length = length.checked_add(7).ok_or(ExtractError::MalformedIgvm(
            "variable header length overflow",
        ))? & !7;
        let body = remaining.get(..length).ok_or(ExtractError::MalformedIgvm(
            "truncated variable header body",
        ))?;
        headers = remaining
            .get(aligned_length..)
            .ok_or(ExtractError::MalformedIgvm(
                "truncated variable header padding",
            ))?;
        match header.typ {
            IgvmVariableHeaderType::IGVM_VHT_SUPPORTED_PLATFORM => {
                let platform: IGVM_VHS_SUPPORTED_PLATFORM = read_header(body)?;
                if platform_mask.is_some()
                    || platform.platform_type != IgvmPlatformType::VSM_ISOLATION
                    || platform.highest_vtl != 2
                    || platform.shared_gpa_boundary != 0
                    || platform.platform_version != igvm_defs::IGVM_VSM_ISOLATION_PLATFORM_VERSION
                    || platform.compatibility_mask.count_ones() != 1
                {
                    return Err(ExtractError::UnsupportedPlatform);
                }
                platform_mask = Some(platform.compatibility_mask);
            }
            IgvmVariableHeaderType::IGVM_VHT_PAGE_DATA => {
                let page: IGVM_VHS_PAGE_DATA = read_header(body)?;
                account_expansion(&mut expanded, PAGE_SIZE)?;
                check_file_data(data, data_start, page.file_offset, PAGE_SIZE)?;
            }
            IgvmVariableHeaderType::IGVM_VHT_PARAMETER_AREA => {
                let area: IGVM_VHS_PARAMETER_AREA = read_header(body)?;
                let size =
                    usize::try_from(area.number_of_bytes).map_err(|_| ExtractError::TooLarge)?;
                account_expansion(&mut expanded, size)?;
                check_file_data(data, data_start, area.file_offset, size)?;
            }
            IgvmVariableHeaderType::IGVM_VHT_VP_CONTEXT => {
                let context: IGVM_VHS_VP_CONTEXT = read_header(body)?;
                let start = context.file_offset as usize;
                if start < data_start || start >= data.len() || context.vp_index != 0 {
                    return Err(ExtractError::MalformedIgvm(
                        "invalid VP context offset or index",
                    ));
                }
                let (context_header, registers) =
                    VbsVpContextHeader::read_from_prefix(&data[start..])
                        .map_err(|_| ExtractError::MalformedIgvm("truncated VBS context"))?;
                let register_bytes = (context_header.register_count as usize)
                    .checked_mul(size_of::<VbsVpContextRegister>())
                    .ok_or(ExtractError::TooLarge)?;
                account_expansion(
                    &mut expanded,
                    register_bytes
                        .checked_mul(3)
                        .ok_or(ExtractError::TooLarge)?,
                )?;
                let registers = registers
                    .get(..register_bytes)
                    .ok_or(ExtractError::MalformedIgvm("truncated VBS registers"))?;
                let mut vtl = None;
                for register in registers.chunks_exact(size_of::<VbsVpContextRegister>()) {
                    let register: VbsVpContextRegister = read_header(register)?;
                    if register.vtl > 2 || vtl.is_some_and(|vtl| vtl != register.vtl) {
                        return Err(ExtractError::MalformedIgvm("inconsistent VBS register VTL"));
                    }
                    vtl = Some(register.vtl);
                }
                if vtl.is_none() {
                    return Err(ExtractError::MalformedIgvm("empty VBS context"));
                }
            }
            IgvmVariableHeaderType::IGVM_VHT_RELOCATABLE_REGION => {
                let region: IGVM_VHS_RELOCATABLE_REGION = read_header(body)?;
                if region.relocation_alignment == 0 {
                    return Err(ExtractError::MalformedIgvm("zero relocation alignment"));
                }
                checked_end(region.relocation_region_gpa, region.relocation_region_size)?;
            }
            IgvmVariableHeaderType::IGVM_VHT_PAGE_TABLE_RELOCATION_REGION => {
                let region: IGVM_VHS_PAGE_TABLE_RELOCATION = read_header(body)?;
                checked_end(region.gpa, region.size)?;
            }
            _ => {}
        }
    }
    platform_mask.ok_or(ExtractError::UnsupportedPlatform)
}

struct Page<'a> {
    gpa: u64,
    flags: IgvmPageDataFlags,
    data_type: IgvmPageDataType,
    data: &'a [u8],
}

#[derive(Debug)]
struct Payload {
    kernel: Vec<u8>,
    initrd: Vec<u8>,
}

fn payload_range(base: u64, size: u64) -> Result<Range<u64>, ExtractError> {
    let end = checked_end(base, size)?;
    if !base.is_multiple_of(igvm_defs::PAGE_SIZE_4K) {
        return Err(ExtractError::InvalidRange);
    }
    if size > MAX_SERVICING_IGVM_SIZE as u64 {
        return Err(ExtractError::TooLarge);
    }
    Ok(base..end)
}

fn copy_range(pages: &[Page<'_>], range: &Range<u64>) -> Result<Vec<u8>, ExtractError> {
    let size = usize::try_from(range.end - range.start).map_err(|_| ExtractError::TooLarge)?;
    if size > MAX_SERVICING_IGVM_SIZE {
        return Err(ExtractError::TooLarge);
    }
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size)?;
    bytes.resize(size, 0);
    let first_page = pages
        .binary_search_by_key(&range.start, |page| page.gpa)
        .map_err(|_| ExtractError::MissingPage(range.start))?;
    for (index, chunk) in bytes.chunks_mut(PAGE_SIZE).enumerate() {
        let gpa = range.start + (index * PAGE_SIZE) as u64;
        let page = pages
            .get(first_page + index)
            .filter(|page| page.gpa == gpa)
            .ok_or(ExtractError::MissingPage(gpa))?;
        if page.flags != IgvmPageDataFlags::new() || page.data_type != IgvmPageDataType::NORMAL {
            return Err(ExtractError::InvalidPage(gpa));
        }
        if !page.data.is_empty() {
            let data = page
                .data
                .get(..chunk.len())
                .ok_or(ExtractError::InvalidPage(gpa))?;
            chunk.copy_from_slice(data);
        }
    }
    Ok(bytes)
}

fn overlaps(first: &Range<u64>, second: &Range<u64>) -> bool {
    first.start < second.end && second.start < first.end
}

fn extract_payload(data: &[u8]) -> Result<Payload, ExtractError> {
    let platform_mask = preflight_igvm(data)?;
    let file = IgvmFile::new_from_binary(data, Some(IsolationType::Vbs))?;
    let mut pages = Vec::new();
    let page_count = file.directives().iter().filter(|directive| {
        matches!(directive, IgvmDirectiveHeader::PageData { compatibility_mask, .. } if compatibility_mask & platform_mask != 0)
    }).count();
    pages.try_reserve_exact(page_count)?;
    for directive in file.directives() {
        if let IgvmDirectiveHeader::PageData {
            gpa,
            compatibility_mask,
            flags,
            data_type,
            data,
        } = directive
        {
            if compatibility_mask & platform_mask == 0 {
                continue;
            }
            payload_range(*gpa, igvm_defs::PAGE_SIZE_4K)?;
            if !data.is_empty() && data.len() != PAGE_SIZE {
                return Err(ExtractError::InvalidPage(*gpa));
            }
            pages.push(Page {
                gpa: *gpa,
                flags: *flags,
                data_type: *data_type,
                data,
            });
        }
    }
    pages.sort_unstable_by_key(|page| page.gpa);
    for pair in pages.windows(2) {
        if pair[0].gpa == pair[1].gpa {
            return Err(ExtractError::DuplicatePage(pair[0].gpa));
        }
    }

    let mut config = None;
    for page in &pages {
        let Ok((header, _)) = ParavisorMeasuredVtl2Config::read_from_prefix(page.data) else {
            continue;
        };
        if header.magic != ParavisorMeasuredVtl2Config::MAGIC {
            continue;
        }
        if config.is_some() {
            return Err(ExtractError::ConfigCount);
        }
        if header.vtom_offset_bit != 0 || header.padding != [0; 7] || header.reserved != [0; 4] {
            return Err(ExtractError::InvalidConfig);
        }
        let offset = kexec_payload_descriptor_offset(header.product_policy_size)
            .ok_or(ExtractError::InvalidConfig)?;
        config = Some((payload_range(page.gpa, CONFIG_SIZE)?, offset));
    }
    let (config_range, descriptor_offset) = config.ok_or(ExtractError::ConfigCount)?;
    let config_bytes = copy_range(&pages, &config_range)?;
    let descriptor_data = config_bytes
        .get(descriptor_offset..)
        .ok_or(ExtractError::InvalidDescriptor)?;
    let (descriptor, _) = KexecPayloadDescriptor::read_from_prefix(descriptor_data)
        .map_err(|_| ExtractError::InvalidDescriptor)?;
    if descriptor.magic != KexecPayloadDescriptor::MAGIC
        || descriptor.version != KexecPayloadDescriptor::VERSION
        || descriptor.reserved != 0
    {
        return Err(ExtractError::InvalidDescriptor);
    }
    let kernel_range = payload_range(descriptor.custom_binary_base, descriptor.custom_binary_size)?;
    let initrd_range = payload_range(descriptor.initrd_base, descriptor.initrd_size)?;
    if overlaps(&kernel_range, &initrd_range)
        || overlaps(&kernel_range, &config_range)
        || overlaps(&initrd_range, &config_range)
    {
        return Err(ExtractError::OverlappingPayloads);
    }
    let mut total = config_bytes.len();
    account_expansion(
        &mut total,
        usize::try_from(descriptor.custom_binary_size).map_err(|_| ExtractError::TooLarge)?,
    )?;
    account_expansion(
        &mut total,
        usize::try_from(descriptor.initrd_size).map_err(|_| ExtractError::TooLarge)?,
    )?;
    let kernel = copy_range(&pages, &kernel_range)?;
    validate_kernel(&kernel)?;
    let initrd = copy_range(&pages, &initrd_range)?;
    Ok(Payload { kernel, initrd })
}

fn validate_kernel(data: &[u8]) -> Result<(), ExtractError> {
    let header = object::elf::FileHeader64::<LittleEndian>::parse(data)?;
    if !header.is_supported()
        || !header.is_little_endian()
        || header.e_type.get(LittleEndian) != object::elf::ET_EXEC
        || header.e_machine.get(LittleEndian) != object::elf::EM_X86_64
    {
        return Err(ExtractError::InvalidKernel);
    }
    let mut loadable = false;
    for program in header.program_headers(LittleEndian, data)? {
        if program.p_type.get(LittleEndian) != object::elf::PT_LOAD {
            continue;
        }
        let file_size = program.p_filesz.get(LittleEndian);
        let memory_size = program.p_memsz.get(LittleEndian);
        let end = program
            .p_offset
            .get(LittleEndian)
            .checked_add(file_size)
            .ok_or(ExtractError::InvalidKernel)?;
        if memory_size < file_size
            || end > data.len() as u64
            || program
                .p_paddr
                .get(LittleEndian)
                .checked_add(memory_size)
                .is_none()
            || program
                .p_vaddr
                .get(LittleEndian)
                .checked_add(memory_size)
                .is_none()
        {
            return Err(ExtractError::InvalidKernel);
        }
        loadable |= memory_size != 0;
    }
    if !loadable {
        return Err(ExtractError::InvalidKernel);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use igvm::Arch;
    use igvm::IgvmPlatformHeader;
    use igvm::IgvmRevision;
    use std::io::Read;
    use std::os::unix::fs::MetadataExt;
    use test_with_tracing::test;
    use zerocopy::FromZeros;
    use zerocopy::IntoBytes;

    const CONFIG_GPA: u64 = 0x10000;
    const KERNEL_GPA: u64 = 0x20000;
    const INITRD_GPA: u64 = 0x30000;

    #[test]
    #[ignore = "requires OPENHCL_TEST_SERVICING_IGVM, OPENHCL_TEST_VMLINUX and OPENHCL_TEST_INITRD artifacts"]
    fn servicing_igvm_artifact_matches_inputs() {
        let read_artifact = |variable| {
            std::fs::read(std::env::var_os(variable).expect("artifact path must be set"))
                .expect("artifact must be readable")
        };
        let image = read_artifact("OPENHCL_TEST_SERVICING_IGVM");
        let payload =
            extract_payload(&image).expect("generated IGVM must be accepted for servicing");
        assert_eq!(payload.kernel, read_artifact("OPENHCL_TEST_VMLINUX"));
        assert_eq!(payload.initrd, read_artifact("OPENHCL_TEST_INITRD"));
    }

    struct Fixture {
        revision: IgvmRevision,
        platforms: Vec<IgvmPlatformHeader>,
        policy_size: u32,
        descriptor: KexecPayloadDescriptor,
        kernel: Vec<u8>,
        initrd: Vec<u8>,
    }

    fn kernel_fixture() -> Vec<u8> {
        let mut kernel = vec![0x37; PAGE_SIZE + 123];
        let header = object::elf::FileHeader64 {
            e_ident: object::elf::Ident {
                magic: *b"\x7fELF",
                class: object::elf::ELFCLASS64,
                data: object::elf::ELFDATA2LSB,
                version: object::elf::EV_CURRENT,
                os_abi: object::elf::ELFOSABI_NONE,
                abi_version: 0,
                padding: [0; 7],
            },
            e_type: object::U16::new(LittleEndian, object::elf::ET_EXEC),
            e_machine: object::U16::new(LittleEndian, object::elf::EM_X86_64),
            e_version: object::U32::new(LittleEndian, object::elf::EV_CURRENT.into()),
            e_entry: object::U64::new(LittleEndian, 0x100000),
            e_phoff: object::U64::new(LittleEndian, 64),
            e_shoff: object::U64::new(LittleEndian, 0),
            e_flags: object::U32::new(LittleEndian, 0),
            e_ehsize: object::U16::new(LittleEndian, 64),
            e_phentsize: object::U16::new(LittleEndian, 56),
            e_phnum: object::U16::new(LittleEndian, 1),
            e_shentsize: object::U16::new(LittleEndian, 0),
            e_shnum: object::U16::new(LittleEndian, 0),
            e_shstrndx: object::U16::new(LittleEndian, 0),
        };
        let program = object::elf::ProgramHeader64 {
            p_type: object::U32::new(LittleEndian, object::elf::PT_LOAD),
            p_flags: object::U32::new(LittleEndian, object::elf::PF_R | object::elf::PF_X),
            p_offset: object::U64::new(LittleEndian, 0),
            p_vaddr: object::U64::new(LittleEndian, 0x100000),
            p_paddr: object::U64::new(LittleEndian, 0x100000),
            p_filesz: object::U64::new(LittleEndian, kernel.len() as u64),
            p_memsz: object::U64::new(LittleEndian, kernel.len() as u64 + 4096),
            p_align: object::U64::new(LittleEndian, 4096),
        };
        kernel[..64].copy_from_slice(object::pod::bytes_of(&header));
        kernel[64..120].copy_from_slice(object::pod::bytes_of(&program));
        kernel
    }

    impl Fixture {
        fn new(policy_size: u32) -> Self {
            let kernel = kernel_fixture();
            let mut initrd = vec![0x5a; PAGE_SIZE + 17];
            initrd[PAGE_SIZE..].fill(0);
            Self {
                revision: IgvmRevision::V1,
                platforms: vec![IgvmPlatformHeader::SupportedPlatform(
                    IGVM_VHS_SUPPORTED_PLATFORM {
                        compatibility_mask: 1,
                        highest_vtl: 2,
                        platform_type: IgvmPlatformType::VSM_ISOLATION,
                        platform_version: 1,
                        shared_gpa_boundary: 0,
                    },
                )],
                policy_size,
                descriptor: KexecPayloadDescriptor {
                    magic: KexecPayloadDescriptor::MAGIC,
                    version: KexecPayloadDescriptor::VERSION,
                    reserved: 0,
                    initrd_base: INITRD_GPA,
                    initrd_size: initrd.len() as u64,
                    custom_binary_base: KERNEL_GPA,
                    custom_binary_size: kernel.len() as u64,
                },
                kernel,
                initrd,
            }
        }

        fn directives(&self) -> Vec<IgvmDirectiveHeader> {
            let config = ParavisorMeasuredVtl2Config {
                magic: ParavisorMeasuredVtl2Config::MAGIC,
                vtom_offset_bit: 0,
                padding: [0; 7],
                product_policy_size: self.policy_size,
                reserved: [0; 4],
            };
            let mut config_bytes = vec![0; CONFIG_SIZE as usize];
            config_bytes[..size_of::<ParavisorMeasuredVtl2Config>()]
                .copy_from_slice(config.as_bytes());
            let policy_end = size_of::<ParavisorMeasuredVtl2Config>() + self.policy_size as usize;
            config_bytes[size_of::<ParavisorMeasuredVtl2Config>()..policy_end].fill(0xa5);
            let offset = kexec_payload_descriptor_offset(self.policy_size).unwrap();
            config_bytes[offset..offset + size_of::<KexecPayloadDescriptor>()]
                .copy_from_slice(self.descriptor.as_bytes());
            let mut directives = Vec::new();
            for (base, bytes) in [
                (CONFIG_GPA, config_bytes.as_slice()),
                (KERNEL_GPA, self.kernel.as_slice()),
                (INITRD_GPA, self.initrd.as_slice()),
            ] {
                for (index, chunk) in bytes.chunks(PAGE_SIZE).enumerate() {
                    directives.push(IgvmDirectiveHeader::PageData {
                        gpa: base + (index * PAGE_SIZE) as u64,
                        compatibility_mask: 1,
                        flags: IgvmPageDataFlags::new(),
                        data_type: IgvmPageDataType::NORMAL,
                        data: if chunk.iter().all(|byte| *byte == 0) {
                            Vec::new()
                        } else {
                            chunk.to_vec()
                        },
                    });
                }
            }
            directives
        }

        fn serialize(&self, directives: Vec<IgvmDirectiveHeader>) -> Vec<u8> {
            let file = IgvmFile::new(
                self.revision,
                self.platforms.clone(),
                Vec::new(),
                directives,
            )
            .unwrap();
            let mut data = Vec::new();
            file.serialize(&mut data).unwrap();
            data
        }

        fn image(&self) -> Vec<u8> {
            self.serialize(self.directives())
        }
    }

    fn page_mut(directives: &mut [IgvmDirectiveHeader], address: u64) -> &mut IgvmDirectiveHeader {
        directives
            .iter_mut()
            .find(|directive| matches!(directive, IgvmDirectiveHeader::PageData { gpa, .. } if *gpa == address))
            .unwrap()
    }

    fn mutate_header<T: FromBytes + IntoBytes + Immutable + KnownLayout>(
        data: &mut [u8],
        offset: usize,
        change: impl FnOnce(&mut T),
    ) {
        let bytes = &mut data[offset..offset + size_of::<T>()];
        let mut header = T::read_from_bytes(bytes).unwrap();
        change(&mut header);
        bytes.copy_from_slice(header.as_bytes());
    }

    fn variable_offset(data: &[u8], typ: IgvmVariableHeaderType) -> usize {
        let (fixed, _) = IGVM_FIXED_HEADER::read_from_prefix(data).unwrap();
        let mut offset = fixed.variable_header_offset as usize;
        let end = offset + fixed.variable_header_size as usize;
        while offset < end {
            let (header, _) = IGVM_VHS_VARIABLE_HEADER::read_from_prefix(&data[offset..]).unwrap();
            if header.typ == typ {
                return offset;
            }
            offset += size_of::<IGVM_VHS_VARIABLE_HEADER>() + ((header.length as usize + 7) & !7);
        }
        panic!("missing fixture header {typ:?}")
    }

    #[test]
    fn extracts_exact_payload_and_implicit_zero_page() {
        let fixture = Fixture::new(9);
        let mut directives = fixture.directives();
        directives.reverse();
        let payload = extract_payload(&fixture.serialize(directives)).unwrap();
        assert_eq!(payload.kernel, fixture.kernel);
        assert_eq!(payload.initrd, fixture.initrd);
        assert_eq!(payload.kernel.len(), PAGE_SIZE + 123);
        assert_eq!(payload.initrd.len(), PAGE_SIZE + 17);
    }

    #[test]
    fn extracts_v2_x64() {
        let mut fixture = Fixture::new(0);
        fixture.revision = IgvmRevision::V2 {
            arch: Arch::X64,
            page_size: PAGE_SIZE as u32,
        };
        let payload = extract_payload(&fixture.image()).unwrap();
        assert_eq!(payload.kernel, fixture.kernel);
        assert_eq!(payload.initrd, fixture.initrd);
    }

    #[test]
    fn descriptor_after_large_policy_uses_complete_config_region() {
        let policy_size = CONFIG_SIZE as usize
            - size_of::<ParavisorMeasuredVtl2Config>()
            - size_of::<KexecPayloadDescriptor>();
        let fixture = Fixture::new(policy_size as u32);
        let payload = extract_payload(&fixture.image()).unwrap();
        assert_eq!(payload.kernel, fixture.kernel);
        assert_eq!(payload.initrd, fixture.initrd);
    }

    #[test]
    fn multipage_policy_obeys_descriptor_region_budget() {
        let policy_size = PAGE_SIZE as u32 + 9;
        if kexec_payload_descriptor_offset(policy_size).is_some() {
            let fixture = Fixture::new(policy_size);
            let payload = extract_payload(&fixture.image()).unwrap();
            assert_eq!(payload.kernel, fixture.kernel);
            assert_eq!(payload.initrd, fixture.initrd);
        } else {
            let fixture = Fixture::new(9);
            let mut directives = fixture.directives();
            if let IgvmDirectiveHeader::PageData { data, .. } =
                page_mut(&mut directives, CONFIG_GPA)
            {
                mutate_header::<ParavisorMeasuredVtl2Config>(data, 0, |config| {
                    config.product_policy_size = policy_size;
                });
            }
            assert!(matches!(
                extract_payload(&fixture.serialize(directives)),
                Err(ExtractError::InvalidConfig)
            ));
        }
    }

    #[test]
    fn missing_payload_or_config_page_is_rejected() {
        let fixture = Fixture::new(9);
        for address in [
            CONFIG_GPA,
            KERNEL_GPA,
            KERNEL_GPA + PAGE_SIZE as u64,
            INITRD_GPA + PAGE_SIZE as u64,
        ] {
            let mut directives = fixture.directives();
            directives.retain(|directive| !matches!(directive, IgvmDirectiveHeader::PageData { gpa, .. } if *gpa == address));
            assert!(extract_payload(&fixture.serialize(directives)).is_err());
        }
    }

    #[test]
    fn duplicate_gpa_is_rejected() {
        let fixture = Fixture::new(9);
        let mut directives = fixture.directives();
        directives.push(page_mut(&mut fixture.directives(), KERNEL_GPA).clone());
        assert!(matches!(
            extract_payload(&fixture.serialize(directives)),
            Err(ExtractError::DuplicatePage(KERNEL_GPA))
        ));
    }

    #[test]
    fn incompatible_pages_do_not_supply_or_duplicate_payload() {
        let fixture = Fixture::new(9);
        let mut directives = fixture.directives();
        let mut extra = page_mut(&mut fixture.directives(), CONFIG_GPA).clone();
        if let IgvmDirectiveHeader::PageData {
            compatibility_mask, ..
        } = &mut extra
        {
            *compatibility_mask = 2;
        }
        directives.push(extra);
        assert!(extract_payload(&fixture.serialize(directives)).is_ok());
        let mut directives = fixture.directives();
        if let IgvmDirectiveHeader::PageData {
            compatibility_mask, ..
        } = page_mut(&mut directives, KERNEL_GPA)
        {
            *compatibility_mask = 2;
        }
        assert!(matches!(
            extract_payload(&fixture.serialize(directives)),
            Err(ExtractError::MissingPage(KERNEL_GPA))
        ));
    }

    #[test]
    fn unmeasured_shared_and_special_pages_are_rejected() {
        let fixture = Fixture::new(9);
        for address in [
            CONFIG_GPA,
            KERNEL_GPA,
            INITRD_GPA,
            INITRD_GPA + PAGE_SIZE as u64,
        ] {
            for flags in [
                IgvmPageDataFlags::new().with_unmeasured(true),
                IgvmPageDataFlags::new().with_shared(true),
            ] {
                let mut directives = fixture.directives();
                if let IgvmDirectiveHeader::PageData {
                    flags: page_flags, ..
                } = page_mut(&mut directives, address)
                {
                    *page_flags = flags;
                }
                assert!(matches!(
                    extract_payload(&fixture.serialize(directives)),
                    Err(ExtractError::InvalidPage(_))
                ));
            }
        }
        let mut directives = fixture.directives();
        if let IgvmDirectiveHeader::PageData { data_type, .. } =
            page_mut(&mut directives, INITRD_GPA)
        {
            *data_type = IgvmPageDataType::SECRETS;
        }
        assert!(matches!(
            extract_payload(&fixture.serialize(directives)),
            Err(ExtractError::InvalidPage(INITRD_GPA))
        ));
    }

    #[test]
    fn ambiguous_config_is_rejected() {
        let fixture = Fixture::new(9);
        let mut directives = fixture.directives();
        let mut extra = page_mut(&mut fixture.directives(), CONFIG_GPA).clone();
        if let IgvmDirectiveHeader::PageData { gpa, .. } = &mut extra {
            *gpa = 0x40000;
        }
        directives.push(extra);
        assert!(matches!(
            extract_payload(&fixture.serialize(directives)),
            Err(ExtractError::ConfigCount)
        ));
    }

    #[test]
    fn missing_or_unsupported_descriptor_is_rejected() {
        for variant in 0..3 {
            let mut fixture = Fixture::new(9);
            match variant {
                0 => fixture.descriptor.magic = 0,
                1 => fixture.descriptor.version += 1,
                _ => fixture.descriptor.reserved = 1,
            }
            assert!(matches!(
                extract_payload(&fixture.image()),
                Err(ExtractError::InvalidDescriptor)
            ));
        }
    }

    #[test]
    fn invalid_config_fields_and_policy_length_are_rejected() {
        for variant in 0..4 {
            let fixture = Fixture::new(9);
            let mut directives = fixture.directives();
            if let IgvmDirectiveHeader::PageData { data, .. } =
                page_mut(&mut directives, CONFIG_GPA)
            {
                mutate_header::<ParavisorMeasuredVtl2Config>(data, 0, |config| match variant {
                    0 => config.vtom_offset_bit = 47,
                    1 => config.padding[0] = 1,
                    2 => config.reserved[0] = 1,
                    _ => config.product_policy_size = u32::MAX,
                });
            }
            assert!(matches!(
                extract_payload(&fixture.serialize(directives)),
                Err(ExtractError::InvalidConfig)
            ));
        }
    }

    #[test]
    fn overlapping_ranges_are_rejected() {
        for variant in 0..3 {
            let mut fixture = Fixture::new(9);
            match variant {
                0 => fixture.descriptor.initrd_base = KERNEL_GPA + PAGE_SIZE as u64,
                1 => fixture.descriptor.initrd_base = CONFIG_GPA,
                _ => fixture.descriptor.custom_binary_base = CONFIG_GPA,
            }
            assert!(matches!(
                extract_payload(&fixture.image()),
                Err(ExtractError::OverlappingPayloads)
            ));
        }
    }

    #[test]
    fn invalid_overflowing_and_oversized_payload_ranges_are_rejected() {
        for variant in 0..6 {
            let mut fixture = Fixture::new(9);
            match variant {
                0 => fixture.descriptor.initrd_size = 0,
                1 => fixture.descriptor.custom_binary_size = u64::MAX,
                2 => fixture.descriptor.initrd_base = u64::MAX - 4095,
                3 => fixture.descriptor.custom_binary_base += 1,
                4 => fixture.descriptor.initrd_size = MAX_SERVICING_IGVM_SIZE as u64 + 1,
                _ => {
                    fixture.descriptor.initrd_base = 0x40000000;
                    fixture.descriptor.initrd_size = MAX_SERVICING_IGVM_SIZE as u64;
                }
            }
            assert!(matches!(
                extract_payload(&fixture.image()),
                Err(ExtractError::InvalidRange | ExtractError::TooLarge)
            ));
        }
    }

    #[test]
    fn malformed_or_wrong_elf_is_rejected() {
        for variant in 0..9 {
            let mut fixture = Fixture::new(9);
            match variant {
                0 => fixture.kernel[0] = 0,
                1 => fixture.kernel[5] = object::elf::ELFDATA2MSB,
                2 => fixture.kernel[16..18].copy_from_slice(&object::elf::ET_DYN.to_le_bytes()),
                3 => fixture.kernel[18..20].copy_from_slice(&object::elf::EM_AARCH64.to_le_bytes()),
                4 => fixture.kernel[56..58].copy_from_slice(&0u16.to_le_bytes()),
                5 => fixture.kernel[72..80].copy_from_slice(&u64::MAX.to_le_bytes()),
                6 => fixture.kernel[104..112].copy_from_slice(&1u64.to_le_bytes()),
                7 => fixture.kernel[32..40].copy_from_slice(&u64::MAX.to_le_bytes()),
                _ => fixture.kernel[64..68].copy_from_slice(&object::elf::PT_NOTE.to_le_bytes()),
            }
            assert!(extract_payload(&fixture.image()).is_err());
        }
    }

    #[test]
    fn unsupported_platforms_and_architecture_are_rejected() {
        for variant in 0..5 {
            let mut fixture = Fixture::new(9);
            let IgvmPlatformHeader::SupportedPlatform(platform) = &mut fixture.platforms[0];
            match variant {
                0 => platform.highest_vtl = 0,
                1 => {}
                2 => {
                    platform.platform_type = IgvmPlatformType::NATIVE;
                    platform.highest_vtl = 0;
                }
                3 => {
                    fixture.revision = IgvmRevision::V2 {
                        arch: Arch::AArch64,
                        page_size: PAGE_SIZE as u32,
                    }
                }
                _ => fixture
                    .platforms
                    .push(IgvmPlatformHeader::SupportedPlatform(
                        IGVM_VHS_SUPPORTED_PLATFORM {
                            compatibility_mask: 2,
                            highest_vtl: 0,
                            platform_type: IgvmPlatformType::NATIVE,
                            platform_version: 1,
                            shared_gpa_boundary: 0,
                        },
                    )),
            }
            let mut data = fixture.image();
            if variant == 1 {
                let offset =
                    variable_offset(&data, IgvmVariableHeaderType::IGVM_VHT_SUPPORTED_PLATFORM) + 8;
                mutate_header::<IGVM_VHS_SUPPORTED_PLATFORM>(&mut data, offset, |platform| {
                    platform.shared_gpa_boundary = 1 << 47;
                });
            }
            assert!(matches!(
                extract_payload(&data),
                Err(ExtractError::UnsupportedPlatform)
            ));
        }
    }

    #[test]
    fn malformed_fixed_and_variable_headers_are_rejected_before_parser() {
        let fixture = Fixture::new(9);
        for variant in 0..7 {
            let mut data = fixture.image();
            mutate_header::<IGVM_FIXED_HEADER>(&mut data, 0, |header| match variant {
                0 => header.magic = 0,
                1 => header.total_file_size += 1,
                2 => header.variable_header_offset = u32::MAX - 7,
                3 => header.variable_header_size = u32::MAX,
                4 => header.variable_header_offset = 8,
                5 => header.variable_header_offset += 1,
                _ => header.variable_header_size -= 1,
            });
            assert!(matches!(
                preflight_igvm(&data),
                Err(ExtractError::MalformedIgvm(_))
            ));
        }
        for length in [0, 1, u32::MAX] {
            let mut data = fixture.image();
            let offset = variable_offset(&data, IgvmVariableHeaderType::IGVM_VHT_PAGE_DATA);
            mutate_header::<IGVM_VHS_VARIABLE_HEADER>(&mut data, offset, |header| {
                header.length = length
            });
            assert!(matches!(
                preflight_igvm(&data),
                Err(ExtractError::MalformedIgvm(_))
            ));
        }
        assert!(preflight_igvm(&[]).is_err());
        assert!(preflight_igvm(&fixture.image()[..20]).is_err());
    }

    #[test]
    fn invalid_v2_page_size_is_rejected_before_parser() {
        let mut fixture = Fixture::new(9);
        fixture.revision = IgvmRevision::V2 {
            arch: Arch::X64,
            page_size: PAGE_SIZE as u32,
        };
        let mut data = fixture.image();
        mutate_header::<IGVM_FIXED_HEADER_V2>(&mut data, 0, |header| header.page_size = 0x200000);
        assert!(matches!(
            preflight_igvm(&data),
            Err(ExtractError::UnsupportedPlatform)
        ));
    }

    #[test]
    fn invalid_page_offsets_are_rejected_even_when_filtered() {
        let fixture = Fixture::new(9);
        for variant in 0..3 {
            let mut data = fixture.image();
            let (fixed, _) = IGVM_FIXED_HEADER::read_from_prefix(&data).unwrap();
            let offset = variable_offset(&data, IgvmVariableHeaderType::IGVM_VHT_PAGE_DATA) + 8;
            let invalid_offset = match variant {
                0 => fixed.variable_header_offset + fixed.variable_header_size - 1,
                1 => data.len() as u32 - PAGE_SIZE as u32 + 1,
                _ => u32::MAX,
            };
            mutate_header::<IGVM_VHS_PAGE_DATA>(&mut data, offset, |page| {
                page.file_offset = invalid_offset;
                page.compatibility_mask = 2;
            });
            assert!(matches!(
                extract_payload(&data),
                Err(ExtractError::MalformedIgvm(_))
            ));
        }
    }

    #[test]
    fn parameter_area_offsets_and_expansion_are_bounded() {
        let fixture = Fixture::new(9);
        let mut directives = fixture.directives();
        directives.push(IgvmDirectiveHeader::ParameterArea {
            number_of_bytes: PAGE_SIZE as u64,
            parameter_area_index: 0,
            initial_data: vec![1; PAGE_SIZE],
        });
        for variant in 0..3 {
            let mut data = fixture.serialize(directives.clone());
            let offset =
                variable_offset(&data, IgvmVariableHeaderType::IGVM_VHT_PARAMETER_AREA) + 8;
            mutate_header::<IGVM_VHS_PARAMETER_AREA>(&mut data, offset, |area| match variant {
                0 => area.file_offset = 1,
                1 => area.number_of_bytes = u64::MAX,
                _ => {
                    area.file_offset = 0;
                    area.number_of_bytes = MAX_SERVICING_IGVM_SIZE as u64;
                }
            });
            assert!(preflight_igvm(&data).is_err());
        }
    }

    fn context_fixture() -> Vec<u8> {
        let fixture = Fixture::new(9);
        let mut directives = fixture.directives();
        directives.push(IgvmDirectiveHeader::X64VbsVpContext {
            vtl: igvm::hv_defs::Vtl::Vtl2,
            registers: vec![
                igvm::registers::X86Register::Rip(0x100000),
                igvm::registers::X86Register::Rip(0),
            ],
            compatibility_mask: 1,
        });
        fixture.serialize(directives)
    }

    fn raw_preflight_fixture(headers: &[(IgvmVariableHeaderType, &[u8])], data: &[u8]) -> Vec<u8> {
        let platform = IGVM_VHS_SUPPORTED_PLATFORM {
            compatibility_mask: 1,
            highest_vtl: 2,
            platform_type: IgvmPlatformType::VSM_ISOLATION,
            platform_version: 1,
            shared_gpa_boundary: 0,
        };
        let mut variable_headers = Vec::new();
        for (typ, body) in std::iter::once((
            IgvmVariableHeaderType::IGVM_VHT_SUPPORTED_PLATFORM,
            platform.as_bytes(),
        ))
        .chain(headers.iter().copied())
        {
            variable_headers.extend_from_slice(
                IGVM_VHS_VARIABLE_HEADER {
                    typ,
                    length: body.len() as u32,
                }
                .as_bytes(),
            );
            variable_headers.extend_from_slice(body);
            variable_headers.resize((variable_headers.len() + 7) & !7, 0);
        }
        let fixed = IGVM_FIXED_HEADER {
            magic: igvm_defs::IGVM_MAGIC_VALUE,
            format_version: igvm_defs::IGVM_FORMAT_VERSION_1,
            variable_header_offset: size_of::<IGVM_FIXED_HEADER>() as u32,
            variable_header_size: variable_headers.len() as u32,
            total_file_size: (size_of::<IGVM_FIXED_HEADER>() + variable_headers.len() + data.len())
                as u32,
            checksum: 0,
        };
        let mut image = fixed.as_bytes().to_vec();
        image.extend_from_slice(&variable_headers);
        image.extend_from_slice(data);
        image
    }

    #[test]
    fn relocation_arithmetic_is_preflighted() {
        for variant in 0..3 {
            let mut region = IGVM_VHS_RELOCATABLE_REGION::new_zeroed();
            region.compatibility_mask = 1;
            region.relocation_alignment = igvm_defs::PAGE_SIZE_4K;
            region.relocation_region_size = igvm_defs::PAGE_SIZE_4K;
            match variant {
                0 => region.relocation_alignment = 0,
                1 => region.relocation_region_size = 0,
                _ => region.relocation_region_gpa = u64::MAX - 4095,
            }
            let data = raw_preflight_fixture(
                &[(
                    IgvmVariableHeaderType::IGVM_VHT_RELOCATABLE_REGION,
                    region.as_bytes(),
                )],
                &[0],
            );
            assert!(preflight_igvm(&data).is_err());
        }
        for size in [0, igvm_defs::PAGE_SIZE_4K] {
            let mut region = IGVM_VHS_PAGE_TABLE_RELOCATION::new_zeroed();
            region.compatibility_mask = 1;
            region.gpa = u64::MAX - 4095;
            region.size = size;
            let data = raw_preflight_fixture(
                &[(
                    IgvmVariableHeaderType::IGVM_VHT_PAGE_TABLE_RELOCATION_REGION,
                    region.as_bytes(),
                )],
                &[0],
            );
            assert!(matches!(
                preflight_igvm(&data),
                Err(ExtractError::InvalidRange)
            ));
        }
    }

    #[test]
    fn repeated_context_expansion_is_bounded() {
        let register_count = 4096;
        let context_count =
            MAX_SERVICING_IGVM_SIZE / (register_count * size_of::<VbsVpContextRegister>()) + 1;
        let mut context = IGVM_VHS_VP_CONTEXT::new_zeroed();
        context.compatibility_mask = 1;
        let context_header_size =
            size_of::<IGVM_VHS_VARIABLE_HEADER>() + ((size_of::<IGVM_VHS_VP_CONTEXT>() + 7) & !7);
        context.file_offset = (size_of::<IGVM_FIXED_HEADER>()
            + size_of::<IGVM_VHS_VARIABLE_HEADER>()
            + size_of::<IGVM_VHS_SUPPORTED_PLATFORM>()
            + context_count * context_header_size) as u32;
        let headers = vec![
            (
                IgvmVariableHeaderType::IGVM_VHT_VP_CONTEXT,
                context.as_bytes()
            );
            context_count
        ];
        let mut registers = VbsVpContextHeader {
            register_count: register_count as u32,
        }
        .as_bytes()
        .to_vec();
        registers.resize(
            registers.len() + register_count * size_of::<VbsVpContextRegister>(),
            0,
        );
        let data = raw_preflight_fixture(&headers, &registers);
        assert!(data.len() < MAX_SERVICING_IGVM_SIZE);
        assert!(matches!(preflight_igvm(&data), Err(ExtractError::TooLarge)));
    }

    #[test]
    fn vbs_context_assertions_offsets_and_expansion_are_preflighted() {
        let original = context_fixture();
        assert!(extract_payload(&original).is_ok());
        for variant in 0..5 {
            let mut data = original.clone();
            let offset = variable_offset(&data, IgvmVariableHeaderType::IGVM_VHT_VP_CONTEXT) + 8;
            let (context, _) = IGVM_VHS_VP_CONTEXT::read_from_prefix(&data[offset..]).unwrap();
            let context_start = context.file_offset as usize;
            match variant {
                0 | 1 => {
                    let invalid_offset = if variant == 0 { 1 } else { data.len() as u32 };
                    mutate_header::<IGVM_VHS_VP_CONTEXT>(&mut data, offset, |context| {
                        context.file_offset = invalid_offset;
                        context.compatibility_mask = 2;
                    });
                }
                2 => mutate_header::<IGVM_VHS_VP_CONTEXT>(&mut data, offset, |context| {
                    context.vp_index = 1
                }),
                3 => mutate_header::<VbsVpContextHeader>(&mut data, context_start, |context| {
                    context.register_count = u32::MAX
                }),
                _ => {
                    let register_offset = context_start
                        + size_of::<VbsVpContextHeader>()
                        + size_of::<VbsVpContextRegister>();
                    mutate_header::<VbsVpContextRegister>(&mut data, register_offset, |register| {
                        register.vtl = 0
                    });
                }
            }
            assert!(preflight_igvm(&data).is_err());
        }
    }

    #[test]
    fn repeated_page_expansion_is_bounded_including_zero_pages() {
        let fixture = Fixture::new(9);
        let count = MAX_SERVICING_IGVM_SIZE / PAGE_SIZE + 1;
        for zero in [false, true] {
            let page = IgvmDirectiveHeader::PageData {
                gpa: 0,
                compatibility_mask: 2,
                flags: IgvmPageDataFlags::new(),
                data_type: IgvmPageDataType::NORMAL,
                data: if zero { Vec::new() } else { vec![1; PAGE_SIZE] },
            };
            let mut directives = fixture.directives();
            directives.extend(std::iter::repeat_n(page, count));
            let data = fixture.serialize(directives);
            assert!(data.len() < MAX_SERVICING_IGVM_SIZE);
            assert!(matches!(preflight_igvm(&data), Err(ExtractError::TooLarge)));
        }
    }

    #[test]
    fn oversized_input_is_rejected() {
        let data = vec![0; MAX_SERVICING_IGVM_SIZE + 1];
        assert!(matches!(
            extract_payload(&data),
            Err(ExtractError::TooLarge)
        ));
    }

    #[test]
    fn staged_file_is_memfd_with_exact_payload() {
        let mut file = stage_file(b"servicing payload", "openhcl-kexec-test").unwrap();
        let metadata = file.metadata().unwrap();
        assert_eq!(metadata.nlink(), 0);
        assert_eq!(metadata.len(), b"servicing payload".len() as u64);
        assert!(metadata.is_file());
        let target = std::fs::read_link(format!("/proc/self/fd/{}", file.as_raw_fd())).unwrap();
        assert_eq!(
            target.to_str().unwrap(),
            "/memfd:openhcl-kexec-test (deleted)"
        );
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        assert_eq!(bytes, b"servicing payload");
    }

    #[test]
    fn staged_memfds_with_the_same_name_are_independent() {
        let mut kernel = stage_file(b"kernel bytes", "openhcl-kexec-test").unwrap();
        let mut initrd = stage_file(b"initrd bytes", "openhcl-kexec-test").unwrap();
        assert_ne!(
            kernel.metadata().unwrap().ino(),
            initrd.metadata().unwrap().ino()
        );
        let mut bytes = Vec::new();
        kernel.read_to_end(&mut bytes).unwrap();
        assert_eq!(bytes, b"kernel bytes");
        bytes.clear();
        initrd.read_to_end(&mut bytes).unwrap();
        assert_eq!(bytes, b"initrd bytes");
    }

    #[test]
    fn staged_memfd_rejects_invalid_name() {
        assert!(stage_file(b"payload", "invalid\0name").is_err());
    }

    #[test]
    fn cmdline_removes_all_stale_options_and_adds_one_marker() {
        let cmdline = build_kexec_cmdline("console=ttyS0 boot_cpus=1 OPENHCL_KEXEC_SERVICING=0\nroot=/dev/ram0 boot_cpus=2 OPENHCL_KEXEC_SERVICING=1").unwrap();
        assert_eq!(
            cmdline.to_str().unwrap(),
            "console=ttyS0 root=/dev/ram0 OPENHCL_KEXEC_SERVICING=1"
        );
        assert_eq!(
            build_kexec_cmdline(" \n").unwrap().to_str().unwrap(),
            "OPENHCL_KEXEC_SERVICING=1"
        );
    }

    #[test]
    fn cmdline_rejects_nul_even_in_removed_options() {
        for raw in [
            "console=ttyS0\0",
            "boot_cpus=1\0",
            "OPENHCL_KEXEC_SERVICING=0\0",
        ] {
            assert!(build_kexec_cmdline(raw).is_err());
        }
    }
}
