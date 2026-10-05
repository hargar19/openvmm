// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Dedicated x64 kexec bootstrap. This does not replay host boot setup.

use arrayvec::ArrayString;
use core::fmt::Write;
use loader_defs::kexec::BootHandoff;
use loader_defs::kexec::MAX_SIDECAR_PLAN_CHUNKS;
use loader_defs::kexec::SIDECAR_PLAN_FLAG_LOGGING;
use loader_defs::kexec::SidecarPlan;
use loader_defs::kexec::SidecarPlanChunk;
use loader_defs::kexec::SidecarRecord;
use sidecar_defs::NUM_CPUS_SUPPORTED_FOR_PER_CPU_STATE;
use sidecar_defs::SidecarNodeParams;
use sidecar_defs::SidecarParams;

const BOOT_CPUS: &[u8] = b"boot_cpus=";
const MAX_BOOT_CPUS_WIDTH: usize = 1024;
const FDT_MAGIC: u32 = 0xd00dfeed;
const FDT_PROP: u32 = 3;
const REG_CELLS: usize = 4;

/// Assign fresh sidecar nodes from the staged plan and the post-save record.
///
/// Chunks whose non-base CPUs all need Linux, or that this sidecar cannot fit,
/// stay with Linux. Without a valid record no chunk starts.
fn assign_sidecar(
    plan: &SidecarPlan,
    record: Option<&SidecarRecord>,
    params: &mut SidecarParams,
) -> [bool; MAX_SIDECAR_PLAN_CHUNKS] {
    let mut started = [false; MAX_SIDECAR_PLAN_CHUNKS];
    params.node_count = 0;
    params.enable_logging = plan.flags & SIDECAR_PLAN_FLAG_LOGGING != 0;
    params.initial_state.per_cpu_state_specified = false;
    params.initial_state.sidecar_starts_cpu.fill(true);
    let Some(record) = record.filter(|record| record.validate(plan.cpu_count)) else {
        return started;
    };
    if record.any_kernel_cpu() {
        if plan.cpu_count as usize > NUM_CPUS_SUPPORTED_FOR_PER_CPU_STATE {
            return started;
        }
        params.initial_state.per_cpu_state_specified = true;
        for (cpu, starts) in params.initial_state.sidecar_starts_cpu[..plan.cpu_count as usize]
            .iter_mut()
            .enumerate()
        {
            *starts = !record.kernel_cpu(cpu as u32);
        }
    }
    for (started, chunk) in started.iter_mut().zip(plan.chunks()) {
        let fits = chunk.dt_reg_offset != 0
            && chunk.vp_count <= 32
            && sidecar_defs::required_memory(chunk.vp_count) as u64 <= chunk.arena_size;
        let has_sidecar_cpu =
            (1..chunk.vp_count).any(|index| !record.kernel_cpu(chunk.base_vp + index));
        if !fits || !has_sidecar_cpu {
            continue;
        }
        params.nodes[params.node_count as usize] = SidecarNodeParams {
            memory_base: plan.region_base + chunk.arena_offset,
            memory_size: chunk.arena_size,
            base_vp: chunk.base_vp,
            vp_count: chunk.vp_count,
        };
        params.node_count += 1;
        if params.initial_state.per_cpu_state_specified {
            params.initial_state.sidecar_starts_cpu[chunk.base_vp as usize] = false;
        }
        *started = true;
    }
    started
}

/// Format the CPUs Linux must start as a range list.
fn boot_cpus(
    plan: &SidecarPlan,
    params: &SidecarParams,
) -> Option<ArrayString<MAX_BOOT_CPUS_WIDTH>> {
    let sidecar_cpu = |cpu: u32| {
        params.nodes[..params.node_count as usize]
            .iter()
            .any(|node| {
                cpu > node.base_vp
                    && cpu < node.base_vp + node.vp_count
                    && (!params.initial_state.per_cpu_state_specified
                        || params.initial_state.sidecar_starts_cpu[cpu as usize])
            })
    };
    let mut list = ArrayString::new();
    let mut cpu = 0;
    let mut separator = "";
    while cpu < plan.cpu_count {
        if sidecar_cpu(cpu) {
            cpu += 1;
            continue;
        }
        let start = cpu;
        while cpu + 1 < plan.cpu_count && !sidecar_cpu(cpu + 1) {
            cpu += 1;
        }
        let result = if start == cpu {
            write!(list, "{separator}{start}")
        } else {
            write!(list, "{separator}{start}-{cpu}")
        };
        result.ok()?;
        separator = ",";
        cpu += 1;
    }
    (list.len() <= plan.boot_cpus_width as usize).then_some(list)
}

/// Overwrite the space-padded `boot_cpus=` value staged by the predecessor.
fn patch_boot_cpus(cmdline: &mut [u8], value: &str, width: usize) -> bool {
    let len = cmdline
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(cmdline.len());
    let cmdline = &mut cmdline[..len];
    let Some(start) = (0..len).find(|&index| {
        cmdline[index..].starts_with(BOOT_CPUS) && (index == 0 || cmdline[index - 1] == b' ')
    }) else {
        return false;
    };
    let start = start + BOOT_CPUS.len();
    let Some(field) = start
        .checked_add(width)
        .filter(|&end| end <= len && (end == len || cmdline[end] == b' '))
        .map(|end| &mut cmdline[start..end])
    else {
        return false;
    };
    let current = field.iter().position(|&b| b == b' ').unwrap_or(width);
    if value.len() > width || field[current..].iter().any(|&b| b != b' ') {
        return false;
    }
    field.fill(b' ');
    field[..value.len()].copy_from_slice(value.as_bytes());
    true
}

/// Returns the byte range of a four-cell `reg` value in a flattened device tree.
fn fdt_reg_value(fdt: &[u8], offset: u32) -> Option<core::ops::Range<usize>> {
    let be = |at: usize| Some(u32::from_be_bytes(fdt.get(at..at + 4)?.try_into().ok()?));
    if be(0)? != FDT_MAGIC {
        return None;
    }
    let total = (be(4)? as usize).min(fdt.len());
    let struct_start = be(8)? as usize;
    let struct_end = struct_start.checked_add(be(36)? as usize)?.min(total);
    let value = offset as usize;
    let end = value.checked_add(REG_CELLS * 8)?;
    if value < struct_start + 12 || end > struct_end || !value.is_multiple_of(4) {
        return None;
    }
    (be(value - 12)? == FDT_PROP && be(value - 8)? as usize == REG_CELLS * 8).then_some(value..end)
}

/// Device tree `reg` cells for a node whose control page is at `arena`.
fn chunk_reg(plan: &SidecarPlan, chunk: &SidecarPlanChunk) -> [u64; REG_CELLS] {
    let control = plan.region_base + chunk.arena_offset;
    let page = sidecar_defs::PAGE_SIZE as u64;
    [
        control,
        page,
        control + page,
        chunk.vp_count as u64 * sidecar_defs::PER_VP_SHMEM_PAGES as u64 * page,
    ]
}

fn write_reg(fdt: &mut [u8], value: core::ops::Range<usize>, cells: [u64; REG_CELLS]) {
    for (bytes, cell) in fdt[value].chunks_exact_mut(8).zip(cells) {
        bytes.copy_from_slice(&cell.to_be_bytes());
    }
}

#[derive(Debug, PartialEq, Eq)]
enum InvalidParams {
    NodeCount,
    CpuRange,
    MemoryRange,
    KernelBaseCpu,
    EmptyNode,
}

fn validate_sidecar_params(
    handoff: &BootHandoff,
    params: &SidecarParams,
) -> Result<(), InvalidParams> {
    let node_count = params.node_count as usize;
    if node_count == 0 || node_count > params.nodes.len() {
        return Err(InvalidParams::NodeCount);
    }
    let runtime = handoff
        .runtime
        .checked_range()
        .ok_or(InvalidParams::MemoryRange)?;
    let mut previous_cpu_end = 0;
    for (index, node) in params.nodes[..node_count].iter().enumerate() {
        let cpu_end = node
            .base_vp
            .checked_add(node.vp_count)
            .ok_or(InvalidParams::CpuRange)?;
        if !(2..=32).contains(&node.vp_count) || node.base_vp < previous_cpu_end {
            return Err(InvalidParams::CpuRange);
        }
        previous_cpu_end = cpu_end;
        let memory_end = node
            .memory_base
            .checked_add(node.memory_size)
            .ok_or(InvalidParams::MemoryRange)?;
        if !node
            .memory_base
            .is_multiple_of(sidecar_defs::PAGE_SIZE as u64)
            || !node
                .memory_size
                .is_multiple_of(sidecar_defs::PAGE_SIZE as u64)
            || node.memory_size < sidecar_defs::required_memory(node.vp_count) as u64
            || node.memory_base < runtime.start
            || memory_end > runtime.end
        {
            return Err(InvalidParams::MemoryRange);
        }
        for earlier in &params.nodes[..index] {
            if node.memory_base < earlier.memory_base + earlier.memory_size
                && earlier.memory_base < memory_end
            {
                return Err(InvalidParams::MemoryRange);
            }
        }
        if params.initial_state.per_cpu_state_specified {
            let starts = params
                .initial_state
                .sidecar_starts_cpu
                .get(node.base_vp as usize..cpu_end as usize)
                .ok_or(InvalidParams::CpuRange)?;
            if starts[0] {
                return Err(InvalidParams::KernelBaseCpu);
            }
            if !starts[1..].iter().any(|&starts| starts) {
                return Err(InvalidParams::EmptyNode);
            }
        }
    }
    Ok(())
}

#[cfg(minimal_rt)]
fn fail(tag: [u8; 8], message: &str) -> ! {
    minimal_rt::enlightened_panic::enable_enlightened_panic();
    minimal_rt::enlightened_panic::report_message(tag, message, |address| Some(address as usize));
    minimal_rt::arch::fault()
}

/// Enter a composite image using the ordinary Linux ELF boot argument.
///
/// # Safety
/// The composite image and Linux boot parameters must be valid writable identity
/// mappings. The first image region contains a validated layout and zeroed scratch.
#[cfg(minimal_rt)]
pub unsafe fn start_composite(boot_params: u64) -> ! {
    use loader_defs::kexec::COMPOSITE_ALIGNMENT;
    use loader_defs::kexec::CompositeHeader;

    unsafe extern "C" {
        static __ehdr_start: u8;
    }
    let Some(base) = (core::ptr::addr_of!(__ehdr_start) as u64).checked_sub(COMPOSITE_ALIGNMENT)
    else {
        fail(*b"KXBASE01", "kexec bootstrap: composite base underflow");
    };
    if base == 0 || !base.is_multiple_of(COMPOSITE_ALIGNMENT) {
        fail(
            *b"KXBASE02",
            "kexec bootstrap: invalid composite base alignment",
        );
    }
    // SAFETY: the bootstrap has a fixed offset from the mapped layout region.
    let layout = unsafe { (base as *const CompositeHeader).read() };
    let Some(mut handoff) = layout.relocate(base, boot_params) else {
        fail(
            *b"KXLAYOUT",
            "kexec bootstrap: invalid composite layout or boot-parameter range",
        );
    };
    let handoff_address = base + 4096;
    // SAFETY: the plan page lies in the zero-initialized metadata region.
    let plan =
        unsafe { &*((base + loader_defs::kexec::SIDECAR_PLAN_OFFSET) as *const SidecarPlan) };
    let prepared = (plan.magic != 0).then(|| {
        // SAFETY: the plan was staged with this composite and is validated
        // before any of its addresses are used.
        unsafe { prepare_sidecar(base, layout.image_size, &mut handoff, plan) }
    });
    // SAFETY: the checked layout dedicates this disjoint page to the handoff.
    unsafe { (handoff_address as *mut BootHandoff).write(handoff) };
    // SAFETY: all handoff addresses were constructed from the loaded composite
    // or the plan's reserved region.
    unsafe {
        start(handoff_address, move |output| {
            if let Some(prepared) = prepared {
                prepared.retarget_device_tree(plan, output);
            }
        })
    }
}

/// Device-tree updates that must follow sidecar initialization.
#[cfg(minimal_rt)]
struct PreparedSidecar {
    fdt: &'static mut [u8],
    started: [bool; MAX_SIDECAR_PLAN_CHUNKS],
}

#[cfg(minimal_rt)]
impl PreparedSidecar {
    /// Point every sidecar node at the region the successor owns. Nodes
    /// sidecar did not start have zeroed control pages that Linux skips.
    fn retarget_device_tree(self, plan: &SidecarPlan, output: &sidecar_defs::SidecarOutput) {
        let fdt = self.fdt;
        let mut node = 0;
        for (chunk, &started) in plan.chunks().iter().zip(&self.started) {
            let Some(value) = fdt_reg_value(fdt, chunk.dt_reg_offset) else {
                continue;
            };
            let cells = if started {
                let out = &output.nodes[node];
                node += 1;
                [
                    out.control_page,
                    sidecar_defs::PAGE_SIZE as u64,
                    out.shmem_pages_base,
                    out.shmem_pages_size,
                ]
            } else {
                chunk_reg(plan, chunk)
            };
            write_reg(fdt, value, cells);
        }
    }
}

/// Locate the device tree that kexec carried in the setup_data chain.
///
/// # Safety
/// Every setup_data entry must be identity mapped, as kexec's boot-parameter
/// segment is.
#[cfg(minimal_rt)]
unsafe fn find_fdt(boot_params: &loader_defs::linux::boot_params) -> Option<&'static mut [u8]> {
    let mut address = boot_params.hdr.setup_data.get();
    for _ in 0..64 {
        if address == 0 {
            return None;
        }
        // SAFETY: guaranteed by the caller.
        let header = unsafe { (address as *const loader_defs::linux::setup_data).read_unaligned() };
        let data = address + size_of::<loader_defs::linux::setup_data>() as u64;
        if header.ty == loader_defs::linux::SETUP_DTB {
            // SAFETY: the kexec loader sized this entry for the whole blob.
            return Some(unsafe {
                core::slice::from_raw_parts_mut(data as *mut u8, header.len as usize)
            });
        }
        address = header.next;
    }
    None
}

/// Returns the NUL-terminated Linux command line as a mutable byte slice.
///
/// # Safety
/// The command line must be identity mapped, as kexec's boot-parameter segment is.
#[cfg(minimal_rt)]
unsafe fn cmdline(boot_params: &loader_defs::linux::boot_params) -> &'static mut [u8] {
    let address =
        boot_params.hdr.cmd_line_ptr.get() as u64 | (boot_params.ext_cmd_line_ptr as u64) << 32;
    let start = address as *mut u8;
    // SAFETY: guaranteed by the caller; scanning stops at the terminator.
    let len = (0..MAX_CMDLINE)
        .find(|&index| unsafe { start.add(index).read() } == 0)
        .unwrap_or(0);
    // SAFETY: the `len` bytes before the terminator are mapped command line.
    unsafe { core::slice::from_raw_parts_mut(start, len) }
}

#[cfg(minimal_rt)]
const MAX_CMDLINE: usize = 4096;

/// Allocates zeroed page-table pages from the composite metadata region.
#[cfg(minimal_rt)]
struct PageTableScratch {
    next: u64,
    end: u64,
}

#[cfg(minimal_rt)]
impl PageTableScratch {
    fn allocate(&mut self) -> Option<*mut u64> {
        if self.next >= self.end {
            return None;
        }
        let page = self.next as *mut u64;
        self.next += 4096;
        // SAFETY: the scratch pages are reserved bootstrap metadata.
        unsafe { core::ptr::write_bytes(page, 0, 512) };
        Some(page)
    }
}

/// Identity map `range` with 2 MiB pages in the active page tables.
///
/// Kexec only maps RAM and the loaded segments, not reserved regions.
///
/// # Safety
/// `range` must be 2 MiB aligned and owned by this boot. The active page tables
/// must be identity mapped, as kexec's are.
#[cfg(minimal_rt)]
unsafe fn identity_map(range: core::ops::Range<u64>, scratch: &mut PageTableScratch) -> bool {
    const PRESENT: u64 = 1;
    const WRITABLE: u64 = 1 << 1;
    const LARGE: u64 = 1 << 7;
    const ADDRESS: u64 = 0x000f_ffff_ffff_f000;
    const LARGE_PAGE: u64 = 2 * 1024 * 1024;
    let (cr3, cr4): (u64, u64);
    // SAFETY: reading control registers has no side effects.
    unsafe {
        core::arch::asm!("mov {}, cr3", out(reg) cr3);
        core::arch::asm!("mov {}, cr4", out(reg) cr4);
    }
    if cr4 & x86defs::X64_CR4_LA57 != 0 || !range.start.is_multiple_of(LARGE_PAGE) {
        return false;
    }
    let mut next_table = |entry: *mut u64| -> Option<*mut u64> {
        // SAFETY: entries point into identity-mapped page-table pages.
        let value = unsafe { entry.read_volatile() };
        if value & PRESENT != 0 {
            return Some((value & ADDRESS) as *mut u64);
        }
        let table = scratch.allocate()?;
        // SAFETY: as above.
        unsafe { entry.write_volatile(table as u64 | PRESENT | WRITABLE) };
        Some(table)
    };
    let pml4 = (cr3 & ADDRESS) as *mut u64;
    let mut address = range.start;
    while address < range.end {
        let index = |shift: u32| ((address >> shift) & 511) as usize;
        // SAFETY: CR3 and every table entry reference identity-mapped tables.
        unsafe {
            let Some(pdpt) = next_table(pml4.add(index(39))) else {
                return false;
            };
            let pdpte = pdpt.add(index(30));
            if pdpte.read_volatile() & (PRESENT | LARGE) == PRESENT | LARGE {
                address = (address | ((1 << 30) - 1)) + 1;
                continue;
            }
            let Some(pd) = next_table(pdpte) else {
                return false;
            };
            pd.add(index(21))
                .write_volatile(address | PRESENT | WRITABLE | LARGE);
            core::arch::asm!("invlpg [{}]", in(reg) address);
        }
        address += LARGE_PAGE;
    }
    true
}

/// Validate the plan, assign CPUs from the post-save record, and stage the
/// incoming sidecar in the plan's reserved region.
///
/// # Safety
/// `base` must be the loaded composite with `image_size` bytes, and `handoff`
/// must describe it. The plan region must be reserved for this boot.
#[cfg(minimal_rt)]
unsafe fn prepare_sidecar(
    base: u64,
    image_size: u64,
    handoff: &mut BootHandoff,
    plan: &SidecarPlan,
) -> PreparedSidecar {
    let region = plan.region_base..plan.region_base + plan.region_size;
    let overlaps = |start: u64, len: u64| start < region.end && region.start < start + len;
    if !plan.validate() || overlaps(base, image_size) || overlaps(handoff.boot_params, 4096) {
        fail(*b"KXPLAN00", "kexec bootstrap: invalid sidecar plan");
    }
    let mut scratch = PageTableScratch {
        next: base + loader_defs::kexec::PAGE_TABLE_SCRATCH_OFFSET,
        end: base
            + loader_defs::kexec::PAGE_TABLE_SCRATCH_OFFSET
            + loader_defs::kexec::PAGE_TABLE_SCRATCH_PAGES * 4096,
    };
    // SAFETY: the plan region is 2 MiB aligned, reserved, and not in use.
    if !unsafe { identity_map(region.clone(), &mut scratch) } {
        fail(*b"KXMAP000", "kexec bootstrap: cannot map sidecar region");
    }
    let record_address = plan.region_base + plan.record_offset;
    // SAFETY: the record page is inside the now-mapped plan region.
    let record = unsafe { (record_address as *const SidecarRecord).read() };
    // SAFETY: as above; a stale record must not be reused by a later boot.
    unsafe { (record_address as *mut u64).write_volatile(0) };

    // SAFETY: boot parameters are mapped by kexec and validated by `relocate`.
    let boot_params = unsafe { &*(handoff.boot_params as *const loader_defs::linux::boot_params) };
    // SAFETY: kexec places setup_data in its mapped boot-parameter segment.
    let Some(fdt) = (unsafe { find_fdt(boot_params) }) else {
        fail(
            *b"KXDTB000",
            "kexec bootstrap: carried device tree not found",
        );
    };
    if plan
        .chunks()
        .iter()
        .any(|chunk| fdt_reg_value(fdt, chunk.dt_reg_offset).is_none())
    {
        fail(
            *b"KXDTB001",
            "kexec bootstrap: sidecar device tree node moved",
        );
    }

    // SAFETY: the parameter page is zeroed, page aligned bootstrap metadata.
    let params = unsafe { &mut *(handoff.sidecar_params as *mut SidecarParams) };
    let mut started = assign_sidecar(plan, Some(&record), params);
    if handoff.sidecar.size > plan.record_offset {
        params.node_count = 0;
        started = [false; MAX_SIDECAR_PLAN_CHUNKS];
    }
    let fallback = || {
        let mut all = ArrayString::<MAX_BOOT_CPUS_WIDTH>::new();
        let _ = write!(all, "0-{}", plan.cpu_count - 1);
        all
    };
    let list = boot_cpus(plan, params).unwrap_or_else(|| {
        params.node_count = 0;
        started = [false; MAX_SIDECAR_PLAN_CHUNKS];
        fallback()
    });
    // SAFETY: the command line is in kexec's mapped boot-parameter segment.
    let cmdline = unsafe { cmdline(boot_params) };
    if !patch_boot_cpus(cmdline, &list, plan.boot_cpus_width as usize) {
        // The staged all-CPU placeholder remains; sidecar must not run.
        params.node_count = 0;
        started = [false; MAX_SIDECAR_PLAN_CHUNKS];
    }

    for (chunk, _) in plan
        .chunks()
        .iter()
        .zip(started)
        .filter(|(_, started)| !started)
    {
        // SAFETY: the control page is inside the mapped plan region.
        unsafe {
            core::ptr::write_bytes(
                (plan.region_base + chunk.arena_offset) as *mut u8,
                0,
                sidecar_defs::PAGE_SIZE,
            )
        };
    }

    if params.node_count != 0 {
        // SAFETY: the incoming image fits before the record page, and the
        // region does not overlap the composite.
        unsafe {
            core::ptr::copy_nonoverlapping(
                handoff.sidecar.base as *const u8,
                plan.region_base as *mut u8,
                handoff.sidecar.size as usize,
            )
        };
        handoff.sidecar_entry = plan.region_base + (handoff.sidecar_entry - handoff.sidecar.base);
        handoff.sidecar.base = plan.region_base;
        handoff.runtime = loader_defs::kexec::PhysicalRange {
            base: plan.region_base + plan.image_size,
            size: plan.region_size - plan.image_size,
        };
        handoff.flags = 0;
    }
    PreparedSidecar { fdt, started }
}

/// Enter after the shared assembly entry has initialized BSS, relocations and stack.
///
/// `finish` runs after sidecar initialization, or immediately if disabled.
///
/// # Safety
/// The handoff page and all listed allocations must be writable identity mappings,
/// exclusively owned by this boot. Parameters must contain valid Rust bool values.
/// The predecessor's VPs and DMA must no longer access the new allocations.
#[cfg(minimal_rt)]
unsafe fn start(handoff_address: u64, finish: impl FnOnce(&sidecar_defs::SidecarOutput)) -> ! {
    if handoff_address == 0 || !handoff_address.is_multiple_of(4096) {
        fail(*b"KXHAND01", "kexec bootstrap: invalid handoff address");
    }
    // SAFETY: the caller supplies an exclusively owned, mapped handoff page.
    let handoff = unsafe { (handoff_address as *const BootHandoff).read() };
    if !handoff.validate(handoff_address) {
        fail(*b"KXHAND02", "kexec bootstrap: invalid physical handoff");
    }
    unsafe extern "C" {
        static __ehdr_start: u8;
        static _end: u8;
    }
    let image_base = core::ptr::addr_of!(__ehdr_start) as u64;
    let image_end = core::ptr::addr_of!(_end) as u64;
    if image_base != handoff.bootstrap.base
        || image_end > handoff.bootstrap.base + handoff.bootstrap.size
    {
        fail(
            *b"KXIMAGE0",
            "kexec bootstrap: loaded image bounds do not match handoff",
        );
    }
    // SAFETY: the validated pages are exclusive, disjoint, aligned mappings;
    // the caller initializes SidecarParams with valid field representations.
    let (params, output, boot_params) = unsafe {
        (
            &mut *(handoff.sidecar_params as *mut SidecarParams),
            &mut *(handoff.sidecar_output as *mut sidecar_defs::SidecarOutput),
            &*(handoff.boot_params as *const loader_defs::linux::boot_params),
        )
    };
    if boot_params.hdr.type_of_loader >> 4 != 0x0d {
        fail(
            *b"KXLOADER",
            "kexec bootstrap: Linux boot parameters lack the kexec loader marker",
        );
    }
    if handoff.flags & loader_defs::kexec::BOOT_FLAG_SIDECAR_DISABLED == 0 {
        if validate_sidecar_params(&handoff, params).is_err() {
            fail(
                *b"KXCPUS00",
                "kexec bootstrap: invalid sidecar CPU or memory assignment",
            );
        }
        crate::arch::hypercall::uninitialize();
        crate::hypercall::hvcall().initialize();
        params.hypercall_page = crate::hypercall::hvcall().hypercall_page();
        // SAFETY: the loader verified and relocated the incoming sidecar ELF;
        // its entry is inside the validated, reserved executable allocation.
        let sidecar_entry: extern "C" fn(&SidecarParams, &mut sidecar_defs::SidecarOutput) -> bool =
            unsafe { core::mem::transmute(handoff.sidecar_entry) };
        if !sidecar_entry(params, output) {
            fail(
                *b"KXSTART0",
                "kexec bootstrap: sidecar initialization failed",
            );
        }
        crate::hypercall::hvcall().uninitialize();
    } else if params.node_count != 0 {
        fail(
            *b"KXOFF000",
            "kexec bootstrap: disabled sidecar has nonzero node count",
        );
    }
    finish(output);
    crate::rt::verify_stack_cookie();
    // SAFETY: the verified Linux entry uses the existing x64 boot-shim ABI.
    let kernel_entry: extern "C" fn(u64, &loader_defs::linux::boot_params) -> ! =
        unsafe { core::mem::transmute(handoff.kernel_entry) };
    kernel_entry(0, boot_params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use loader_defs::kexec::PhysicalRange;
    use sidecar_defs::SidecarNodeParams;
    use test_with_tracing::test;
    use zerocopy::FromZeros;

    fn fixture() -> (BootHandoff, SidecarParams) {
        let handoff = BootHandoff {
            magic: loader_defs::kexec::BOOT_ENTRY_MAGIC,
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
            flags: 0,
            reserved: 0,
        };
        let mut params = SidecarParams::new_zeroed();
        params.node_count = 2;
        for (index, node) in params.nodes[..2].iter_mut().enumerate() {
            *node = SidecarNodeParams {
                base_vp: index as u32 * 4,
                vp_count: 4,
                memory_base: handoff.runtime.base + index as u64 * 0x100000,
                memory_size: sidecar_defs::required_memory(4) as u64,
            };
        }
        (handoff, params)
    }

    #[test]
    fn kexec_sidecar_params_accept_default_and_excluded_cpus() {
        let (handoff, mut params) = fixture();
        assert_eq!(validate_sidecar_params(&handoff, &params), Ok(()));
        params.initial_state.per_cpu_state_specified = true;
        params.initial_state.sidecar_starts_cpu[2] = true;
        params.initial_state.sidecar_starts_cpu[7] = true;
        assert_eq!(validate_sidecar_params(&handoff, &params), Ok(()));
    }

    #[test]
    fn kexec_sidecar_params_reject_invalid_ownership() {
        for variant in 0..7 {
            let (handoff, mut params) = fixture();
            match variant {
                0 => params.node_count = 0,
                1 => params.node_count = params.nodes.len() as u32 + 1,
                2 => params.nodes[0].vp_count = 33,
                3 => params.nodes[1].base_vp = 1,
                4 => params.nodes[1].base_vp = u32::MAX,
                5 => {
                    params.initial_state.per_cpu_state_specified = true;
                    params.initial_state.sidecar_starts_cpu.fill(true);
                }
                _ => params.initial_state.per_cpu_state_specified = true,
            }
            assert!(
                validate_sidecar_params(&handoff, &params).is_err(),
                "variant {variant}"
            );
        }
    }

    #[test]
    fn kexec_sidecar_params_reject_invalid_memory() {
        for variant in 0..5 {
            let (handoff, mut params) = fixture();
            match variant {
                0 => params.nodes[0].memory_base += 1,
                1 => params.nodes[0].memory_base = handoff.sidecar.base,
                2 => params.nodes[0].memory_size -= 4096,
                3 => params.nodes[1].memory_base = params.nodes[0].memory_base,
                _ => params.nodes[1].memory_size = u64::MAX,
            }
            assert_eq!(
                validate_sidecar_params(&handoff, &params),
                Err(InvalidParams::MemoryRange)
            );
        }
    }

    fn plan_fixture() -> SidecarPlan {
        use loader_defs::kexec::SIDECAR_REGION_IMAGE_SIZE;
        let arena = sidecar_defs::required_memory(32) as u64;
        let mut plan = SidecarPlan::new_zeroed();
        plan.magic = SidecarPlan::MAGIC;
        plan.version = SidecarPlan::VERSION;
        plan.region_base = 0x4000_0000;
        plan.region_size = 0x40_0000;
        plan.image_size = SIDECAR_REGION_IMAGE_SIZE;
        plan.record_offset = SIDECAR_REGION_IMAGE_SIZE - 4096;
        plan.cpu_count = 64;
        plan.chunk_count = 2;
        plan.boot_cpus_width = 144;
        for (index, chunk) in plan.chunks[..2].iter_mut().enumerate() {
            *chunk = SidecarPlanChunk {
                base_vp: index as u32 * 32,
                vp_count: 32,
                arena_offset: SIDECAR_REGION_IMAGE_SIZE + index as u64 * arena,
                arena_size: arena,
                dt_reg_offset: 0x100,
                reserved: 0,
            };
        }
        assert!(plan.validate());
        plan
    }

    fn runtime_handoff(plan: &SidecarPlan) -> BootHandoff {
        let (mut handoff, _) = fixture();
        handoff.sidecar.base = plan.region_base;
        handoff.sidecar.size = 0x20000;
        handoff.sidecar_entry = plan.region_base;
        handoff.runtime = PhysicalRange {
            base: plan.region_base + plan.image_size,
            size: plan.region_size - plan.image_size,
        };
        handoff
    }

    fn assign(plan: &SidecarPlan, kernel_cpus: &[u32]) -> (SidecarParams, [bool; 128]) {
        let mut record = SidecarRecord::new(plan.cpu_count);
        for &cpu in kernel_cpus {
            assert!(record.set_kernel_cpu(cpu));
        }
        let mut params = SidecarParams::new_zeroed();
        let started = assign_sidecar(plan, Some(&record), &mut params);
        (params, started)
    }

    #[test]
    fn kexec_assigns_fresh_nodes_without_exclusions() {
        let plan = plan_fixture();
        let (params, started) = assign(&plan, &[]);
        assert_eq!(params.node_count, 2);
        assert!(started[0] && started[1] && !started[2]);
        assert!(!params.initial_state.per_cpu_state_specified);
        assert_eq!(
            params.nodes[1].memory_base,
            0x4000_0000 + plan.chunks[1].arena_offset
        );
        assert_eq!(boot_cpus(&plan, &params).unwrap().as_str(), "0,32");
        assert_eq!(
            validate_sidecar_params(&runtime_handoff(&plan), &params),
            Ok(())
        );
    }

    #[test]
    fn kexec_keeps_restored_interrupt_cpus_with_linux() {
        let plan = plan_fixture();
        let (params, started) = assign(&plan, &[5, 40, 41]);
        assert_eq!(params.node_count, 2);
        assert!(started[0] && started[1]);
        let starts = &params.initial_state.sidecar_starts_cpu;
        assert!(params.initial_state.per_cpu_state_specified);
        assert!(!starts[0] && !starts[5] && !starts[32] && !starts[40] && starts[6]);
        assert_eq!(boot_cpus(&plan, &params).unwrap().as_str(), "0,5,32,40-41");
        assert_eq!(
            validate_sidecar_params(&runtime_handoff(&plan), &params),
            Ok(())
        );

        let all_second: [u32; 31] = core::array::from_fn(|index| 33 + index as u32);
        let (params, started) = assign(&plan, &all_second);
        assert_eq!(params.node_count, 1);
        assert!(started[0] && !started[1]);
        assert_eq!(boot_cpus(&plan, &params).unwrap().as_str(), "0,32-63");
    }

    #[test]
    fn kexec_falls_back_to_linux_cpus() {
        let plan = plan_fixture();
        let mut params = SidecarParams::new_zeroed();
        assert!(!assign_sidecar(&plan, None, &mut params).contains(&true));
        assert_eq!(boot_cpus(&plan, &params).unwrap().as_str(), "0-63");
        let stale = SidecarRecord::new(63);
        assert!(!assign_sidecar(&plan, Some(&stale), &mut params).contains(&true));

        let mut small = plan;
        small.chunks[1].arena_size -= 4096;
        small.chunks[0].dt_reg_offset = 0;
        let (params, started) = assign(&small, &[]);
        assert_eq!(
            (params.node_count, started[0], started[1]),
            (0, false, false)
        );

        let mut large = plan;
        large.cpu_count = NUM_CPUS_SUPPORTED_FOR_PER_CPU_STATE as u32 + 1;
        assert_eq!(assign(&large, &[]).0.node_count, 2);
        assert_eq!(assign(&large, &[1]).0.node_count, 0);

        let mut narrow = plan;
        narrow.boot_cpus_width = 3;
        assert!(boot_cpus(&narrow, &assign(&narrow, &[]).0).is_none());
    }

    #[test]
    fn kexec_patches_padded_boot_cpus_in_place() {
        let mut cmdline = *b"a boot_cpus=0-63     b\0junk";
        assert!(patch_boot_cpus(&mut cmdline, "0,32", 8));
        assert_eq!(&cmdline, b"a boot_cpus=0,32     b\0junk");
        for (input, width, value) in [
            (&b"a b"[..], 9, "0"),
            (b"xboot_cpus=0-63      b", 9, "0"),
            (b"boot_cpus=0-63 1     b", 9, "0"),
            (b"boot_cpus=0-63      ", 9, "0123456789"),
            (b"boot_cpus=0-63      b", 12, "0"),
        ] {
            assert!(!patch_boot_cpus(&mut input.to_vec(), value, width));
        }
    }

    #[test]
    fn kexec_retargets_only_four_cell_sidecar_reg() {
        let plan = plan_fixture();
        let mut buf = vec![0; 0x1000];
        let mut builder = fdt::builder::Builder::new(fdt::builder::BuilderConfig {
            blob_buffer: &mut buf,
            string_table_cap: 64,
            memory_reservations: &[],
        })
        .unwrap();
        let p_reg = builder.add_string("reg").unwrap();
        let p_other = builder.add_string("other").unwrap();
        let len = builder
            .start_node("")
            .unwrap()
            .start_node("sidecar")
            .unwrap()
            .add_u64_array(p_reg, &[0x1111, 0x1000, 0x2111, 0x40000])
            .unwrap()
            .add_u64_array(p_other, &[0x3333])
            .unwrap()
            .end_node()
            .unwrap()
            .end_node()
            .unwrap()
            .build(0)
            .unwrap();
        let fdt = &mut buf[..len];
        let find = |fdt: &[u8], value: u64| {
            fdt.windows(8)
                .position(|w| w == value.to_be_bytes())
                .unwrap() as u32
        };
        let reg = find(fdt, 0x1111);
        assert!(fdt_reg_value(fdt, find(fdt, 0x3333)).is_none());
        assert!(fdt_reg_value(fdt, reg + 8).is_none());
        let range = fdt_reg_value(fdt, reg).unwrap();
        let cells = chunk_reg(&plan, &plan.chunks[1]);
        assert_eq!(
            cells,
            [
                0x4000_0000 + plan.chunks[1].arena_offset,
                4096,
                0x4000_0000 + plan.chunks[1].arena_offset + 4096,
                32 * 2 * 4096
            ]
        );
        write_reg(fdt, range, cells);
        assert_eq!(find(fdt, cells[0]), reg);
        assert_eq!(find(fdt, cells[3]), reg + 24);
    }
}
