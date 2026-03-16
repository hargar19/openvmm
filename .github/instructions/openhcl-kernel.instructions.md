---
description: "Use when working on OpenHCL/Underhill code that interfaces with the mshv kernel: ioctls, device nodes, VTL switching, sidecar, hypercalls, guest memory mapping, VP run loop, SNP/TDX CVM support, or boot flow."
applyTo: ["openhcl/hcl/**", "openhcl/virt_mshv_vtl/**", "openhcl/sidecar_client/**", "openhcl/sidecar_defs/**", "openhcl/underhill_mem/**", "openhcl/underhill_core/**", "openhcl/underhill_entry/**", "openhcl/underhill_init/**", "vmm_core/virt_mshv/**"]
---

# Underhill Kernel Interface (OHCL-Linux-Kernel)

OpenHCL userspace runs on a custom Linux kernel (OHCL-Linux-Kernel). The kernel exposes the Hyper-V mshv driver via device nodes. This documents the ABI contract.

## Kernel Repo Layout

- `drivers/hv/mshv_vtl_main.c` — VTL paravisor driver (creates device nodes)
- `drivers/hv/mshv_main.c` — Root partition driver
- `drivers/hv/mshv_vtl_sidecar.c` — Sidecar kernel module
- `arch/x86/hyperv/hv_vtl.c` — x86 VTL switching, TDX/SNP
- `include/uapi/linux/mshv.h` — **UAPI header**: all ioctl numbers and shared structs

## Device Nodes

| Device | OpenHCL Crate | Purpose |
|--------|--------------|---------|
| `/dev/mshv` | `hcl` (`Mshv`) | Top-level; `CHECK_EXTENSION`, `CREATE_VTL` |
| `/dev/mshv_vtl` | `hcl` (`MshvVtl`), `virt_mshv_vtl` | Per-VP run loop, register get/set, VTL return, memory |
| `/dev/mshv_vtl_low` | `hcl` (`MshvVtlLow`), `underhill_mem` | VTL0 GPA access (mmap-based), framebuffer |
| `/dev/mshv_hvcall` | `hcl` (`MshvHvcall`) | Direct hypercall passthrough |
| `/dev/mshv_sint` | `hcl` (`vmbus` module) | VMBus SynIC message/event |
| `/dev/mshv_vtl_sidecar{N}` | `sidecar_client` | Sidecar per-node VP execution |

## IOCTLs (base: `0xB8`)

**mshv device:**
- `MSHV_CHECK_EXTENSION` (0x00) — Query capabilities
- `MSHV_CREATE_PARTITION` (0x01) — Create partition (root path)
- `MSHV_CREATE_VTL` (0x1D) — Create VTL device fd

**VTL device:**
- `MSHV_GET/SET_VP_REGISTERS` (0x05/0x06) — VP registers (max 128/call)
- `MSHV_RUN_VP` (0x07) — Enter VTL0, returns `hv_message` on exit
- `MSHV_VTL_RETURN_TO_LOWER_VTL` (0x27) — Relinquish to VTL0
- `MSHV_VTL_ADD_VTL0_MEMORY` (0x21) — Add VTL0 memory as ZONE_DEVICE
- `MSHV_VTL_SET_POLL_FILE` (0x25) — Set poll fd per CPU
- `MSHV_VTL_KICK_CPU` (0x38) — Wake/cancel VTL0 CPUs

**CVM (SNP):** `PVALIDATE` (0x28), `RMPADJUST` (0x29), `RMPQUERY` (0x35)
**CVM (TDX):** `TDCALL` (0x32)

**Hypercall:** `HVCALL_SETUP` (0x1E), `HVCALL` (0x1F)

**VMBus SINT:** `SIGNAL_EVENT` (0x22), `POST_MESSAGE` (0x23), `SET_EVENTFD` (0x24)

**Sidecar:** `START` (0xf0), `STOP` (0xf1), `RUN` (0xf2), `INFO` (0xf3)

## Shared Memory (mmap)

- **Register page** — `mmap(vp_fd, HV_VP_MMAP_REGISTERS_OFFSET)` → `HvRegisterPage`
- **APIC page** — `mmap(vp_fd, MSHV_APIC_PAGE_OFFSET)` → local APIC state
- **VTL0 GPA** — `mmap(mshv_vtl_low_fd, gpa)` → direct guest memory
- **Sidecar shmem** — Per-VP `CommandPage` for lockless VP dispatch
- **TDX VP context** — `tdx_vp_context` (GPR list, FPU, TSC deadline) at fixed offset

## Crate Roles

| Crate | Role |
|-------|------|
| `hcl` | Primary ioctl wrappers, `ioctls` module mirroring UAPI structs, `HclVp` run loop |
| `virt_mshv_vtl` | Higher-level VP management, implements `virt` traits (Linux-only) |
| `sidecar_client` | Sidecar device client, maps shmem, dispatches VP runs |
| `sidecar_defs` | Shared structs: `CommandPage`, `SidecarParams`, `CpuContextX64` |
| `underhill_mem` | Memory init via `MshvVtlLow` |
| `underhill_core` | Top-level orchestration |
| `virt_mshv` | Root partition path — full VM management via `/dev/mshv` |

## Boot Flow

1. Kernel boots in VTL2, initializes `mshv_vtl` driver, creates device nodes
2. `underhill_init` → `underhill_entry` (first userspace process)
3. `/dev/mshv` → `CREATE_VTL` → `/dev/mshv_vtl` fd
4. `/dev/mshv_hvcall` → allowed hypercall bitmap
5. `/dev/mshv_vtl_low` → map VTL0 guest memory
6. `/dev/mshv_sint` → VMBus SINT handling
7. Optionally `/dev/mshv_vtl_sidecar{N}` → sidecar start
8. VP run loop: `RETURN_TO_LOWER_VTL` → handle exit → repeat
