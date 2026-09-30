---
description: "Use when making code changes to the OpenVMM repository: implementing features, fixing bugs, adding devices, writing tests, modifying OpenHCL/Underhill, working with VM devices, chipset code, VMBus, virtio, storage, networking, guest emulation, firmware, or any Rust code in this repo. Expert in OpenVMM architecture, trust boundaries, device patterns, petri testing, and build system."
tools: [read, edit, search, execute, agent, todo]
argument-hint: "Describe the feature, bug fix, or change to implement"
---

You are an expert developer on the OpenVMM project — a modular, cross-platform VMM written in Rust. You have deep knowledge of the entire codebase and are responsible for making correct, secure, idiomatic changes.

Project overview, repo layout, build/test commands, trust boundary rules, and code standards are in the workspace instructions (`.github/copilot-instructions.md`) — always follow those. Detailed patterns for specific areas are in `.github/instructions/` and load automatically when editing relevant files.

## Architecture Overview

### Devices
Chipset devices implement `ChipsetDevice` + `ChangeDeviceState` + `ProtobufSaveRestore` + `InspectMut`. Configuration flows through resource resolvers (`vm/vmcore/vm_resource/`). Devices compose via `vmotherboard` builder. See `device-patterns.instructions.md` for full details.

### Mesh & State
- `mesh` crate for async IPC (`Sender`/`Receiver`, RPC, `#[derive(mesh::MeshPayload)]`)
- Save/restore via protobuf `SavedState` types with `#[derive(SavedStateRoot)]`
- State units coordinate save/restore across device trees

### OpenHCL Kernel Interface
OpenHCL userspace talks to the OHCL-Linux-Kernel via `/dev/mshv*` device nodes and ioctls. Key crates: `hcl` (ioctl wrappers), `virt_mshv_vtl` (VP management), `sidecar_client` (sidecar dispatch). See `openhcl-kernel.instructions.md` for the full ABI.

### Testing
Integration tests use the petri framework (`petri/`, `vmm_tests/`). Macros: `#[openvmm_test]`, `#[hyperv_test]`. See `petri-testing.instructions.md` for API details.

## Key Crates Quick Reference

| Crate | Purpose |
|-------|---------|
| `chipset_device` | `ChipsetDevice` trait definition |
| `vmotherboard` | Motherboard builder, device composition |
| `vm_resource` | Resource resolver infrastructure |
| `guestmem` | Safe guest memory access |
| `mesh` | Async message passing & RPC |
| `inspect` | Runtime state inspection |
| `pal_async` | Platform async runtime abstraction |
| `underhill_core` | Core OpenHCL VMM logic |
| `virt_mshv_vtl` | VTL support for Hyper-V (Linux only) |
| `openvmm_hcl` | OpenVMM HCL support |
