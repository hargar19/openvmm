Yes. Backporting LUO on its own isn't enough, and your teammate's no-IOMMU work is one of the required pieces. LUO preserves a device by preserving an open file for it. For VFIO that file is the per-device cdev (`/dev/vfio/devices/vfioN`), which runs through iommufd. OpenHCL can't use that path today:

- **What OpenHCL uses now:** VTL2 has no IOMMU, so `vfio.rs` opens devices through the legacy group/container API in no-IOMMU mode (`open_noiommu`, `IommuType::NoIommu`). It also calls the out-of-tree `VFIO_GROUP_KEEP_ALIVE` ioctl to keep devices alive across host servicing.
- **Why that blocks LUO:** until recently the cdev/iommufd path refused no-IOMMU devices. Jacob Pan's series (e8efdf02d3a97, "vfio: Enable cdev noiommu mode under iommufd", with Jason Gunthorpe's iommufd patches) lifts that restriction. It also adds `IOMMU_IOAS_NOIOMMU_GET_PA`, so userspace can still find the physical addresses of its DMA memory. It's in `liveupdate/next` but not in your 6.18 kernel, and the OHCL config doesn't enable `IOMMUFD` or `VFIO_DEVICE_CDEV`. That's the backport your teammate is doing.

**Everything needed to keep an NVMe/MANA device live across kexec:**

| Layer | Upstream status | In OHCL 6.18 |
|---|---|---|
| KHO | Merged, but upstream has since reworked it (renamed scratch to bootmem, removed the finalize step) | Older version; the lowmem fix is local only |
| LUO core (`/dev/liveupdate`) | Merged in 6.19 | Missing; it depends on the newer KHO, so the KHO rework comes with it |
| iommufd no-IOMMU and VFIO cdev | In `liveupdate/next` | Missing (teammate's backport) |
| PCI core preservation: no reset at enumeration, bus mastering left on during kexec | Only on the liveupdate `lu-pci-preservation` branch (merged into `next`), not in mainline | Missing |
| VFIO-pci file handler that preserves the device cdev | **Not in any tree I have**; not in `drivers/vfio` on `liveupdate/next` | Missing |
| OpenVMM user driver | n/a | Needs to move to cdev + iommufd, preserve the device through a LUO session before kexec, then retrieve it and call `NvmeDriver::restore()` |

The DMA queue memory is the easier part. It already comes from the VTL2 private pool, which survives kexec today. With no IOMMU there are also no IOMMU page tables to preserve.

**What this means:**
- The VFIO-pci handler isn't upstream yet, so even with every backport above, the full LUO approach isn't complete.
- Backporting this much to 6.18 is large. Moving the OHCL kernel to a newer base may cost less.
- **Shortcut if the data justifies it:** OHCL already has `VFIO_GROUP_KEEP_ALIVE`, which skips the device reset when the new instance reopens it. For kexec, the remaining gaps are the old kernel's shutdown path, which disables bus mastering and resets the device, and making sure the new kernel doesn't reset it during enumeration. Extending the out-of-tree keepalive to cover kexec could keep devices live without LUO. It's out-of-tree, though, so it adds to your kernel maintenance.

This is why the `vm_build` breakdown and the NVMe keepalive runs matter. They show how many milliseconds device teardown and setup actually cost, which tells you whether the LUO plus no-IOMMU work, or the shortcut, is worth doing. I haven't changed any code for this question.

**Task completed:**

Yes, a LUO backport alone isn't enough, and no-IOMMU support is required. OpenHCL opens devices through the legacy VFIO group API in no-IOMMU mode, but LUO preserves a VFIO device through the per-device cdev, which needs Jacob Pan's iommufd no-IOMMU series. That series is in `liveupdate/next` but not in your 6.18 kernel.

Other pieces are still needed:
- LUO core plus the newer KHO it depends on.
- PCI preservation, which is so far only on the liveupdate tree's `lu-pci-preservation`/`next` branches, not mainline.
- A VFIO-pci LUO file handler, which isn't in any tree I checked.
- OpenVMM changes to use the cdev/iommufd path and LUO sessions.

A cheaper interim option is extending OHCL's out-of-tree `VFIO_GROUP_KEEP_ALIVE` to cover kexec. The `vm_build` breakdown and NVMe keepalive measurements will show which option is worth the effort. I made no code changes.
