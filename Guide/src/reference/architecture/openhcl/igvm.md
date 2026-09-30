# IGVM Image

The Independent Guest Virtual Machine (IGVM) format describes the initial state of an isolated virtual machine. OpenHCL is delivered as an IGVM image.

> **Note:** For more details on the IGVM specification, see the [IGVM repository](https://github.com/microsoft/igvm).

## Purpose

The IGVM file serves as the "firmware image" for the OpenHCL paravisor. It allows the host VMM to:

1. Load the OpenHCL components into VTL2 memory.
2. Place them at specific, required physical addresses in. (Components are loaded in a well-defined order to ensure that measurements are reproducable).
3. Pass initial configuration data to the paravisor.

## IGVM Image Contents

An OpenHCL IGVM image bundles the following artifacts:

- **Boot Shim (`openhcl_boot`):** The entry point for VTL2 execution.
- **Linux Kernel:** The operating system kernel.
- **Sidecar Kernel (x86_64):** The lightweight kernel for APs.
- **Initial Ramdisk (initrd):** The root filesystem containing userspace binaries (`underhill_init`, `openvmm_hcl`, etc.).
- **Memory Layout:** Directives specifying where each component should be loaded in memory.
- **Measurements:** Information that the underlying platform uses to confirm that the file was loaded correctly and signed by the appropriate authorities.
- **Configuration:** Boot-time parameters. This includes the data that is known at build time (and measured), and data that is not known until the VM is started (e.g. CPU topology, device settings, etc.). See [`ParavisorMeasuredVtl0Config`](https://openvmm.dev/rustdoc/linux/loader_defs/paravisor/struct.ParavisorMeasuredVtl0Config.html) and [`ParavisorMeasuredVtl2Config`](https://openvmm.dev/rustdoc/linux/loader_defs/paravisor/struct.ParavisorMeasuredVtl2Config.html) for examples of data known at build time.

## Experimental kexec servicing

An IGVM can also carry the replacement kernel and initrd for in-place OpenHCL
servicing. This experimental path requires a compatible custom Hyper-V
delivery implementation installed on the host and supports only x64,
non-isolated VMs. Both bootstrap and successor images must support the
payload descriptor and persisted-state restore contract described here.
Unmodified Mayank images are not supported.

### Measured payload descriptor

The custom raw binary is stored in ordinary measured IGVM `PageData`,
alongside the image's existing initrd. For kexec, this binary must be the
same raw, uncompressed x64 ELF `vmlinux` used to build the image's kernel.
It is not a kernel file added to the root filesystem.

`loader_defs::paravisor` defines the shared `KexecPayloadDescriptor` used by
the image builder and servicing parser. It follows the existing inline
product policy bytes in the measured VTL2 configuration region. The
`ParavisorMeasuredVtl2Config` layout remains 24 bytes, and the product policy
still begins at byte offset 24. The descriptor starts at
`align8(24 + product_policy_size)`, relative to the start of that region,
where `align8` rounds up to an eight-byte boundary.

The descriptor is 48 bytes, with the following fields in order:

| Field | Type | Meaning |
| --- | --- | --- |
| `magic` | `u64` | Little-endian bytes `b"OHCLKEX1"`. |
| `version` | `u32` | `1`. |
| `reserved` | `u32` | Must be `0`. |
| `initrd_base` | `u64` | Initrd guest physical address before relocation. |
| `initrd_size` | `u64` | Exact initrd byte length, excluding page padding. |
| `custom_binary_base` | `u64` | Raw binary guest physical address before relocation. |
| `custom_binary_size` | `u64` | Exact raw binary byte length, excluding page padding. |

The descriptor must fit within the existing measured configuration region;
the builder and parser reject a payload that would require extending that
region. Neither the product policy nor the existing configuration header
is moved to make room.

### Delivery and transition

GED notification 10 (`SEND_IGVM_TO_GUEST`) transports the complete incoming
IGVM. Only one image transfer is accepted at a time. A 128 MiB limit applies
both to the transferred image and to its expanded contents, so a small file
cannot bypass the limit through expansion.

The current parser requires a single x64 VSM platform with highest VTL 2
and no shared GPA boundary. The standard `x64` recipe selects a single
non-isolated VTL2 configuration; CVM or multi-platform images are not
supported by this servicing path.

OpenHCL validates the incoming image and uses its descriptor to extract the
kernel and initrd from normal, private, measured 4 KiB pages. The recorded
sizes preserve exact payload bytes rather than including page padding.
Servicing does not read a root filesystem `boot/vmlinux` or rebuild a CPIO
archive from the running system.

Before stopping the VM, OpenHCL writes the extracted kernel and initrd into
separate anonymous, RAM-backed memfds using `sparse_mmap::alloc_shared_memory`.
Owned `File` handles keep both descriptors alive through
`kexec_sys::kexec_file_load` and close them automatically afterward, including
on errors. No temporary filesystem paths are needed. The syscall uses the
existing `CStr` command-line interface and `KEXEC_FILE_FORCE_DTB` flag.
The transition reuses the existing persisted-state
memory for the successor's userspace restore; it does not package that state
into a newly built initrd.

The old host save notification still follows the normal host-driven restart
path. Its `enable_kexec` bit is retained for wire compatibility but ignored.
The obsolete `service-vtl2 --kexec` console path and
`OPENHCL_SERVICING_RESTART_VIA_KEXEC` boot option have been removed. Incoming
`SEND_IGVM_TO_GUEST` notifications select this IGVM kexec path.

There is no new wire servicing-response message. `HostRequests` value 30
remains `LOAD_FIRMWARE`, and successful servicing uses ordinary OpenHCL
start completion. This repository provides neither the compatible host
delivery source nor a host trigger CLI.

See [building and testing kexec payloads](../../../dev_guide/getting_started/build_openhcl.md#experimental-igvm-kexec-payloads)
for the build recipe, artifact checks, and same-build and A-to-B validation.

## Build Process

The IGVM artifact is generated by the OpenHCL build system.
See [Building OpenHCL](../../../dev_guide/getting_started/build_openhcl.md) for instructions on how to build it.
