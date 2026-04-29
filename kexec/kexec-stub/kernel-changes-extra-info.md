## BUG 1

---

## The Register Page: What It Is

The **register page** (`HV_REGISTER_REG_PAGE`) is a performance optimization for VTL2 intercept handling. Here's the full data flow:

### Normal VTL intercept flow (without register page):

```
Guest VTL0 hits intercept (e.g., MMIO access)
  → Hypervisor traps to VTL2
  → VTL2 kernel dispatches the VP (mshv_vtl ioctl returns to Underhill)
  → Underhill needs guest registers (RAX, RIP, etc.)
  → Underhill calls get_vp_registers hypercall          ← EXPENSIVE
  → Underhill emulates the intercept
  → Underhill calls set_vp_registers hypercall          ← EXPENSIVE
  → Underhill resumes VTL0 (ioctl back into kernel → hv_call_dispatch_vp)
```

Each `get_vp_registers` / `set_vp_registers` is a full hypercall (~microseconds each). For hot-path intercepts (MMIO, port I/O), this adds up quickly.

### Optimized flow (with register page):

```
Guest VTL0 hits intercept
  → Hypervisor traps to VTL2
  → Hypervisor WRITES guest GP registers, RIP, RFLAGS, segments
    directly into the register page at the configured physical address
  → Hypervisor sets reg_page.is_valid = 1
  → VTL2 kernel dispatches the VP
  → Underhill reads registers directly from the mmap'd register page  ← FAST (memory read)
  → Underhill modifies registers in the register page, sets dirty bits
  → Underhill resumes VTL0
  → flush_register_page() commits dirty regs via set_vp_registers    ← only for modified regs
  → hv_call_dispatch_vp runs, hypervisor reads reg page for updated values
```

The struct is **4KB** (one page), containing:
- `is_valid` flag (hypervisor sets this)
- `dirty` bitmask (Underhill sets this when modifying)
- `vtl` (which VTL the intercept came from)
- RAX through R15, RIP, RFLAGS
- XMM0–XMM5
- ES, CS, SS, DS, FS, GS segment registers
- CR0, CR3, CR4, CR8, EFER, DR7
- Pending interruption state

### Setup flow (what the old kernel did):

```c
// In mshv_vtl_configure_reg_page():
reg_page = alloc_page(GFP_KERNEL);              // allocate a physical 4K page
overlay.pfn = page_to_hvpfn(reg_page);          // get its physical frame number
overlay.enabled = 1;
reg_assoc.name = HV_REGISTER_REG_PAGE;
reg_assoc.value.reg64 = overlay.as_uint64;
hv_call_set_vp_registers(HV_VP_INDEX_SELF, ...); // tell hypervisor: "write here"
```

This says: "Hypervisor, for this VP, every time you dispatch it to VTL2, write the guest's registers to **physical page 0x12345**."

---

## What Happens After Kexec

### Step 1: Old kernel sets up register pages

```
VP0: register page at physical address 0xA000_0000
VP1: register page at physical address 0xA000_1000
VP2: register page at physical address 0xA000_2000
...
VP31: register page at physical address 0xA001_F000
```

The hypervisor records these in its internal VP state. These persist across kexec — the hypervisor has no concept of "kexec happened."

### Step 2: Kexec executes, new kernel boots

The new kernel's memory allocator (`page_alloc`) has no knowledge of the old kernel's allocations. Those physical pages (0xA000_0000, etc.) are now part of the **free page pool**. The buddy allocator will hand them out to anyone requesting memory.

### Step 3: New kernel allocates memory at those physical addresses

```
kmalloc(sizeof(struct task_struct))  →  happens to return a virtual address
                                        backed by physical page 0xA000_2000
                                        (which is VP2's stale register page!)
```

Now a `task_struct` lives at the same physical address the hypervisor thinks is VP2's register page.

### Step 4: VP2 wakes up (HVCALL_START_VP) and gets dispatched

The hypervisor dispatches VP2 to VTL2. As part of dispatch, it writes **564 bytes** of register state to physical address 0xA000_2000:

```
offset 0x00: version = 1
offset 0x02: is_valid = 1
offset 0x04: dirty = 0
offset 0x08: rax = <guest_rax_value>
offset 0x10: rcx = <guest_rcx_value>
...
offset 0x90: rip = <guest_rip>
offset 0x98: rflags = <guest_rflags>
offset 0xA0: xmm0 = ...
...
```

This **overwrites the task_struct** that lives at that physical page.

### Step 5: Scheduler uses the corrupted task_struct

```c
struct task_struct {
    // ... many fields ...
    const struct cpumask *cpus_ptr;  // at some offset within the struct
    // ...
};
```

The hypervisor wrote `0x0000000000000000` (guest register values) over the `cpus_ptr` field. When the scheduler runs:

```c
try_to_wake_up(task) {
    cpumask_test_cpu(cpu, task->cpus_ptr);  // cpus_ptr is now NULL!
    → _find_first_bit(NULL, ...)
    → BUG: kernel NULL pointer dereference, address: 0x0000000000000000
}
```

### The Corruption Is Non-Deterministic

The exact crash depends on:
- **What** got allocated at the stale physical page (task_struct, page table, inode, slab metadata...)
- **When** the hypervisor writes (every VP dispatch to VTL2)
- **Which registers** happen to be zero (guest RAX=0 → writes 0x00 → looks like NULL pointer)

This makes debugging extremely difficult — the crash call trace gives no hint that the hypervisor is the culprit. The stack trace (`try_to_wake_up → sched_balance → sysvec_hyperv_stimer0 → mshv_vtl_idle`) just shows the scheduler was running when it tried to access the corrupted data.

### Why It's Specifically the Scheduler

The timing works out because:
1. SMP bringup starts VPs at ~0.63s into boot
2. The hypervisor immediately starts writing to stale register pages
3. The scheduler is one of the first subsystems to allocate task_structs for the new CPUs
4. These task_structs happen to land on the now-corrupted physical pages
5. Within milliseconds, the scheduler tries to use `cpus_ptr` → NULL deref

### The Fix

```c
// Before HVCALL_START_VP:
hv_call_set_vp_registers(vp_index, ...,
    HV_REGISTER_REG_PAGE = 0);  // "stop writing to that address"
```

Setting the register page to 0 tells the hypervisor: "this VP has no register page overlay." The hypervisor stops writing to the stale physical address. When `mshv_vtl` initializes later, it allocates a **fresh** page and sets it up properly.


//////////////////////////////////////////////////////////////////////////////////

## BUG 2:

## SINT0: What It Is

The Hyper-V **SynIC (Synthetic Interrupt Controller)** provides 16 **SINTs (Synthetic Interrupt Sources)**, SINT0 through SINT15. Each SINT is configured via an MSR:

```
MSR address = HV_MSR_SINT0 + index
```

Each SINT register has these fields:
```
union hv_synic_sint {
    u64 as_uint64;
    struct {
        u64 vector   : 8;    // IDT vector to inject when this SINT fires
        u64 reserved : 8;
        u64 masked   : 1;    // if 1, interrupt is suppressed
        u64 auto_eoi : 1;    // if 1, auto-EOI (no need to ack)
        ...
    };
};
```

When the hypervisor wants to signal a SINT, it **injects an interrupt at the configured vector** into the VP. The CPU then looks up that vector in the IDT and jumps to the corresponding ISR.

### SINT assignments in VTL2

| SINT | Index | Purpose | Who configures it |
|---|---|---|---|
| SINT0 | 0 (`HV_SYNIC_INTERCEPTION_SINT_INDEX`) | **VTL intercept notifications** — hypervisor signals VTL2 that a lower-VTL intercept needs handling | mshv_vtl_main.c |
| SINT2 | 2 (`VMBUS_MESSAGE_SINT`) | VMBus messages and events | `vmbus_drv.c` |
| SINT7 | 7 (`VTL2_VMBUS_SINT_INDEX`) | VTL2-specific VMBus | mshv_vtl_main.c |

### How SINT0 works in normal operation

```
                                    Hypervisor
                                        │
  VTL0 guest hits MMIO ─────────────────┤
                                        │
                                        ▼
                              Hypervisor intercepts,
                              needs VTL2 to emulate
                                        │
                                        ▼
                              Signal SINT0 on the VP
                                        │
                                        ▼
                              Read SINT0 MSR:
                              vector=0xF3, masked=0
                                        │
                                        ▼
                              Inject interrupt vector 0xF3
                              into VP (in VTL2 context)
                                        │
                                        ▼
                    CPU's IDT[0xF3] → sysvec_hyperv_callback()
                                        │
                                        ▼
                              mshv_handler() / vmbus_handler()
                                        │
                                        ▼
                              Read intercept message from
                              synic_message_page[SINT0]
                                        │
                                        ▼
                              Wake up the mshv_vtl thread
                              to handle the intercept in userspace
```

### How the old kernel configured SINT0

In `hv_vtl_setup_synic()` (called per-CPU during `mshv_vtl_init`):

```c
sint.as_uint64 = 0;
sint.vector = vmbus_interrupt;     // = HYPERVISOR_CALLBACK_VECTOR = 0xF3
sint.masked = false;               // enable delivery
sint.auto_eoi = hv_recommend_using_aeoi();

hv_set_msr(HV_MSR_SINT0 + HV_SYNIC_INTERCEPTION_SINT_INDEX, sint.as_uint64);
```

This tells the hypervisor: "For this VP, when you need to signal a VTL intercept, inject vector 0xF3."

---

## What Happens After Kexec

### Step 1: Old kernel's SINT0 state persists

When the old kernel calls kexec, it runs `hv_kexec_handler()` which cleans up **SINT2** (VMBus) but **NOT SINT0** (interception). The per-VP MSR state remains:

```
For every VP:
  SINT0 MSR = { vector: 0xF3, masked: false, auto_eoi: true }
```

This persists in the hypervisor's VP state across kexec — the hypervisor doesn't know a new kernel started.

### Step 2: New kernel starts, installs fresh IDT

The new kernel builds its IDT from scratch. Early in boot, `sysvec_install(HYPERVISOR_CALLBACK_VECTOR, sysvec_hyperv_callback)` installs the handler at vector 0xF3. **However**, the `mshv_handler` callback pointer is still NULL — it won't be set until `mshv_vtl_init` runs much later.

But here's the critical issue: it's even worse for APs. During SMP bringup, the AP's IDT may not have all entries populated yet, or the AP may be in a very early initialization state where interrupt handling isn't fully set up.

### Step 3: VP wakes up, hypervisor delivers SINT0 interrupt

When `HVCALL_START_VP` wakes an AP, the hypervisor may immediately have a pending intercept (or the VTL0 guest may immediately trigger one). The hypervisor reads the stale SINT0 configuration:

```
SINT0: vector=0xF3, masked=false → inject interrupt 0xF3 into the VP
```

The interrupt arrives on the AP while it's still in early CPU bringup. Several things can go wrong:

### Failure Mode 1: No handler registered

If the interrupt arrives before `mshv_handler` is set, `sysvec_hyperv_callback` runs but does nothing useful:

```c
DEFINE_IDTENTRY_SYSVEC(sysvec_hyperv_callback)
{
    if (mshv_handler)     // NULL — not set yet!
        mshv_handler();
    if (vmbus_handler)    // NULL — not set yet!
        vmbus_handler();
    apic_eoi();           // EOI the interrupt, but the message is lost
}
```

The hypervisor had a pending intercept message in the SynIC message page. But the old kernel's message page addresses are stale (same problem as register page). So the handler either reads garbage or the message slot is never consumed. The hypervisor sees the message slot as "still occupied" and **stops delivering further SINT0 interrupts to this VP**.

The VP appears stuck — it can't process intercepts, so VTL0 can't make progress, and any CPU hotplug state machine waiting for this VP's cooperation hangs.

### Failure Mode 2: Spurious interrupt during early AP init

If the SINT0 interrupt fires during a very early phase of AP startup — before the per-CPU interrupt infrastructure is ready — the CPU may:
- Triple-fault (if IDT isn't loaded yet)
- Execute the wrong handler (if the vector collides with something else during early init)
- Corrupt per-CPU state by running interrupt code before the CPU's GS base / per-CPU area is set up

### The symptom you observed

```
cpuhp_setup_state hangs waiting for CPU 18
```

This is the CPU hotplug state machine. It's trying to bring CPU 18 to a specific state (e.g., `CPUHP_AP_ONLINE`). The protocol is:

```
BSP (CPU 0)                                    AP (CPU 18)
    │                                              │
    ├── cpuhp: "bring CPU 18 to state X"           │
    │                                              │
    ├── signal CPU 18 to advance ──────────────────►│
    │                                              │── receives SINT0 interrupt
    ├── wait for CPU 18 to report "done"           │   (stale, from hypervisor)
    │   ...                                        │── CPU is stuck handling
    │   ...                                        │   the spurious interrupt
    │   ...                                        │   or its intercept pipeline
    │   ...                                        │   is jammed
    │   TIMEOUT / HANG                             │
```

CPU 18 never reports back because it's either:
- Stuck in an interrupt loop (SINT0 keeps firing because the message slot is never properly consumed)
- Wedged because its `mshv_vtl` per-CPU state isn't initialized yet when the interrupt arrives

### Why it's CPU 18 specifically

It's non-deterministic — it could be any AP. CPU 18 just happened to be the one where the timing was worst: the SINT0 interrupt arrived at exactly the wrong moment during that VP's hotplug state transition. Other VPs got lucky — they either processed the stale interrupt harmlessly or didn't receive one during the critical window.

---

## The Fix

```c
// Before HVCALL_START_VP for each AP:
sint.as_uint64 = 0;
sint.masked = true;          // suppress all SINT0 delivery
regs[1].name = 0x000A0000;   // HV_REGISTER_SINT0
regs[1].value.reg64 = sint.as_uint64;

hv_call_set_vp_registers(vp_index, ..., 2, vtl, regs);
```

Masking SINT0 before starting the VP tells the hypervisor: "Do not deliver interception interrupts to this VP." Any pending intercept notifications are held until `mshv_vtl_init` later configures SINT0 properly with:
- A valid vector pointing to a registered IDT handler
- A valid SynIC message page to receive the intercept details
- The `mshv_handler` callback installed to process them

The VP starts cleanly, does its early init without spurious interrupts, and SINT0 gets unmasked only when everything is ready to handle it.