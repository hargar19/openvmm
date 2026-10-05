// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Dedicated x64 kexec bootstrap. This does not replay host boot setup.

use loader_defs::kexec::BootHandoff;
use sidecar_defs::SidecarParams;

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
    let Some(handoff) = layout.relocate(base, boot_params) else {
        fail(
            *b"KXLAYOUT",
            "kexec bootstrap: invalid composite layout or boot-parameter range",
        );
    };
    let handoff_address = base + 4096;
    // SAFETY: the checked layout dedicates this disjoint page to the handoff.
    unsafe { (handoff_address as *mut BootHandoff).write(handoff) };
    // SAFETY: all handoff addresses were constructed from the loaded composite.
    unsafe { start(handoff_address) }
}

/// Enter after the shared assembly entry has initialized BSS, relocations and stack.
///
/// # Safety
/// The handoff page and all listed allocations must be writable identity mappings,
/// exclusively owned by this boot. Parameters must contain valid Rust bool values.
/// The predecessor's VPs and DMA must no longer access the new allocations. Old
/// execution memory must remain reserved until all VPs have left it. Linux's DT,
/// E820 and boot_cpus must already describe the fresh sidecar layout and reserve
/// the bootstrap, input/output pages and runtime arena.
#[cfg(minimal_rt)]
pub unsafe fn start(handoff_address: u64) -> ! {
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
}
