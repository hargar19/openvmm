# Servicing Saved State — Complete Reference

This document describes **everything** that is persisted across a servicing
event (both the traditional host-held path and the kexec in-guest path).

Data is split into two categories:

1. **The `ServicingState` blob** — a protobuf-encoded binary payload that
   captures all VM device and subsystem state.
2. **Persistent memory regions** — physical memory ranges that survive the VTL 2
   kernel restart and carry topology/layout information.

## Kernel vs Userspace Ownership

During a servicing restart there are three consumers of saved data:

| Consumer | What it reads | When |
|----------|--------------|------|
| **Boot shim (`openhcl_boot`)** | `PersistedStateHeader` + protobuf topology (`partition_memory`, `partition_mmio`, `cpus_with_mapped_interrupts_no_io`, `cpus_with_outstanding_io`). | Very early, before the Linux kernel boots. Uses this to reconstruct the memory layout, decide sidecar policy, and build the device-tree (FDT) for the kernel. |
| **Linux kernel** | The FDT written by the boot shim (memory nodes, reserved regions, command-line flags like sidecar disable). | Kernel boot. The kernel does **not** read the persisted state or ServicingState directly — it only sees the FDT that `openhcl_boot` constructed from it. |
| **Underhill userspace** | The full `ServicingState` blob (`init_state` + `units`). | After the kernel boots and `underhill_entry` starts the VMM process. |

### What the Kernel Side Needs (boot shim / `openhcl_boot`)

These live in the **persistent memory regions** (Section 2) and are consumed
before the Linux kernel starts:

| Data | Purpose |
|------|---------|
| `PersistedStateHeader` | Locates the protobuf region and servicing state region. |
| `partition_memory` (Vec of `MemoryEntry`) | Rebuilds the VTL 2 RAM map and address space without re-querying the host. Determines persisted-header, protobuf, servicing-state, and GPA-pool sub-regions. Written into FDT memory nodes for the kernel. |
| `partition_mmio` (Vec of `MmioEntry`) | Rebuilds VTL 0 and VTL 2 MMIO ranges. |
| `cpus_with_mapped_interrupts_no_io` | Heuristic: if non-empty, the boot shim may disable **sidecar** for small VMs (since keepalive CPUs need to be powered on to service interrupts, removing the sidecar amortization benefit). |
| `cpus_with_outstanding_io` | Same heuristic as above. |
| Sidecar disable flag | Derived from the above; written into the kernel command line as `SidecarOptions::DisabledServicing`. |

The boot shim does **not** read or interpret the `ServicingState` blob. It only
reserves the memory region where the blob sits (so the kernel won't overwrite it)
and passes the region location through the FDT.

### What Userspace Needs (Underhill VMM)

Everything in the **`ServicingState` blob** (Section 1) is consumed exclusively
by the Underhill userspace VMM. The blob is read in
[`UhVmWorker::new()`](openhcl/underhill_core/src/worker.rs)
— either from the persisted memory region (kexec path) via
`read_servicing_state_from_persisted()`, or from the host via
`get_saved_state_from_host()`.

The blob is split into two parts:

| Part | Consumer | Purpose |
|------|----------|---------|
| `init_state` (`ServicingInitState`) | `new_underhill_vm()` | Passed into VM construction. Each field seeds a specific subsystem: `firmware_type` → firmware selection, `emuplat` → RTC/PCI-bridge/NetVSP restore, `vmgs` → VMGS thin-client reopen, `nvme_state` → NVMe reconnect, `dma_manager_state` → DMA pool restore, `vmbus_client` → VMBus client reconnect, `mana_state` → MANA NIC restore, `overlay_shutdown_device` → shutdown IC routing. |
| `units` (`Vec<SavedStateUnit>`) | `LoadedVm::restore_units()` | Fed back into `state_units.restore()` which dispatches each blob to the matching state unit by name (chipset devices, VMBus channels, partition, vmtime, etc.). |

No kernel component ever interprets these fields.

---

## 1. The ServicingState Blob

Defined in
[openhcl/underhill_core/src/servicing.rs](openhcl/underhill_core/src/servicing.rs).

The blob is created in
[`LoadedVm::save()`](openhcl/underhill_core/src/dispatch/mod.rs)
by serializing the `ServicingState` struct with `mesh::payload::encode`.

`ServicingState` has two top-level fields:

```
ServicingState
├── init_state: ServicingInitState   // boot/restore setup + subsystem state
└── units: Vec<SavedStateUnit>       // state-unit graph snapshots
```

### 1.1 `init_state` — `ServicingInitState`

These fields live outside the state-unit graph and are collected directly
in `save()`.

| Field | Type | Description | Condition |
|-------|------|-------------|-----------|
| `firmware_type` | `Firmware` (enum: Uefi, Pcat, None) | Which firmware mode VTL 0 booted with. | Always |
| `vm_stop_reference_time` | `u64` | Hypervisor reference time (100 ns ticks) when VPs and state units were stopped. | Always |
| `correlation_id` | `Option<Guid>` | Tracing correlation ID for the servicing event. Set before sending; `None` during initial construction. | Always (set later) |
| `emuplat` | `EmuplatSavedState` | Emuplat glue state (see below). | Always |
| `flush_logs_result` | `Option<FlushLogsResult>` | Duration and error, if any, from the log-flush call near the end of save. Set after save by the caller. | Always (set later) |
| `vmgs` | `Option<(SavedVmgsState, SavedBlockStorageMetadata)>` | VMGS thin-client state + disk metadata. | When VMGS is configured |
| `overlay_shutdown_device` | `bool` | Whether the host shutdown IC is intercepted/overlaid by Underhill. | Always |
| `nvme_state` | `Option<NvmeSavedState>` | NVMe manager saved state (wraps `NvmeManagerSavedState`). | When NVMe controllers exist **and** keepalive is enabled |
| `dma_manager_state` | `Option<OpenhclDmaManagerState>` | DMA manager saved state (shared + private page pools). | When NVMe **or** MANA keepalive is enabled |
| `vmbus_client` | `Option<vmbus_client::SavedState>` | VMBus client connection state, channels, GPADLs, pending messages. | When VMBus client exists |
| `mana_state` | `Option<Vec<ManaSavedState>>` | Per-adapter MANA network device state (GDMA queues, DMA mappings). | When MANA NICs exist **and** keepalive is enabled |

#### `EmuplatSavedState` sub-fields

| Field | Type | Description |
|-------|------|-------------|
| `rtc_local_clock` | RTC local-clock saved state | The local clock offset used by the emulated RTC. |
| `get_backed_adjust_gpa_range` | `Option<GetBackedAdjustGpaRange saved state>` | State for GPA range adjustments by the 440BX host-PCI bridge (PCAT only). |
| `netvsp_state` | `Vec<netvsp::SavedState>` | Per-NIC NetVSP runtime state accumulated during the VM's lifetime. |

#### `NvmeSavedState` sub-fields

| Field | Type | Description |
|-------|------|-------------|
| `nvme_state.cpu_count` | `u32` | Number of CPUs at save time. |
| `nvme_state.nvme_disks` | `Vec<NvmeSavedDiskConfig>` | Per-disk PCI ID + NVMe driver saved state (queues, completions). |

#### `OpenhclDmaManagerState` sub-fields

| Field | Type | Description |
|-------|------|-------------|
| `shared_pool` | `Option<PagePoolState>` | Shared DMA page pool state. |
| `private_pool` | `Option<PagePoolState>` | Private DMA page pool state. |

#### `ManaSavedState` (per adapter)

| Field | Type | Description |
|-------|------|-------------|
| `pci_id` | `String` | PCI device identifier. |
| `mana_device` | `ManaDeviceSavedState` | GDMA driver state: memory, EQ, CQ, RQ, SQ. |

#### `vmbus_client::SavedState`

| Field | Type | Description |
|-------|------|-------------|
| `client_state` | `ClientState` (Disconnected or Connected { version, feature_flags }) | VMBus protocol connection state. |
| `channels` | `Vec<Channel>` | Active VMBus channels and their state. |
| `gpadls` | `Vec<Gpadl>` | Guest Physical Address Descriptor Lists. |
| `pending_messages` | `Vec<PendingMessage>` | Queued VMBus messages not yet delivered. |

#### `SavedVmgsState`

| Field | Type | Description |
|-------|------|-------------|
| `active_header_index` | `usize` | Which VMGS header is active. |
| `active_header_sequence_number` | `u32` | Sequence number of the active header. |
| `version` | `u32` | VMGS format version. |
| `fcbs` | `Vec<(u32, SavedResolvedFileControlBlock)>` | File control blocks (key → FCB). |
| `encryption_algorithm` | `u16` | Encryption algorithm in use. |
| `datastore_key_count` | `u8` | Number of datastore keys. |
| `active_datastore_key_index` | `Option<usize>` | Which key is active. |
| `datastore_keys` | `[SavedVmgsDatastoreKey; 2]` | The two datastore encryption keys. |
| `metadata_key` | `SavedVmgsDatastoreKey` | Metadata encryption key. |
| `encrypted_metadata_keys` | `[SavedVmgsEncryptionKey; 2]` | Encrypted metadata key blobs. |
| `reprovisioned` | `bool` | Whether the VMGS was reprovisioned. |

#### `FlushLogsResult`

| Field | Type | Description |
|-------|------|-------------|
| `duration_us` | `u64` | Time taken to flush logs, in microseconds. |
| `error` | `Option<String>` | Error message, if the flush failed. |

---

### 1.2 `units` — State-Unit Graph Snapshots

`units` is a `Vec<SavedStateUnit>`, where each entry has a `name: String` and
a `state: SavedStateBlob` (opaque, per-unit serialized state). The type is
defined in
[vmm_core/state_unit/src/lib.rs](vmm_core/state_unit/src/lib.rs).

All running state units are collected by `state_units.save()`. The set of
units present depends on the VM's configuration.

#### 1.2.1 Top-level units

Registered directly in
[openhcl/underhill_core/src/worker.rs](openhcl/underhill_core/src/worker.rs).

| Unit name | Description | Dependencies |
|-----------|-------------|--------------|
| `vmtime` | VM time keeper (reference time, VM time offset). | — |
| `input` | Input distributor (keyboard/mouse multiplexer). | — |
| `vmbus` | VMBus server (channel management, interrupts). | — |
| `vmbus_relay` | Host VMBus relay (relays host VMBus to guest). | `vmbus` |
| `partition` | Partition unit (VP register state, processor context). | `chipset`, `vmtime` |

#### 1.2.2 Chipset units

Registered via `builder.arc_mutex_device("name")` in
[vmm_core/vmotherboard/src/base_chipset.rs](vmm_core/vmotherboard/src/base_chipset.rs).
Each device unit is a dependency of the root `chipset` unit.

| Unit name | Description | When present |
|-----------|-------------|--------------|
| `chipset` | Root chipset unit (coordinates all chipset devices). | Always |
| `pic` | i8259 PIC (dual PIC cascade). | x86, PCAT |
| `ioapic` | I/O APIC. | x86 |
| `pci_bus` | Generic PCI bus (Gen2/UEFI). | Gen2 |
| `piix4-pci-bus` | PIIX4 PCI bus. | PCAT |
| `440bx-host-pci-bridge` | 440BX host-PCI bridge. | PCAT |
| `dma` | i8237 DMA controller. | PCAT |
| `piix4-pci-isa-bridge` | PIIX4 PCI-to-ISA bridge. | PCAT |
| `piix4-usb-uhci-stub` | USB UHCI stub device. | PCAT |
| `pit` | i8254 Programmable Interval Timer. | PCAT |
| `floppy` | Generic ISA floppy controller. | Feature-gated |
| `floppy-sio` | Winbond Super-IO + floppy controller. | Feature-gated |
| `ide` | IDE controller (PIIX4). | PCAT |
| `rtc` | MC146818A RTC + CMOS. | Gen2 |
| `piix4-rtc` | PIIX4-flavored RTC + CMOS. | PCAT |
| `pm` | Power management (Gen2). | Gen2 |
| `piix4-pm` | PIIX4 power management. | PCAT |
| `guest-watchdog` | Guest watchdog timer. | When configured |
| `uefi` | UEFI platform device. | UEFI firmware |
| `pcat` | PCAT BIOS platform device. | PCAT firmware |
| `fb` | Framebuffer device. | When video is configured |
| `vga` | Hyper-V VGA device. | Feature-gated |
| `vga_proxy` | Underhill VGA proxy. | Feature-gated |
| *(dynamic)* | Additional devices from `device.name` via `add_dyn_device`. | Per config |

#### 1.2.3 VMBus channel device units

Registered via `offer_channel_unit`, `offer_simple_device_unit`, or
`offer_vmbus_device_handle_unit` in
[vmm_core/src/vmbus_unit.rs](vmm_core/src/vmbus_unit.rs).
All depend on the `vmbus` unit. Names follow the pattern
`"{interface_name}:{instance_id}"`.

| Name pattern | Interface | Source |
|--------------|-----------|--------|
| `net:{instance_id}` | NetVSP NIC | `offer_channel_unit` in worker.rs (MANA NICs) |
| `ide-accel:{instance_id}` | StorVSP IDE-accel | `offer_channel_unit` in worker.rs |
| `scsi:{instance_id}` | StorVSP SCSI | `offer_vmbus_device_handle_unit` via `controllers.vmbus_devices` |
| `video:{instance_id}` | Synthetic video | `vmbus_device_handles` (SynthVideoHandle) |
| `keyboard:{instance_id}` | Synthetic keyboard | `vmbus_device_handles` (SynthKeyboardHandle) |
| `mouse:{instance_id}` | Synthetic mouse | `vmbus_device_handles` (SynthMouseHandle) |
| `shutdown_ic:{instance_id}` | Shutdown integration component | `vmbus_device_handles` (when overlay is active) |
| Other ICs / GET / etc. | Various interface names | `offer_vmbus_device_handle_unit` |

#### 1.2.4 VPCI relay units (dynamic, added at runtime)

Registered via `chipset.add_dyn_device()` in
[vm/devices/pci/vpci_relay/src/lib.rs](vm/devices/pci/vpci_relay/src/lib.rs)
when VPCI devices arrive.

| Name pattern | Description |
|--------------|-------------|
| `assigned_device:vpci-{instance_id}` | Relayed PCI config-space device. Note: returns `SaveError::NotSupported`. |
| `vpci:{instance_id}` | VpciBus instance for the relayed device. |

### 1.3 Compatibility Fixups

Before sending the blob, `ServicingState::fix_pre_save()` runs to add
legacy compatibility data. Currently this rewrites `vmbus_relay` unit state
to include `vmbus_client` fields using
`vmbus_relay::legacy_saved_state::SavedState`, so that older paravisors
(e.g., release/2411) can restore. The `vmbus_client` field is then cleared
from `init_state`.

On the restore side, `ServicingState::fix_post_restore()` performs the
inverse: if `vmbus_client` is `None` (state from an older version), it
extracts client state from the legacy `vmbus_relay` blob and populates
`init_state.vmbus_client`.

Both methods are in
[openhcl/underhill_core/src/servicing.rs](openhcl/underhill_core/src/servicing.rs).

### 1.4 Blob Delivery

| Path | Mechanism | Code |
|------|-----------|------|
| **Normal (host-held)** | `send_servicing_state(state_buf)` via GET → host holds the blob → new instance receives it via `get_saved_state_from_host()`. | [dispatch/mod.rs](openhcl/underhill_core/src/dispatch/mod.rs) |
| **Kexec (in-guest)** | `write_servicing_state_to_persisted()` writes the blob to the `VTL2_PERSISTED_SERVICING_STATE` memory region → new instance reads via `read_servicing_state_from_persisted()`. | [loader/vtl2_config/mod.rs](openhcl/underhill_core/src/loader/vtl2_config/mod.rs) |

---

## 2. Persistent Memory Regions

These are reserved physical memory ranges that survive the VTL 2 kernel
restart. Their locations are defined in the device tree (FDT) parsed by
`bootloader_fdt_parser`, and the boot shim (`openhcl_boot`) splits the
persisted region into three sub-regions.

Defined in
[openhcl/openhcl_boot/src/memory.rs](openhcl/openhcl_boot/src/memory.rs)
and
[vm/loader/loader_defs/src/shim.rs](vm/loader/loader_defs/src/shim.rs).

### 2.1 Region Layout

| Region | `MemoryVtlType` constant | Content |
|--------|--------------------------|---------|
| Header (1 page) | `VTL2_PERSISTED_STATE_HEADER` | `PersistedStateHeader` struct |
| Protobuf | `VTL2_PERSISTED_STATE_PROTOBUF` | Serialized topology `SavedState` |
| Servicing state | `VTL2_PERSISTED_SERVICING_STATE` | Full `ServicingState` blob (kexec only) |

### 2.2 `PersistedStateHeader`

A fixed C-repr struct at the start of VTL 2 memory. This struct is **never
changed** — new data goes into the protobuf payload.

| Field | Type | Description |
|-------|------|-------------|
| `magic` | `u64` | `"OHCLPHDR"` in ASCII. Indicates the header is valid. |
| `protobuf_base` | `u64` | GPA of the protobuf region (4 K aligned). |
| `protobuf_region_len` | `u64` | Size of the protobuf region in bytes. |
| `protobuf_payload_len` | `u64` | Size of the actual protobuf payload (≤ region len). |
| `servicing_state_base` | `u64` | GPA of the servicing state region (4 K aligned). 0 = none stored. |
| `servicing_state_region_len` | `u64` | Size of the servicing state region in bytes. |
| `servicing_state_payload_len` | `u64` | Size of the servicing state payload (≤ region len). |

### 2.3 Protobuf Topology Data (`SavedState`)

Written by `write_persisted_info()` in
[openhcl/underhill_core/src/loader/vtl2_config/mod.rs](openhcl/underhill_core/src/loader/vtl2_config/mod.rs).
This data lets the new VTL 2 instance reconstruct the partition memory and
MMIO layout without re-querying the host.

| Field | Type | Description |
|-------|------|-------------|
| `partition_memory` | `Vec<MemoryEntry>` | All memory ranges for the partition. |
| `partition_mmio` | `Vec<MmioEntry>` | All MMIO ranges for the partition. |
| `cpus_with_mapped_interrupts_no_io` | `Vec<u32>` | vCPU IDs that had mapped device interrupts but no outstanding I/O at save time. Used for sidecar-disable heuristics on restore. |
| `cpus_with_outstanding_io` | `Vec<u32>` | vCPU IDs that had outstanding I/O at save time. |

#### `MemoryEntry`

| Field | Type | Description |
|-------|------|-------------|
| `range` | `MemoryRange` | Start + length of the memory region. |
| `vnode` | `u32` | NUMA vnode for this range. |
| `vtl_type` | `MemoryVtlType` | VTL usage (VTL0, VTL2, persisted, etc.). |
| `igvm_type` | `IgvmMemoryType` | IGVM memory-map entry type as reported by the host. |

#### `MmioEntry`

| Field | Type | Description |
|-------|------|-------------|
| `range` | `MemoryRange` | Start + length of the MMIO region. |
| `vtl_type` | `MemoryVtlType` | VTL MMIO type (VTL0_MMIO or VTL2_MMIO). |

#### `VPInterruptState` (input to `write_persisted_info`)

Computed by `nvme_interrupt_state()` in
[openhcl/underhill_core/src/nvme_manager/save_restore_helpers.rs](openhcl/underhill_core/src/nvme_manager/save_restore_helpers.rs).

| Field | Description |
|-------|-------------|
| `vps_with_mapped_interrupts_no_io` | vCPUs with NVMe device interrupts that had no outstanding I/O. |
| `vps_with_outstanding_io` | vCPUs that had outstanding NVMe I/O at save time. |

### 2.4 Persisted Servicing State (kexec path only)

Written by `write_servicing_state_to_persisted()`, read by
`read_servicing_state_from_persisted()`. Both in
[openhcl/underhill_core/src/loader/vtl2_config/mod.rs](openhcl/underhill_core/src/loader/vtl2_config/mod.rs).

This region holds the exact same `mesh::payload::encode`d `ServicingState`
blob described in Section 1. On read, the header's servicing-state fields are
cleared and the region is zeroed to prevent stale reads on a subsequent
non-kexec boot.

### 2.5 Hardware Keepalive (Not Persisted, But Kept Alive)

When NVMe or MANA keepalive is enabled, the actual hardware devices are **not
torn down** across the VTL 2 restart. DMA mappings, queues, and interrupt
configurations remain live in the device. The saved state fields
(`nvme_state`, `dma_manager_state`, `mana_state`) describe how to
**reconnect** to these live devices — they do not capture the full device
state.

PCI devices that are not kept alive are unbound by
`pci_shutdown::shutdown_pci_devices()` during teardown.
