---
description: "Use when implementing, modifying, or debugging VM devices: ChipsetDevice trait, ChangeDeviceState lifecycle, ProtobufSaveRestore, InspectMut, resource resolution, PIO/MMIO intercepts, interrupt handling, virtio devices, or chipset components."
applyTo: ["vm/devices/**", "vm/chipset_device/**", "vm/chipset_device_resources/**", "vm/vmcore/vm_resource/**", "vmm_core/vmotherboard/**", "workers/**"]
---

# Device Implementation Patterns

## ChipsetDevice Trait

All chipset devices implement `ChipsetDevice` (`vm/chipset_device/src/lib.rs`) with optional sub-traits:
- `PortIoIntercept` — x86 port I/O (PIO) handling
- `MmioIntercept` — Memory-mapped I/O handling
- `PciConfigSpace` — PCI configuration space reads/writes
- `PollDevice` — Async polling for device readiness (wake on input, timers, etc.)
- `LineInterruptTarget` — Receives interrupt line level changes
- `HandleEoi` — End-of-interrupt handling
- `AcknowledgePicInterrupt` — PIC interrupt acknowledgment

## Required Companion Traits

Every device must also implement:
- **`ChangeDeviceState`** — Lifecycle: `start()` (begin async work, sync), `stop()` (pause, async), `reset()` (zero state, async)
- **`ProtobufSaveRestore`** — Serialize state to stable protobuf `SavedState` messages. Use `#[derive(SavedStateRoot)]`. Always define stable schemas separate from runtime types.
- **`InspectMut`** — Runtime state inspection via `#[derive(Inspect)]` or manual `impl InspectMut`

The combined supertrait `VmmChipsetDevice` (in `vmotherboard`) requires all three.

## Resource Resolution

Device configuration flows through `vm/vmcore/vm_resource/`:
1. Define a resource config type (e.g., `VirtioNetHandle`) implementing `Resource<Kind>`
2. Implement `ResolveResource` (sync) or `AsyncResolveResource` (can resolve sub-resources)
3. Register with `declare_static_resolver!` macro for link-time discovery
4. Resolution receives runtime context: `GuestMemory`, `VmTimeSource`, `VmTaskDriverSource`, PIO/MMIO registration

## Motherboard Builder

`vmm_core/vmotherboard/` composes devices: `BaseChipsetBuilder` → `ChipsetBuilder` → `Chipset`
- Handles interrupt routing, I/O dispatch, device lifecycle coordination
- Devices declared with `configure.omit_saved_state()` skip save/restore

## Device Workers

Devices can run out-of-process via `workers/chipset_device_worker/`:
- `ChipsetDeviceProxy` forwards all trait methods over mesh channels
- `RemoteChipsetDeviceWorker` receives requests, updates device locally
- Used for: GED (Guest Emulation Device), VNC, debug stub
