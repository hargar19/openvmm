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

The V1 descriptor is 48 bytes, with the following fields in order:

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

#### Raw sidecar payload (V2)

The earlier raw-sidecar format uses `KexecPayloadDescriptorV2`. Its 64-byte
layout retains the V1 magic and common
field offsets, sets `version` to `2`, and appends:

| Field | Type | Meaning |
| --- | --- | --- |
| `sidecar_base` | `u64` | Raw sidecar ELF GPA before relocation, or `0`. |
| `sidecar_size` | `u64` | Exact sidecar ELF file size, or `0`. |

Both fields must be zero when sidecar is absent. The descriptor's complete
version-specific length must fit after the product policy. Unknown versions
are rejected. Images without a custom binary have no descriptor; images with
a custom binary but no sidecar continue to use V1.

The raw payload is copied from the same ELF used for normal sidecar loading.
It is separate from the loaded sidecar segments and includes the original
file bytes, excluding IGVM page padding. It does not depend on reading the
running sidecar's reserved memory or adding sidecar to the root filesystem.
The extractor checks private measured pages, bounds, nonoverlapping payload
ranges, and a supported x64 ELF with a file-backed executable entry point.

V2 remains readable for extraction tests, but a V2 sidecar payload cannot be
staged: it lacks the incoming bootstrap required by V3.

#### Bootstrap payload (V3)

An x64 image with a custom binary and sidecar now emits
`KexecPayloadDescriptorV3`. Its 80-byte layout retains the V2 prefix, sets
the common `version` to `3`, and appends `bootstrap_base: u64` and
`bootstrap_size: u64`. Both sidecar and bootstrap are required. The bootstrap
is the raw `openhcl_boot` ELF from the same build. Its composite entry is
advertised by a standard ELF note owned by `OPENHCL`, type `0x4f48434c`.
The 16-byte descriptor contains `OHCLCE01` and a signed 64-bit displacement
from the displacement field to that entry. Missing, duplicate, or invalid
notes are rejected before stopping the VM. Older V3 bootstrap artifacts
without this note must be rebuilt. Normal host boot is unchanged.

OpenVMM constructs a standard x64 `ET_EXEC` ELF on the kernel memfd. Its four
`PT_LOAD` regions contain metadata/scratch, bootstrap, sidecar, and Linux.
Each region starts at a 2 MiB-aligned relative address. Input load segments
retain their byte offsets within each component, including valid unaligned
segments; BSS and gaps are zero-filled. The separate initrd fd is unchanged.
All parsing, bounds checks, composition, and staging happen before VM stop.

The existing ELF loader places this image at one relocated base and passes
Linux's boot parameters through its existing `RSI` argument. The ELF entry
points to the advertised bootstrap stub. The stub selects composite startup;
after stack/BSS/relocation setup, the bootstrap locates its relative metadata
one 2 MiB region before its own base. It constructs and validates the physical
handoff itself and passes the original boot parameters to Linux.

There is no kernel-recognized bundle header, companion loader, or special
register ABI. The pre-existing raw-ELF, DTB, and VTL2 kexec support is still
required; this does not imply compatibility with an unmodified upstream
kernel. The old `OHCLKB01` bundle-specific kernel changes are no longer used.

Runtime composition does not preserve a signature on the original Linux
ELF. Kernel signature enforcement is not bypassed: deployments requiring
signed kexec images must authenticate the complete composite through a
trusted signing flow. Automatic composite signing is not implemented.

```admonish warning
The current test mode requires `OPENHCL_SIDECAR=off` and no active sidecar
client in the running partition. It exercises kexec through the incoming bootstrap
but does not start sidecar VPs. Fresh sidecar CPU assignment, predecessor
retirement, and DT/E820 regeneration remain unimplemented and gated off.
Both OpenHCL instances need the composite staging/entry implementation. The
kernel requires only the pre-existing ELF/DTB/VTL2 kexec implementation.
```

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

Before stopping the VM, OpenHCL writes the direct or composite ELF and initrd into
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
