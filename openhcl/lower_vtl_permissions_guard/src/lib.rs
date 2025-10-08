// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Implements a VtlMemoryProtection guard that can be used to temporarily allow
//! access to pages that were previously protected.

#![cfg(target_os = "linux")]

mod device_dma;

pub use device_dma::LowerVtlDmaBuffer;

use anyhow::Context;
use anyhow::Result;
use inspect::Inspect;
use std::sync::Arc;
use user_driver::DmaClient;
use user_driver::memory::MemoryBlock;
use virt::VtlMemoryProtection;

/// A guard that will restore [`hvdef::HV_MAP_GPA_PERMISSIONS_NONE`] permissions
/// on the pages when dropped.
#[derive(Inspect)]
struct PagesAccessibleToLowerVtl {
    #[inspect(skip)]
    vtl_protect: Arc<dyn VtlMemoryProtection + Send + Sync>,
    #[inspect(hex, iter_by_index)]
    pages: Vec<u64>,
}

impl PagesAccessibleToLowerVtl {
    /// Creates a new guard that will lower the VTL permissions of the pages
    /// while the returned guard is held.
    fn new_from_pages(
        vtl_protect: Arc<dyn VtlMemoryProtection + Send + Sync>,
        pages: &[u64],
    ) -> Result<Self> {
        // If skipping VTL protection changes (kexec scenario), do nothing and
        // return an empty guard that will also no-op on Drop.
        let skip = std::env::var("OPENHCL_SKIP_VTL_PROTECT")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if skip {
            return Ok(Self {
                vtl_protect,
                pages: Vec::new(),
            });
        }
        for pfn in pages {
            vtl_protect
                .modify_vtl_page_setting(*pfn, hvdef::HV_MAP_GPA_PERMISSIONS_ALL)
                .context("failed to update VTL protections on page")?;
        }
        Ok(Self {
            vtl_protect,
            pages: pages.to_vec(),
        })
    }
}

impl Drop for PagesAccessibleToLowerVtl {
    fn drop(&mut self) {
        // Skip restoration entirely if user requested skipping protection
        // transitions (kexec debug scenario). Intentionally leaves any pages
        // in their elevated state – do NOT use outside controlled debugging.
        let skip = std::env::var("OPENHCL_SKIP_VTL_PROTECT")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if skip {
            return;
        }
        if let Err(err) = self
            .pages
            .iter()
            .map(|pfn| {
                self.vtl_protect
                    .modify_vtl_page_setting(*pfn, hvdef::HV_MAP_GPA_PERMISSIONS_NONE)
                    .context("failed to update VTL protections on page")
            })
            .collect::<Result<Vec<_>>>()
        {
            // Normally the inability to rollback any pages is fatal because
            // leaving elevated permissions compromises the platform. For
            // kexec debugging scenarios we allow opting-out via the
            // OPENHCL_IGNORE_VTL_PROTECT_RESET=1 environment variable so we
            // can progress further and uncover subsequent failures.
            let ignore = std::env::var("OPENHCL_IGNORE_VTL_PROTECT_RESET")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
            if ignore {
                eprintln!(
                    "[lower_vtl_permissions_guard] WARN ignoring failed to reset page protections: {}",
                    err
                );
            } else {
                panic!(
                    "failed to reset page protections {}",
                    err.as_ref() as &dyn std::error::Error
                );
            }
        }
    }
}

/// A [`DmaClient`] wrapper that will lower the VTL permissions of the page
/// on the allocated memory block.
#[derive(Inspect)]
pub struct LowerVtlMemorySpawner<T: DmaClient> {
    #[inspect(skip)]
    spawner: T,
    #[inspect(skip)]
    vtl_protect: Arc<dyn VtlMemoryProtection + Send + Sync>,
}

impl<T: DmaClient> LowerVtlMemorySpawner<T> {
    /// Create a new wrapped [`DmaClient`] spawner that will lower the VTL
    /// permissions of the returned [`MemoryBlock`].
    pub fn new(spawner: T, vtl_protect: Arc<dyn VtlMemoryProtection + Send + Sync>) -> Self {
        Self {
            spawner,
            vtl_protect,
        }
    }
}

impl<T: DmaClient> DmaClient for LowerVtlMemorySpawner<T> {
    fn allocate_dma_buffer(&self, len: usize) -> Result<MemoryBlock> {
        let mem = self.spawner.allocate_dma_buffer(len)?;
        let vtl_guard =
            PagesAccessibleToLowerVtl::new_from_pages(self.vtl_protect.clone(), mem.pfns())
                .context("failed to lower VTL permissions on memory block")?;

        Ok(MemoryBlock::new(LowerVtlDmaBuffer {
            block: mem,
            _vtl_guard: vtl_guard,
        }))
    }

    fn attach_pending_buffers(&self) -> Result<Vec<MemoryBlock>> {
        anyhow::bail!("restore is not supported for LowerVtlMemorySpawner")
    }
}
