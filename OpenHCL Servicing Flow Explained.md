## OpenHCL Servicing Flow Explained

Based on worker.rs, here's how **servicing** (also called "live migration" or "VM restart") works in OpenVMM/OpenHCL:

### **1. What is Servicing?**

Servicing is the process of **updating/restarting the VTL2 (OpenHCL) paravisor** while keeping the VTL0 guest running. Think of it like hot-swapping the hypervisor layer without fully shutting down the VM.

### **2. Entry Points (How Servicing Starts)**

There are **two** ways servicing can be initiated:

#### **A. Host-Initiated Servicing** (`is_servicing_scenario = true`)
- The **host** (Azure/Hyper-V) triggers the update
- Saved state is stored **on the host** side
- Flow:
  1. VTL2 saves its state and sends it to the host via GET (Guest Emulation Transport)
  2. VTL2 shuts down
  3. New VTL2 binary boots up
  4. New VTL2 fetches saved state **from the host** via `get_saved_state_from_host()`
  5. VTL2 restores and resumes

#### **B. Local Servicing** (your kexec scenario)
- **VTL2 itself** initiates the update (e.g., via kexec)
- Saved state stays **in VTL2 memory** 
- Flow:
  1. VTL2 saves state to memory
  2. Kexec loads new kernel **in same address space**
  3. New VTL2 finds saved state in memory and restores

### **3. Key Code Flow in worker.rs**

```rust
async fn new_or_restart(
    ...
    boot_init: bool,               // false on servicing restart
    mut servicing_state: Option<ServicingState>,  // None on first boot, Some() on restart
    ...
) {
    // 1. Read DPS (Device Platform Settings) from host
    let dps = read_device_platform_settings(&get_client).await?;
    
    // 2. Check if this is host-initiated servicing
    let saved_state_from_host = dps.general.is_servicing_scenario;
    
    if saved_state_from_host {
        // Host has our state - fetch it
        let saved_state_buf = get_client
            .get_saved_state_from_host()
            .await?;
        
        servicing_state = Some(mesh::payload::decode(&saved_state_buf)?);
    }
    
    // 3. Fix up servicing state for compatibility
    if let Some(state) = &mut servicing_state {
        state.fix_post_restore()?;  // Handle version differences
    }
    
    let is_post_servicing = servicing_state.is_some();
    
    // 4. Build the VM with restored state
    let mut vm = new_underhill_vm(
        ...
        servicing_state: servicing_init_state,  // Contains firmware type, stop time, etc.
        ...
    ).await?;
}
```

### **4. What Gets Saved/Restored (`ServicingState`)**

From servicing.rs:

```rust
pub struct ServicingState {
    pub init_state: ServicingInitState,  // Core boot state
    pub units: Vec<SavedStateUnit>,      // Device/component states
}

pub struct ServicingInitState {
    pub firmware_type: Firmware,              // UEFI/PCAT/None
    pub vm_stop_reference_time: u64,         // When we paused
    pub correlation_id: Option<Guid>,        // For tracing
    pub emuplat: EmuplatSavedState,          // Emulation layer state
    pub vmgs: Option<...>,                   // VMGS storage state
    pub nvme_state: Option<NvmeSavedState>,  // NVMe controller state
    pub dma_manager_state: Option<...>,      // DMA buffer state
    pub vmbus_client: Option<...>,           // VMBus channel state
    // ... more device states
}
```

### **5. Critical Servicing Requirements**

#### **Memory Must Be Preserved:**
- VTL0 memory **stays untouched** (guest keeps running)
- VTL2 memory layout **must be compatible** (same addresses for critical structures)
- **Assist pages, hypercall pages, etc. must be valid**

#### **Device State Continuity:**
- All emulated devices (disk, network, TPM, etc.) save/restore their state
- VMBus channels must reconnect seamlessly
- DMA buffers must be preserved or remapped

#### **Firmware/Kernel Awareness:**
- The guest **never knows** servicing happened (except maybe timing)
- UEFI runtime services keep working
- Linux kernel VTL/VSM drivers see consistent state

### **6. Your Kexec Issue - Root Cause**

In your kexec scenario:

```
First Boot:
  ┌─────────────────────────────────────┐
  │ 1. Boot shim initializes VTL2       │
  │ 2. Sidecar starts (if enabled)      │
  │    - Sets up assist page            │
  │    - Triggers VTL transitions       │
  │    → entry_reason = INTERRUPT (2)   │
  │ 3. Kernel checks assist page ✓      │
  └─────────────────────────────────────┘

Kexec Boot (servicing=true, sidecar=off):
  ┌─────────────────────────────────────┐
  │ 1. Boot shim initializes VTL2       │
  │ 2. Sidecar SKIPPED (disabled)       │
  │    - Assist page NEVER set up       │
  │    → entry_reason = RESERVED (0)    │
  │ 3. Kernel checks assist page ✗      │
  │    PANIC: "unknown entry reason: 0" │
  └─────────────────────────────────────┘
```

### **7. Why `OPENHCL_FORCE_SERVICING` Doesn't Help**

Looking at the code, `OPENHCL_FORCE_SERVICING` is **not** a recognized flag in worker.rs or the boot shim. It's likely:
- A test/debug flag you added, or
- Checked in your custom kernel

The issue is **architectural**: the boot shim doesn't initialize the VTL2 assist page when sidecar is disabled, but your kernel **requires** it to be initialized before checking it.

### **8. The Real Solution**

You have **three options**:

#### **Option A: Fix Your Kernel** (Recommended)
Patch your OHCL-Linux-Kernel to handle `entry_reason == 0`:

```c
// In your kernel's VTL init code:
if (assist_page->vtl_control.entry_reason == HV_VTL_ENTRY_REASON_RESERVED) {
    // Servicing boot without sidecar - treat as normal interrupt entry
    assist_page->vtl_control.entry_reason = HV_VTL_ENTRY_REASON_INTERRUPT;
}
```

#### **Option B: Initialize Assist Page in Boot Shim**
Add VTL2 assist page setup to `openhcl_boot` even when sidecar is off (requires Rust changes)

#### **Option C: Keep Sidecar Enabled**
Let sidecar initialize (which sets up assist page) but have it exit early before allocating memory

---

**Summary:** Servicing is about **preserving running state** across VTL2 restarts. Your kexec scenario is a special case where you're doing servicing **without** a proper saved-state handoff, so you need to ensure all critical hypervisor structures (like the assist page) are initialized even when sidecar is disabled.

The panic you're seeing is because your **kernel expects initialized VTL state** that the **boot shim isn't providing** when sidecar is off.