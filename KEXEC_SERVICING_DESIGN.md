# Kexec-Based Servicing Flow for ARM64

## Goal
Test full servicing flow using kexec instead of host-initiated lazy boot:
- Save VTL2 state before kexec
- Boot new VTL2 kernel
- Restore state and resume VTL0 without interruption

## Architecture

### Memory Layout for Servicing State

We'll use a **reserved memory region** that survives kexec:

```
Physical Memory Layout (ARM64):
┌────────────────────────────────────────┐
│ VTL0 Memory (Guest OS)                 │
├────────────────────────────────────────┤
│ VTL2 Config Pages (measured)           │ ← Page 0x9b89000 (from your previous logs)
│  - Measured VTL2 Config                │
│  - VTL0 Config                         │
├────────────────────────────────────────┤
│ **Servicing State Buffer** (NEW)       │ ← Reserved 16MB region
│  - ServicingState serialized data      │
│  - Magic marker: "OHCLSERV"            │
│  - Size field                          │
│  - Checksum                            │
├────────────────────────────────────────┤
│ VTL2 Kernel & Runtime                  │
│ OpenHCL Boot Shim                      │
│ Initramfs                              │
└────────────────────────────────────────┘
```

### Servicing State Structure

```rust
// New structure in loader_defs or underhill_core
#[repr(C)]
struct KexecServicingHeader {
    magic: [u8; 8],        // "OHCLSERV"
    version: u32,          // Format version
    state_size: u32,       // Size of serialized ServicingState
    checksum: u64,         // CRC64 of state_size bytes following header
    reserved: [u8; 40],    // Future expansion
}

// Followed by:
// - Serialized ServicingState (mesh::payload format)
```

### Flow

#### **Phase 1: Pre-Kexec (Current VTL2)**

1. **Trigger State Save via Diagnostic Interface**
   ```
   User/Script → diag_client → underhill_core
                                    ↓
                            save() → ServicingState
                                    ↓
                            Write to reserved memory:
                            - Header with magic
                            - Serialized state
                            - Checksum
   ```

2. **Mark VTL2 Config for Servicing**
   - Set a flag in the measured VTL2 config page
   - Indicates next boot should restore state

3. **Execute Kexec**
   - New kernel + initramfs loaded
   - Reserved memory regions preserved
   - Kexec with special cmdline

#### **Phase 2: Post-Kexec (New VTL2)**

1. **Boot Detection**
   ```rust
   fn detect_kexec_servicing(gm: &GuestMemory) -> Option<ServicingState> {
       // Check reserved memory for magic marker
       let header = read_header_from_reserved_region()?;
       
       if header.magic != b"OHCLSERV" {
           return None; // Normal boot
       }
       
       // Validate checksum
       verify_checksum(&header)?;
       
       // Deserialize state
       let state: ServicingState = mesh::payload::decode(
           &read_state_bytes(header.state_size)
       )?;
       
       Some(state)
   }
   ```

2. **Standard Servicing Restore**
   - Same path as host-initiated servicing
   - `LoadKind::None` (don't reload VTL0)
   - Restore device states
   - Resume VTL0

## Implementation Steps

### Step 1: Add Diagnostic Command for State Save
```rust
// In openhcl/underhill_core/src/lib.rs
pub enum ControlRequest {
    FlushLogs(Rpc<CancelContext, Result<(), CancelReason>>),
    SaveForKexec(Rpc<CancelContext, Result<KexecServicingInfo, String>>),  // NEW
}

pub struct KexecServicingInfo {
    pub state_address: u64,     // Physical address of saved state
    pub state_size: usize,      // Size in bytes
    pub checksum: u64,          // For verification
}
```

### Step 2: Reserve Memory Region in Boot Shim
```rust
// In openhcl/openhcl_boot/src/dt.rs
// Add a new reserved memory region for kexec servicing state

const KEXEC_SERVICING_REGION_SIZE: u64 = 16 * 1024 * 1024; // 16MB

fn reserve_kexec_servicing_memory(
    dt: &mut DeviceTree,
    base_addr: u64,
) -> Result<(), Error> {
    // Reserve region that kexec won't touch
    dt.add_reserved_memory(
        "kexec-servicing-state",
        MemoryRange::new(base_addr..base_addr + KEXEC_SERVICING_REGION_SIZE),
        Some(ReservedMemoryType::KexecServicingState),
    )?;
    Ok(())
}
```

### Step 3: Kexec Script Integration
```bash
#!/bin/sh
# Pre-kexec: Trigger state save

# 1. Request state save via ohcldiag-dev
echo "[KEXEC] Requesting servicing state save"
echo "save_for_kexec" > /sys/kernel/debug/underhill/control

# 2. Read back state info
STATE_ADDR=$(cat /sys/kernel/debug/underhill/kexec_state_addr)
STATE_SIZE=$(cat /sys/kernel/debug/underhill/kexec_state_size)

echo "[KEXEC] Servicing state saved:"
echo "  Address: $STATE_ADDR"
echo "  Size: $STATE_SIZE bytes"

# 3. Add to kexec command line
CMDLINE="$CMDLINE OPENHCL_KEXEC_SERVICING=1"
CMDLINE="$CMDLINE OPENHCL_SERVICING_STATE_ADDR=$STATE_ADDR"

# 4. Execute kexec (state will be preserved in reserved region)
kexec -l /boot/Image --initrd=/tmp/initrd.gz --command-line="$CMDLINE"
kexec -e
```

### Step 4: Post-Kexec Detection
```rust
// In openhcl/underhill_core/src/worker.rs::new_or_restart()

// NEW: Check for kexec servicing before checking DPS
let kexec_servicing_state = detect_kexec_servicing(&gm)
    .context("failed to check for kexec servicing state")?;

if let Some(state) = kexec_servicing_state {
    tracing::info!(
        CVM_ALLOWED,
        "Detected kexec servicing, restoring state"
    );
    servicing_state = Some(state);
}

// Then continue with existing flow...
let saved_state_from_host = dps.general.is_servicing_scenario;
```

## Testing Plan

### Test 1: Basic Kexec Without Servicing
```bash
# Should start VTL0 fresh
kexec -l /boot/Image --initrd=/tmp/initrd.gz
kexec -e
# Expected: VTL0 boots from scratch
```

### Test 2: Kexec With Servicing
```bash
# Should preserve VTL0 state
./kexec_servicing_test_arm64.sh
# Expected: VTL0 continues running, no reboot visible to guest
```

### Test 3: State Corruption Handling
```bash
# Corrupt the saved state, should fall back to normal boot
# (or panic safely)
```

## Advantages of This Approach

1. **No Host Involvement** - Pure VTL2 operation
2. **ARM64 Native** - Uses device tree, no x86 baggage
3. **Sidecar Not Needed** - Already disabled on ARM64
4. **Realistic Testing** - Same code path as production servicing
5. **Debuggable** - State is in guest memory, can be inspected

## Potential Issues

1. **Memory Pressure** - 16MB reserved for state (acceptable for testing)
2. **State Compatibility** - New/old VTL2 versions must agree on format
3. **Checksum Validation** - Need robust error handling
4. **Device State** - Some devices may not handle kexec well (NVMe, network)

## Next Steps

1. Implement `ControlRequest::SaveForKexec`
2. Add memory reservation in boot shim
3. Create diagnostic interface for state save
4. Update kexec script
5. Add post-kexec detection logic
6. Test incrementally
