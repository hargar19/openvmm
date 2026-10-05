// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Compose a normal ELF for the existing x64 kexec loader.

use anyhow::Context;
use anyhow::ensure;
use loader_defs::kexec::COMPOSITE_ALIGNMENT;
use loader_defs::kexec::COMPOSITE_ENTRY_MAGIC;
use loader_defs::kexec::COMPOSITE_NOTE_NAME;
use loader_defs::kexec::COMPOSITE_NOTE_TYPE;
use loader_defs::kexec::CompositeHeader;
use loader_defs::kexec::SIDECAR_PLAN_OFFSET;
use loader_defs::kexec::SidecarPlan;
use object::LittleEndian as LE;
use object::elf;
use object::read::elf::FileHeader;
use object::read::elf::NoteIterator;
use std::fs::File;
use std::os::unix::fs::FileExt;
use zerocopy::IntoBytes;

const ALIGN: u64 = COMPOSITE_ALIGNMENT;

struct Segment<'a> {
    address: u64,
    virtual_address: u64,
    memory_size: u64,
    bytes: &'a [u8],
    executable: bool,
}

struct Image<'a> {
    segments: Vec<Segment<'a>>,
    lowest: u64,
    size: u64,
    entry: u64,
}

fn file_range(data: &[u8], offset: u64, size: u64) -> anyhow::Result<&[u8]> {
    let end = offset
        .checked_add(size)
        .context("ELF file range overflow")?;
    data.get(usize::try_from(offset)?..usize::try_from(end)?)
        .context("ELF file range out of bounds")
}

impl<'a> Image<'a> {
    fn parse(data: &'a [u8], companion: bool, composite_entry: bool) -> anyhow::Result<Self> {
        let header = elf::FileHeader64::<LE>::parse(data)?;
        ensure!(
            header.is_supported()
                && header.is_little_endian()
                && header.e_machine.get(LE) == elf::EM_X86_64
                && (header.e_type.get(LE) == elf::ET_EXEC
                    || (companion && header.e_type.get(LE) == elf::ET_DYN)),
            "unsupported composite component ELF"
        );
        let programs = header.program_headers(LE, data)?;
        ensure!(programs.len() <= 128, "too many ELF program headers");
        let mut segments: Vec<Segment<'_>> = Vec::new();
        let mut lowest = u64::MAX;
        let mut highest = 0;
        for program in programs {
            if program.p_type.get(LE) != elf::PT_LOAD {
                continue;
            }
            let address = program.p_paddr.get(LE);
            let virtual_address = program.p_vaddr.get(LE);
            let memory_size = program.p_memsz.get(LE);
            let file_size = program.p_filesz.get(LE);
            let alignment = program.p_align.get(LE);
            let end = address
                .checked_add(memory_size)
                .context("ELF memory range overflow")?;
            virtual_address
                .checked_add(memory_size)
                .context("ELF virtual range overflow")?;
            ensure!(
                memory_size != 0 && file_size <= memory_size,
                "invalid ELF segment size"
            );
            ensure!(
                alignment <= ALIGN
                    && (alignment <= 1
                        || (alignment.is_power_of_two()
                            && virtual_address % alignment
                                == program.p_offset.get(LE) % alignment)),
                "invalid ELF segment alignment"
            );
            ensure!(
                !companion || address == virtual_address,
                "companion requires identity-linked segments"
            );
            ensure!(
                segments
                    .iter()
                    .all(|other| address >= other.address + other.memory_size
                        || other.address >= end),
                "overlapping ELF segments"
            );
            segments.push(Segment {
                address,
                virtual_address,
                memory_size,
                bytes: file_range(data, program.p_offset.get(LE), file_size)?,
                executable: program.p_flags.get(LE) & elf::PF_X != 0,
            });
            lowest = lowest.min(address);
            highest = highest.max(end);
        }
        ensure!(
            !segments.is_empty() && (!companion || lowest == 0),
            "invalid ELF load base"
        );
        let size = highest
            .checked_sub(lowest)
            .and_then(|size| size.checked_add(4095))
            .map(|size| size & !4095)
            .context("ELF span overflow")?;
        ensure!(
            size <= super::MAX_SERVICING_IGVM_SIZE as u64,
            "ELF memory span exceeds servicing limit"
        );
        let requested_entry = if composite_entry {
            find_composite_entry(data, header)?
        } else {
            header.e_entry.get(LE)
        };
        let entry = segments
            .iter()
            .find_map(|segment| {
                if companion && !segment.executable {
                    return None;
                }
                [segment.virtual_address, segment.address]
                    .into_iter()
                    .find_map(|base| {
                        requested_entry
                            .checked_sub(base)
                            .filter(|offset| *offset < segment.bytes.len() as u64)
                            .map(|offset| segment.address - lowest + offset)
                    })
            })
            .context("ELF entry is not in file-backed executable code")?;
        Ok(Self {
            segments,
            lowest,
            size,
            entry,
        })
    }

    fn write(&self, file: &File, base: u64) -> anyhow::Result<()> {
        for segment in &self.segments {
            file.write_all_at(segment.bytes, ALIGN + base + segment.address - self.lowest)?;
        }
        Ok(())
    }
}

fn find_composite_entry(data: &[u8], header: &elf::FileHeader64<LE>) -> anyhow::Result<u64> {
    let mut entry = None;
    for program in header.program_headers(LE, data)? {
        if program.p_type.get(LE) != elf::PT_NOTE {
            continue;
        }
        let bytes = file_range(data, program.p_offset.get(LE), program.p_filesz.get(LE))?;
        let mut notes =
            NoteIterator::<elf::FileHeader64<LE>>::new(LE, program.p_align.get(LE), bytes)?;
        while let Some(note) = notes.next()? {
            if note.name_bytes() != COMPOSITE_NOTE_NAME || note.n_type(LE) != COMPOSITE_NOTE_TYPE {
                continue;
            }
            ensure!(
                entry.is_none() && note.desc().len() == 16,
                "invalid or duplicate composite entry note"
            );
            let magic = u64::from_le_bytes(note.desc()[..8].try_into()?);
            ensure!(
                magic == COMPOSITE_ENTRY_MAGIC,
                "unsupported composite entry note"
            );
            let delta = i64::from_le_bytes(note.desc()[8..].try_into()?);
            let descriptor_offset = (note.desc().as_ptr() as usize)
                .checked_sub(bytes.as_ptr() as usize)
                .context("invalid note offset")? as u64;
            entry = Some(
                program
                    .p_vaddr
                    .get(LE)
                    .checked_add(descriptor_offset)
                    .and_then(|address| address.checked_add(8))
                    .and_then(|address| address.checked_add_signed(delta))
                    .context("composite entry overflow")?,
            );
        }
    }
    entry.context("bootstrap lacks composite entry note; rebuild the incoming IGVM")
}

pub(super) fn stage(
    kernel: &[u8],
    sidecar: &[u8],
    bootstrap: &[u8],
    plan: Option<&SidecarPlan>,
) -> anyhow::Result<File> {
    let kernel = Image::parse(kernel, false, false).context("invalid Linux ELF")?;
    let sidecar = Image::parse(sidecar, true, false).context("invalid sidecar ELF")?;
    let bootstrap = Image::parse(bootstrap, true, true).context("invalid bootstrap ELF")?;
    ensure!(
        bootstrap.size <= 16 * 1024 * 1024,
        "bootstrap image too large"
    );
    let layout = CompositeHeader::new(
        kernel.size,
        kernel.entry,
        sidecar.size,
        sidecar.entry,
        bootstrap.size,
    )
    .context("invalid composite layout")?;
    let file_size = ALIGN
        .checked_add(layout.image_size)
        .context("composite file size overflow")?;
    ensure!(
        file_size <= super::MAX_SERVICING_IGVM_SIZE as u64,
        "composite ELF exceeds servicing limit"
    );
    let file: File =
        sparse_mmap::alloc_shared_memory(file_size as usize, "openhcl-kexec-elf")?.into();
    let header = elf::FileHeader64 {
        e_ident: elf::Ident {
            magic: elf::ELFMAG,
            class: elf::ELFCLASS64,
            data: elf::ELFDATA2LSB,
            version: elf::EV_CURRENT,
            os_abi: 0,
            abi_version: 0,
            padding: [0; 7],
        },
        e_type: object::U16::new(LE, elf::ET_EXEC),
        e_machine: object::U16::new(LE, elf::EM_X86_64),
        e_version: object::U32::new(LE, elf::EV_CURRENT.into()),
        e_entry: object::U64::new(LE, layout.bootstrap.base + bootstrap.entry),
        e_phoff: object::U64::new(LE, 64),
        e_shoff: object::U64::new(LE, 0),
        e_flags: object::U32::new(LE, 0),
        e_ehsize: object::U16::new(LE, 64),
        e_phentsize: object::U16::new(LE, 56),
        e_phnum: object::U16::new(LE, 4),
        e_shentsize: object::U16::new(LE, 0),
        e_shnum: object::U16::new(LE, 0),
        e_shstrndx: object::U16::new(LE, 0),
    };
    file.write_all_at(object::pod::bytes_of(&header), 0)?;
    for (index, (base, size, flags)) in [
        (0, ALIGN, elf::PF_R | elf::PF_W),
        (
            layout.bootstrap.base,
            bootstrap.size,
            elf::PF_R | elf::PF_W | elf::PF_X,
        ),
        (
            layout.sidecar.base,
            sidecar.size,
            elf::PF_R | elf::PF_W | elf::PF_X,
        ),
        (
            layout.kernel.base,
            kernel.size,
            elf::PF_R | elf::PF_W | elf::PF_X,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let program = elf::ProgramHeader64 {
            p_type: object::U32::new(LE, elf::PT_LOAD),
            p_flags: object::U32::new(LE, flags),
            p_offset: object::U64::new(LE, ALIGN + base),
            p_vaddr: object::U64::new(LE, base),
            p_paddr: object::U64::new(LE, base),
            p_filesz: object::U64::new(LE, size),
            p_memsz: object::U64::new(LE, size),
            p_align: object::U64::new(LE, ALIGN),
        };
        file.write_all_at(object::pod::bytes_of(&program), 64 + index as u64 * 56)?;
    }
    file.write_all_at(layout.as_bytes(), ALIGN)?;
    if let Some(plan) = plan {
        ensure!(plan.validate(), "invalid sidecar plan");
        file.write_all_at(plan.as_bytes(), ALIGN + SIDECAR_PLAN_OFFSET)?;
    }
    bootstrap.write(&file, layout.bootstrap.base)?;
    sidecar.write(&file, layout.sidecar.base)?;
    kernel.write(&file, layout.kernel.base)?;
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::super::tests::kernel_fixture;
    use super::*;
    use std::io::Read;
    use test_with_tracing::test;
    use zerocopy::FromBytes;

    fn companion(note: bool) -> Vec<u8> {
        let mut bytes = kernel_fixture();
        bytes[16..18].copy_from_slice(&elf::ET_DYN.to_le_bytes());
        bytes[24..32].copy_from_slice(&0x100u64.to_le_bytes());
        bytes[80..88].copy_from_slice(&0u64.to_le_bytes());
        bytes[88..96].copy_from_slice(&0u64.to_le_bytes());
        if note {
            bytes[56..58].copy_from_slice(&2u16.to_le_bytes());
            let program = elf::ProgramHeader64 {
                p_type: object::U32::new(LE, elf::PT_NOTE),
                p_flags: object::U32::new(LE, elf::PF_R),
                p_offset: object::U64::new(LE, 0x200),
                p_vaddr: object::U64::new(LE, 0x200),
                p_paddr: object::U64::new(LE, 0x200),
                p_filesz: object::U64::new(LE, 36),
                p_memsz: object::U64::new(LE, 36),
                p_align: object::U64::new(LE, 4),
            };
            bytes[120..176].copy_from_slice(object::pod::bytes_of(&program));
            bytes[0x200..0x204].copy_from_slice(&8u32.to_le_bytes());
            bytes[0x204..0x208].copy_from_slice(&16u32.to_le_bytes());
            bytes[0x208..0x20c].copy_from_slice(&COMPOSITE_NOTE_TYPE.to_le_bytes());
            bytes[0x20c..0x214].copy_from_slice(COMPOSITE_NOTE_NAME);
            bytes[0x214..0x21c].copy_from_slice(&COMPOSITE_ENTRY_MAGIC.to_le_bytes());
            bytes[0x21c..0x224].copy_from_slice(&(0x100i64 - 0x21c).to_le_bytes());
        }
        bytes
    }

    #[test]
    fn composite_matches_existing_loader_and_relocated_handoff() {
        let kernel = kernel_fixture();
        let sidecar = companion(false);
        let bootstrap = companion(true);
        let mut file = stage(&kernel, &sidecar, &bootstrap, None).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let header = elf::FileHeader64::<LE>::parse(bytes.as_slice()).unwrap();
        assert_eq!(header.e_type.get(LE), elf::ET_EXEC);
        let mut loaded = vec![0u8; bytes.len() - ALIGN as usize];
        for program in header.program_headers(LE, bytes.as_slice()).unwrap() {
            assert_eq!(program.p_type.get(LE), elf::PT_LOAD);
            assert_eq!(program.p_paddr.get(LE) % ALIGN, 0);
            assert_eq!(program.p_align.get(LE), ALIGN);
            let source =
                file_range(&bytes, program.p_offset.get(LE), program.p_filesz.get(LE)).unwrap();
            let start = program.p_paddr.get(LE) as usize;
            loaded[start..start + source.len()].copy_from_slice(source);
        }
        let (layout, _) = CompositeHeader::read_from_prefix(&loaded).unwrap();
        assert_eq!(header.e_entry.get(LE), layout.bootstrap.base + 0x100);
        for (range, input) in [
            (layout.kernel, &kernel),
            (layout.sidecar, &sidecar),
            (layout.bootstrap, &bootstrap),
        ] {
            let start = range.base as usize;
            assert_eq!(&loaded[start..start + input.len()], input.as_slice());
            assert!(
                loaded[start + input.len()..start + range.size as usize]
                    .iter()
                    .all(|&byte| byte == 0)
            );
        }
        assert!(
            loaded[size_of::<CompositeHeader>()..ALIGN as usize]
                .iter()
                .all(|&byte| byte == 0)
        );
        assert_eq!(
            layout.relocate(0x400000000, 0x10010).unwrap().boot_params,
            0x10010
        );
    }

    #[test]
    fn composite_rejects_missing_or_invalid_entry_note() {
        let kernel = kernel_fixture();
        let sidecar = companion(false);
        assert!(stage(&kernel, &sidecar, &sidecar, None).is_err());
        for variant in 0..4 {
            let mut bootstrap = companion(true);
            match variant {
                0 => bootstrap[0x214] = 0,
                1 => bootstrap[0x21c..0x224].copy_from_slice(&i64::MIN.to_le_bytes()),
                2 => bootstrap[0x21c..0x224].copy_from_slice(&0x10000000i64.to_le_bytes()),
                _ => bootstrap[0x204..0x208].copy_from_slice(&0xffffu32.to_le_bytes()),
            }
            assert!(stage(&kernel, &sidecar, &bootstrap, None).is_err());
        }
    }

    #[test]
    fn composite_rejects_invalid_memory_layout() {
        for variant in 0..5 {
            let mut input = companion(false);
            match variant {
                0 => input[88..96].copy_from_slice(&1u64.to_le_bytes()),
                1 => input[104..112].copy_from_slice(&u64::MAX.to_le_bytes()),
                2 => input[112..120].copy_from_slice(&3u64.to_le_bytes()),
                3 => input[24..32].copy_from_slice(&0x10000000u64.to_le_bytes()),
                _ => input[104..112].copy_from_slice(
                    &(super::super::MAX_SERVICING_IGVM_SIZE as u64 + 1).to_le_bytes(),
                ),
            }
            assert!(Image::parse(&input, true, false).is_err());
        }
    }

    #[test]
    fn composite_preserves_unaligned_segments_and_bss() {
        let mut sidecar = companion(false);
        sidecar.resize(0x2000, 0);
        sidecar[56..58].copy_from_slice(&2u16.to_le_bytes());
        sidecar[96..104].copy_from_slice(&0x1000u64.to_le_bytes());
        sidecar[104..112].copy_from_slice(&0x1000u64.to_le_bytes());
        let segment = elf::ProgramHeader64 {
            p_type: object::U32::new(LE, elf::PT_LOAD),
            p_flags: object::U32::new(LE, elf::PF_R | elf::PF_W),
            p_offset: object::U64::new(LE, 0x1123),
            p_vaddr: object::U64::new(LE, 0x2123),
            p_paddr: object::U64::new(LE, 0x2123),
            p_filesz: object::U64::new(LE, 0x20),
            p_memsz: object::U64::new(LE, 0x40),
            p_align: object::U64::new(LE, 4096),
        };
        sidecar[120..176].copy_from_slice(object::pod::bytes_of(&segment));
        sidecar[0x1123..0x1143].fill(0xa5);
        let file = stage(&kernel_fixture(), &sidecar, &companion(true), None).unwrap();
        let mut header_bytes = [0; size_of::<CompositeHeader>()];
        file.read_exact_at(&mut header_bytes, ALIGN).unwrap();
        let layout = CompositeHeader::read_from_bytes(&header_bytes).unwrap();
        let mut bytes = vec![0; layout.sidecar.size as usize];
        file.read_exact_at(&mut bytes, ALIGN + layout.sidecar.base)
            .unwrap();
        assert!(bytes[0x1000..0x2123].iter().all(|&byte| byte == 0));
        assert_eq!(&bytes[0x2123..0x2143], &[0xa5; 0x20]);
        assert!(bytes[0x2143..].iter().all(|&byte| byte == 0));
        sidecar[128..136].copy_from_slice(&0x1124u64.to_le_bytes());
        assert!(stage(&kernel_fixture(), &sidecar, &companion(true), None).is_err());
    }

    #[test]
    fn composite_accepts_linux_init_entry_without_execute_flag() {
        let mut kernel = kernel_fixture();
        kernel[68..72].copy_from_slice(&(elf::PF_R | elf::PF_W).to_le_bytes());
        assert!(stage(&kernel, &companion(false), &companion(true), None).is_ok());
    }

    #[test]
    fn composite_carries_validated_sidecar_plan() {
        use loader_defs::kexec::SIDECAR_REGION_IMAGE_SIZE;
        use zerocopy::FromZeros;

        let mut plan = SidecarPlan::new_zeroed();
        plan.magic = SidecarPlan::MAGIC;
        plan.version = SidecarPlan::VERSION;
        plan.region_base = 0x4000_0000;
        plan.region_size = 0x20_0000;
        plan.image_size = SIDECAR_REGION_IMAGE_SIZE;
        plan.record_offset = SIDECAR_REGION_IMAGE_SIZE - 4096;
        plan.cpu_count = 4;
        plan.boot_cpus_width = 64;
        let parts = (kernel_fixture(), companion(false), companion(true));
        let file = stage(&parts.0, &parts.1, &parts.2, Some(&plan)).unwrap();
        let mut bytes = vec![0; size_of::<SidecarPlan>()];
        file.read_exact_at(&mut bytes, ALIGN + SIDECAR_PLAN_OFFSET)
            .unwrap();
        assert_eq!(bytes, plan.as_bytes());
        plan.magic = 0;
        assert!(stage(&parts.0, &parts.1, &parts.2, Some(&plan)).is_err());
    }
}
