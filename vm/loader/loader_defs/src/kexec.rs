// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! The x64 OpenHCL composite-ELF layout and bootstrap-private handoff.

use core::ops::Range;
use zerocopy::FromBytes;
use zerocopy::Immutable;
use zerocopy::IntoBytes;
use zerocopy::KnownLayout;

/// Alignment of metadata and scratch pages in the composite image.
pub const PAGE_ALIGNMENT: u64 = 4096;

/// Identifies a bootstrap-private physical handoff.
pub const BOOT_ENTRY_MAGIC: u64 = u64::from_le_bytes(*b"OHCLKE01");

/// Validate the bootstrap handoff and enter Linux without starting sidecar VPs.
pub const BOOT_FLAG_SIDECAR_DISABLED: u64 = 1;

/// Alignment required by the existing x64 ELF kexec loader.
pub const COMPOSITE_ALIGNMENT: u64 = 2 * 1024 * 1024;
/// Selects the composite entry, whose RSI argument is Linux's boot parameters.
pub const COMPOSITE_ENTRY_MAGIC: u64 = u64::from_le_bytes(*b"OHCLCE01");
/// ELF note type advertising the bootstrap's composite entry.
pub const COMPOSITE_NOTE_TYPE: u32 = 0x4f48434c;
/// ELF note owner, including the terminating NUL.
pub const COMPOSITE_NOTE_NAME: &[u8] = b"OPENHCL\0";

/// Relative component layout at the first loaded byte of a composite ELF.
///
/// The first 2 MiB hold this record and bootstrap scratch. The bootstrap is
/// always at offset 2 MiB, allowing it to find this record without kernel help.
#[repr(C)]
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct CompositeHeader {
    /// Must be [`Self::MAGIC`].
    pub magic: u64,
    /// Must be [`Self::VERSION`].
    pub version: u32,
    /// Exact header size.
    pub header_size: u32,
    /// Linux image offset and memory size, including BSS.
    pub kernel: PhysicalRange,
    /// Linux entry offset from the start of the composite image.
    pub kernel_entry: u64,
    /// Sidecar image offset and memory size, including BSS.
    pub sidecar: PhysicalRange,
    /// Sidecar entry offset from the start of the composite image.
    pub sidecar_entry: u64,
    /// Bootstrap image offset and memory size, including BSS.
    pub bootstrap: PhysicalRange,
    /// Total loaded image size.
    pub image_size: u64,
    /// Must be zero.
    pub reserved: [u64; 2],
}

impl CompositeHeader {
    /// Identifies relative layout data, never a kernel file format.
    pub const MAGIC: u64 = u64::from_le_bytes(*b"OHCLCM01");
    /// Layout version.
    pub const VERSION: u32 = 1;

    /// Lay out nonempty components without overflowing the relative address space.
    pub fn new(
        kernel_size: u64,
        kernel_entry: u64,
        sidecar_size: u64,
        sidecar_entry: u64,
        bootstrap_size: u64,
    ) -> Option<Self> {
        if kernel_size == 0
            || sidecar_size == 0
            || bootstrap_size == 0
            || kernel_entry >= kernel_size
            || sidecar_entry >= sidecar_size
            || sidecar_size > COMPOSITE_ALIGNMENT
        {
            return None;
        }
        let align = |value: u64| {
            Some(value.checked_add(COMPOSITE_ALIGNMENT - 1)? & !(COMPOSITE_ALIGNMENT - 1))
        };
        let bootstrap = PhysicalRange {
            base: COMPOSITE_ALIGNMENT,
            size: bootstrap_size,
        };
        let sidecar = PhysicalRange {
            base: align(bootstrap.base.checked_add(bootstrap.size)?)?,
            size: sidecar_size,
        };
        let kernel = PhysicalRange {
            base: align(sidecar.base.checked_add(sidecar.size)?)?,
            size: kernel_size,
        };
        Some(Self {
            magic: Self::MAGIC,
            version: Self::VERSION,
            header_size: size_of::<Self>() as u32,
            kernel_entry: kernel.base.checked_add(kernel_entry)?,
            sidecar_entry: sidecar.base.checked_add(sidecar_entry)?,
            image_size: kernel.base.checked_add(kernel.size)?,
            kernel,
            sidecar,
            bootstrap,
            reserved: [0; 2],
        })
    }

    /// Relocate the canonical layout and validate the resulting physical handoff.
    pub fn relocate(&self, base: u64, boot_params: u64) -> Option<BootHandoff> {
        let expected = Self::new(
            self.kernel.size,
            self.kernel_entry.checked_sub(self.kernel.base)?,
            self.sidecar.size,
            self.sidecar_entry.checked_sub(self.sidecar.base)?,
            self.bootstrap.size,
        )?;
        if self.as_bytes() != expected.as_bytes()
            || base == 0
            || !base.is_multiple_of(COMPOSITE_ALIGNMENT)
            || base.checked_add(self.image_size)? > 1 << 47
            || (boot_params < base.checked_add(self.image_size)?
                && base < boot_params.checked_add(PAGE_ALIGNMENT)?)
        {
            return None;
        }
        let physical = |range: PhysicalRange| {
            Some(PhysicalRange {
                base: base.checked_add(range.base)?,
                size: range.size,
            })
        };
        let handoff = BootHandoff {
            magic: BOOT_ENTRY_MAGIC,
            version: BootHandoff::VERSION,
            header_size: size_of::<BootHandoff>() as u32,
            kernel: physical(self.kernel)?,
            kernel_entry: base.checked_add(self.kernel_entry)?,
            sidecar: physical(self.sidecar)?,
            sidecar_entry: base.checked_add(self.sidecar_entry)?,
            bootstrap: physical(self.bootstrap)?,
            boot_params,
            sidecar_params: base.checked_add(2 * PAGE_ALIGNMENT)?,
            sidecar_output: base.checked_add(3 * PAGE_ALIGNMENT)?,
            runtime: PhysicalRange {
                base: base.checked_add(4 * PAGE_ALIGNMENT)?,
                size: PAGE_ALIGNMENT,
            },
            flags: BOOT_FLAG_SIDECAR_DISABLED,
            reserved: 0,
        };
        handoff
            .validate(base.checked_add(PAGE_ALIGNMENT)?)
            .then_some(handoff)
    }
}

/// A physical range which must be identity mapped by the kexec loader.
#[repr(C)]
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct PhysicalRange {
    /// Physical base address.
    pub base: u64,
    /// Length in bytes, including BSS for executable images.
    pub size: u64,
}

impl PhysicalRange {
    /// Validate a nonempty, page-aligned positive-canonical identity mapping.
    pub fn checked_range(&self) -> Option<Range<u64>> {
        let end = self.base.checked_add(self.size)?;
        (self.base != 0
            && self.base.is_multiple_of(PAGE_ALIGNMENT)
            && self.size != 0
            && end <= 1 << 47)
            .then_some(self.base..end)
    }
}

/// Physical handoff constructed by the composite bootstrap, not by Linux.
///
/// The loader must reserve every listed allocation, supply fresh DT/E820 and
/// CPU ownership, and quiesce the predecessor before entering the bootstrap.
/// Range validation does not establish that those lifecycle requirements hold.
#[repr(C)]
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct BootHandoff {
    /// Must be [`BOOT_ENTRY_MAGIC`].
    pub magic: u64,
    /// Must be [`Self::VERSION`].
    pub version: u32,
    /// Exact size of this handoff record.
    pub header_size: u32,
    /// Loaded Linux image allocation.
    pub kernel: PhysicalRange,
    /// Relocated physical Linux entry point.
    pub kernel_entry: u64,
    /// Loaded sidecar image allocation, including BSS.
    pub sidecar: PhysicalRange,
    /// Relocated physical sidecar entry point.
    pub sidecar_entry: u64,
    /// Loaded boot-shim allocation, including its stack and BSS.
    pub bootstrap: PhysicalRange,
    /// Linux boot parameters in a separate, at least 16-byte-aligned allocation.
    pub boot_params: u64,
    /// Reserved page containing `sidecar_defs::SidecarParams`.
    pub sidecar_params: u64,
    /// Reserved page for `sidecar_defs::SidecarOutput`.
    pub sidecar_output: u64,
    /// Arena reserved for the new sidecar's node allocations.
    pub runtime: PhysicalRange,
    /// Combination of the defined `BOOT_FLAG_*` values.
    pub flags: u64,
    /// Must be zero.
    pub reserved: u64,
}

impl BootHandoff {
    /// Physical handoff layout version.
    pub const VERSION: u32 = 1;

    /// Check the fixed ABI, entry points and all allocation boundaries.
    pub fn validate(&self, handoff_address: u64) -> bool {
        if self.magic != BOOT_ENTRY_MAGIC
            || self.version != Self::VERSION
            || self.header_size as usize != size_of::<Self>()
            || self.reserved != 0
            || self.flags & !BOOT_FLAG_SIDECAR_DISABLED != 0
            || !self.sidecar.base.is_multiple_of(2 * 1024 * 1024)
            || self.sidecar.size > 2 * 1024 * 1024
        {
            return false;
        }
        let page = |base| PhysicalRange {
            base,
            size: PAGE_ALIGNMENT,
        };
        let Some(params_end) = self.boot_params.checked_add(PAGE_ALIGNMENT) else {
            return false;
        };
        if self.boot_params == 0 || !self.boot_params.is_multiple_of(16) || params_end > 1 << 47 {
            return false;
        }
        let allocations = [
            self.kernel,
            self.sidecar,
            self.bootstrap,
            self.runtime,
            page(self.sidecar_params),
            page(self.sidecar_output),
            page(handoff_address),
        ];
        for (index, allocation) in allocations.iter().enumerate() {
            let Some(range) = allocation.checked_range() else {
                return false;
            };
            if range.start < params_end && self.boot_params < range.end {
                return false;
            }
            for earlier in &allocations[..index] {
                let Some(other) = earlier.checked_range() else {
                    return false;
                };
                if range.start < other.end && other.start < range.end {
                    return false;
                }
            }
        }
        self.kernel
            .checked_range()
            .is_some_and(|range| range.contains(&self.kernel_entry))
            && self
                .sidecar
                .checked_range()
                .is_some_and(|range| range.contains(&self.sidecar_entry))
    }
}

/// Offset of the optional [`SidecarPlan`] from the start of a composite image.
pub const SIDECAR_PLAN_OFFSET: u64 = 5 * PAGE_ALIGNMENT;
/// Offset of zeroed pages the bootstrap may use to extend the kexec page tables.
pub const PAGE_TABLE_SCRATCH_OFFSET: u64 = 8 * PAGE_ALIGNMENT;
/// Number of page-table scratch pages.
pub const PAGE_TABLE_SCRATCH_PAGES: u64 = 8;
/// Maximum number of sidecar nodes described by a [`SidecarPlan`].
pub const MAX_SIDECAR_PLAN_CHUNKS: usize = 128;
/// Bytes at the start of a kexec sidecar region for the sidecar image and record.
pub const SIDECAR_REGION_IMAGE_SIZE: u64 = 512 * 1024;
/// Sidecar maps its image with one large page, which must stay inside the region.
pub const SIDECAR_REGION_MIN_SIZE: u64 = 2 * 1024 * 1024;
/// Enable sidecar serial logging.
pub const SIDECAR_PLAN_FLAG_LOGGING: u32 = 1;
/// Root device-tree node describing the reserved kexec sidecar regions.
pub const SIDECAR_DT_NODE_NAME: &str = "openhcl-kexec-sidecar";
/// `u32` VP count property.
pub const SIDECAR_DT_CPU_COUNT: &str = "cpu-count";
/// `u64` image area size property.
pub const SIDECAR_DT_IMAGE_SIZE: &str = "image-size";
/// `u64` base/size pairs of the two regions.
pub const SIDECAR_DT_REGIONS: &str = "regions";
/// `u64` base VP, VP count, arena offset and arena size per sidecar node.
pub const SIDECAR_DT_CHUNKS: &str = "chunks";

/// One sidecar node inside a reserved kexec sidecar region.
#[repr(C)]
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct SidecarPlanChunk {
    /// First VP of the node; Linux always owns it.
    pub base_vp: u32,
    /// Number of VPs in the node.
    pub vp_count: u32,
    /// Node memory offset from the region base.
    pub arena_offset: u64,
    /// Node memory size.
    pub arena_size: u64,
    /// Offset of the node's four-cell `reg` value in the carried-over device
    /// tree, or zero if the device tree has no node to retarget.
    pub dt_reg_offset: u32,
    /// Must be zero.
    pub reserved: u32,
}

/// Fresh sidecar assignment staged before the VM stops.
///
/// CPUs that must stay with Linux are only final after save, so the bootstrap
/// reads them from a [`SidecarRecord`] at `region_base + record_offset`.
#[repr(C)]
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct SidecarPlan {
    /// Must be [`Self::MAGIC`].
    pub magic: u64,
    /// Must be [`Self::VERSION`].
    pub version: u32,
    /// Combination of `SIDECAR_PLAN_FLAG_*` values.
    pub flags: u32,
    /// Reserved region not used by the running sidecar.
    pub region_base: u64,
    /// Size of the reserved region.
    pub region_size: u64,
    /// Bytes at the region start for the sidecar image and the record.
    pub image_size: u64,
    /// Offset of the [`SidecarRecord`] page from the region base.
    pub record_offset: u64,
    /// Number of VPs in the partition.
    pub cpu_count: u32,
    /// Number of valid `chunks`.
    pub chunk_count: u32,
    /// Width of the space-padded `boot_cpus=` value in the staged command line.
    pub boot_cpus_width: u32,
    /// Must be zero.
    pub reserved: u32,
    /// Sidecar nodes, in device-tree order.
    pub chunks: [SidecarPlanChunk; MAX_SIDECAR_PLAN_CHUNKS],
}

impl SidecarPlan {
    /// Identifies a sidecar plan.
    pub const MAGIC: u64 = u64::from_le_bytes(*b"OHCLSP01");
    /// Plan layout version.
    pub const VERSION: u32 = 1;

    /// Returns the valid chunks.
    pub fn chunks(&self) -> &[SidecarPlanChunk] {
        &self.chunks[..(self.chunk_count as usize).min(MAX_SIDECAR_PLAN_CHUNKS)]
    }

    /// Validate invariants that do not depend on the incoming sidecar.
    pub fn validate(&self) -> bool {
        let page = PAGE_ALIGNMENT;
        let Some(region_end) = self.region_base.checked_add(self.region_size) else {
            return false;
        };
        if self.magic != Self::MAGIC
            || self.version != Self::VERSION
            || self.flags & !SIDECAR_PLAN_FLAG_LOGGING != 0
            || self.reserved != 0
            || self.region_base == 0
            || !self.region_base.is_multiple_of(COMPOSITE_ALIGNMENT)
            || self.region_size < SIDECAR_REGION_MIN_SIZE
            || !self.region_size.is_multiple_of(page)
            || region_end > 1 << 47
            || self.image_size < 2 * page
            || !self.image_size.is_multiple_of(page)
            || self.image_size > self.region_size
            || !self.record_offset.is_multiple_of(page)
            || self
                .record_offset
                .checked_add(page)
                .is_none_or(|end| end > self.image_size)
            || self.chunk_count as usize > MAX_SIDECAR_PLAN_CHUNKS
            || self.cpu_count == 0
            || self.cpu_count > SidecarRecord::MAX_CPUS
            || self.boot_cpus_width == 0
        {
            return false;
        }
        let mut next_cpu = 0;
        let mut next_arena = self.image_size;
        for chunk in self.chunks() {
            let (Some(cpu_end), Some(arena_end)) = (
                chunk.base_vp.checked_add(chunk.vp_count),
                chunk.arena_offset.checked_add(chunk.arena_size),
            ) else {
                return false;
            };
            if chunk.vp_count < 2
                || chunk.base_vp < next_cpu
                || cpu_end > self.cpu_count
                || chunk.arena_offset < next_arena
                || !chunk.arena_offset.is_multiple_of(page)
                || chunk.arena_size == 0
                || !chunk.arena_size.is_multiple_of(page)
                || arena_end > self.region_size
                || chunk.reserved != 0
            {
                return false;
            }
            next_cpu = cpu_end;
            next_arena = arena_end;
        }
        true
    }
}

/// CPUs that must start in Linux, written by the predecessor after save.
#[repr(C)]
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct SidecarRecord {
    /// Must be [`Self::MAGIC`].
    pub magic: u64,
    /// Must be [`Self::VERSION`].
    pub version: u32,
    /// Number of VPs in the partition.
    pub cpu_count: u32,
    /// Bitmap of CPUs with restored device interrupts.
    pub kernel_cpus: [u64; 64],
}

impl SidecarRecord {
    /// Identifies a sidecar record.
    pub const MAGIC: u64 = u64::from_le_bytes(*b"OHCLSR01");
    /// Record layout version.
    pub const VERSION: u32 = 1;
    /// Largest supported VP count.
    pub const MAX_CPUS: u32 = 64 * 64;

    /// Create an empty record.
    pub fn new(cpu_count: u32) -> Self {
        Self {
            magic: Self::MAGIC,
            version: Self::VERSION,
            cpu_count,
            kernel_cpus: [0; 64],
        }
    }

    /// Require `cpu` to start in Linux. Returns false if it is out of range.
    pub fn set_kernel_cpu(&mut self, cpu: u32) -> bool {
        if cpu >= self.cpu_count || cpu >= Self::MAX_CPUS {
            return false;
        }
        self.kernel_cpus[cpu as usize / 64] |= 1 << (cpu % 64);
        true
    }

    /// Returns whether `cpu` must start in Linux.
    pub fn kernel_cpu(&self, cpu: u32) -> bool {
        cpu < Self::MAX_CPUS && self.kernel_cpus[cpu as usize / 64] & (1 << (cpu % 64)) != 0
    }

    /// Returns whether any CPU must start in Linux.
    pub fn any_kernel_cpu(&self) -> bool {
        self.kernel_cpus.iter().any(|&bits| bits != 0)
    }

    /// Check the header against the plan's VP count.
    pub fn validate(&self, cpu_count: u32) -> bool {
        self.magic == Self::MAGIC
            && self.version == Self::VERSION
            && self.cpu_count == cpu_count
            && cpu_count <= Self::MAX_CPUS
            && (cpu_count..Self::MAX_CPUS).all(|cpu| !self.kernel_cpu(cpu))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_with_tracing::test;
    use zerocopy::FromZeros;

    #[test]
    fn composite_layout_relocates_above_four_gib() {
        let layout = CompositeHeader::new(0x1200000, 0x100, 0x20000, 0x4000, 0x100000).unwrap();
        assert_eq!(size_of::<CompositeHeader>(), 104);
        for base in [0x200000, 0x400000000] {
            let handoff = layout.relocate(base, 0x10000).unwrap();
            assert_eq!(handoff.bootstrap.base, base + COMPOSITE_ALIGNMENT);
            assert_eq!(handoff.kernel_entry, base + layout.kernel_entry);
            assert_eq!(handoff.sidecar_entry, base + layout.sidecar_entry);
            assert!(handoff.validate(base + PAGE_ALIGNMENT));
        }
        assert!(layout.relocate(0, 0x10000).is_none());
        assert!(layout.relocate(0x201000, 0x10000).is_none());
        assert!(
            layout
                .relocate((1 << 47) - COMPOSITE_ALIGNMENT, 0x10000)
                .is_none()
        );
        assert!(layout.relocate(0x200000, 0x201000).is_none());
    }

    #[test]
    fn composite_rejects_invalid_layout() {
        let valid = CompositeHeader::new(0x1000000, 0x100, 0x20000, 0x4000, 0x100000).unwrap();
        for variant in 0..7 {
            let mut header = valid;
            match variant {
                0 => header.magic = 0,
                1 => header.version += 1,
                2 => header.reserved[0] = 1,
                3 => header.sidecar.base = header.bootstrap.base,
                4 => header.bootstrap.base += 0x1000,
                5 => header.image_size += 1,
                _ => header.kernel_entry = header.kernel.base - 1,
            }
            assert!(header.relocate(0x200000, 0x10000).is_none());
        }
        assert!(CompositeHeader::new(0, 0, 1, 0, 1).is_none());
        assert!(CompositeHeader::new(1, 1, 1, 0, 1).is_none());
        assert!(CompositeHeader::new(u64::MAX, 0, 1, 0, 1).is_none());
        assert!(CompositeHeader::new(1, 0, COMPOSITE_ALIGNMENT + 1, 0, 1).is_none());
    }

    #[test]
    fn boot_handoff_layout_and_bounds() {
        assert_eq!(size_of::<BootHandoff>(), 136);
        assert_eq!(core::mem::offset_of!(BootHandoff, kernel), 16);
        assert_eq!(core::mem::offset_of!(BootHandoff, sidecar), 40);
        assert_eq!(core::mem::offset_of!(BootHandoff, bootstrap), 64);
        let valid = BootHandoff {
            magic: BOOT_ENTRY_MAGIC,
            version: BootHandoff::VERSION,
            header_size: size_of::<BootHandoff>() as u32,
            kernel: PhysicalRange {
                base: 0x200000,
                size: 0x200000,
            },
            kernel_entry: 0x200000,
            sidecar: PhysicalRange {
                base: 0x400000,
                size: 0x200000,
            },
            sidecar_entry: 0x400000,
            bootstrap: PhysicalRange {
                base: 0x600000,
                size: 0x200000,
            },
            runtime: PhysicalRange {
                base: 0x800000,
                size: 0x200000,
            },
            boot_params: 0x10000,
            sidecar_params: 0x11000,
            sidecar_output: 0x12000,
            flags: BOOT_FLAG_SIDECAR_DISABLED,
            reserved: 0,
        };
        assert!(valid.validate(0x13000));
        for variant in 0..12 {
            let mut handoff = valid;
            match variant {
                0 => handoff.magic = 0,
                1 => handoff.version += 1,
                2 => handoff.header_size -= 1,
                3 => handoff.reserved = 1,
                4 => handoff.kernel_entry = 0,
                5 => handoff.sidecar_entry = handoff.sidecar.base + handoff.sidecar.size,
                6 => handoff.sidecar.base += PAGE_ALIGNMENT,
                7 => handoff.sidecar.size += 1,
                8 => handoff.sidecar_output = handoff.sidecar_params,
                9 => handoff.runtime.base = handoff.sidecar.base,
                10 => handoff.kernel.size = u64::MAX,
                _ => handoff.bootstrap.base = 1 << 47,
            }
            assert!(!handoff.validate(0x13000), "variant {variant}");
        }
        assert!(!valid.validate(valid.sidecar_params));
        assert!(!valid.validate(0));
        let unaligned_params = BootHandoff {
            boot_params: 0x18010,
            ..valid
        };
        assert!(unaligned_params.validate(0x13000));
    }

    fn plan_fixture() -> SidecarPlan {
        let mut plan = SidecarPlan::new_zeroed();
        plan.magic = SidecarPlan::MAGIC;
        plan.version = SidecarPlan::VERSION;
        plan.region_base = 0x4000_0000;
        plan.region_size = 0x40_0000;
        plan.image_size = SIDECAR_REGION_IMAGE_SIZE;
        plan.record_offset = SIDECAR_REGION_IMAGE_SIZE - PAGE_ALIGNMENT;
        plan.cpu_count = 64;
        plan.chunk_count = 2;
        plan.boot_cpus_width = 64;
        for (index, chunk) in plan.chunks[..2].iter_mut().enumerate() {
            *chunk = SidecarPlanChunk {
                base_vp: index as u32 * 32,
                vp_count: 32,
                arena_offset: SIDECAR_REGION_IMAGE_SIZE + index as u64 * 0x10_0000,
                arena_size: 0x10_0000,
                dt_reg_offset: 0x100 + index as u32 * 0x40,
                reserved: 0,
            };
        }
        plan
    }

    #[test]
    fn sidecar_plan_layout_and_bounds() {
        assert_eq!(size_of::<SidecarPlanChunk>(), 32);
        assert_eq!(size_of::<SidecarPlan>(), 64 + 32 * MAX_SIDECAR_PLAN_CHUNKS);
        const {
            assert!(
                SIDECAR_PLAN_OFFSET + size_of::<SidecarPlan>() as u64 <= PAGE_TABLE_SCRATCH_OFFSET
            );
            assert!(
                PAGE_TABLE_SCRATCH_OFFSET + PAGE_TABLE_SCRATCH_PAGES * PAGE_ALIGNMENT
                    <= COMPOSITE_ALIGNMENT
            );
        }
        assert!(plan_fixture().validate());
        for variant in 0..14 {
            let mut plan = plan_fixture();
            match variant {
                0 => plan.magic = 0,
                1 => plan.version += 1,
                2 => plan.flags = 2,
                3 => plan.region_base += PAGE_ALIGNMENT,
                4 => plan.region_size = SIDECAR_REGION_MIN_SIZE - PAGE_ALIGNMENT,
                5 => plan.record_offset = plan.image_size,
                6 => plan.chunk_count = MAX_SIDECAR_PLAN_CHUNKS as u32 + 1,
                7 => plan.cpu_count = 63,
                8 => plan.chunks[0].vp_count = 1,
                9 => plan.chunks[1].base_vp = 16,
                10 => plan.chunks[0].arena_offset = 0,
                11 => plan.chunks[1].arena_offset = plan.chunks[0].arena_offset,
                12 => plan.chunks[1].arena_size = plan.region_size,
                _ => plan.boot_cpus_width = 0,
            }
            assert!(!plan.validate(), "variant {variant}");
        }
    }

    #[test]
    fn sidecar_record_bits() {
        let mut record = SidecarRecord::new(130);
        assert!(record.validate(130));
        assert!(!record.any_kernel_cpu());
        assert!(record.set_kernel_cpu(0));
        assert!(record.set_kernel_cpu(129));
        assert!(!record.set_kernel_cpu(130));
        assert!(record.kernel_cpu(0) && record.kernel_cpu(129) && !record.kernel_cpu(64));
        assert!(record.any_kernel_cpu());
        assert!(!record.validate(129));
        record.kernel_cpus[63] = 1 << 63;
        assert!(!record.validate(130));
    }
}
