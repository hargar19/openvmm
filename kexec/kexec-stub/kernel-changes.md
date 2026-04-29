# Kernel Changes for Kexec Stub

Commit `89cfd2f` on branch `user/hargar/kexec-2026` in OHCL-Linux-Kernel.

**2 files changed, 81 insertions, 2 deletions.**

```
 arch/x86/hyperv/hv_vtl.c   | 46 ++++++++++++++++++++++++++++++++++++
 drivers/hv/mshv_vtl_main.c | 37 +++++++++++++++++++++++++++--
```

---

## The Problem

After kexec, the hypervisor retains per-VP configuration from the old kernel. The new kernel has no knowledge of this state. Two pieces are dangerous:

### 1. Stale Register Page (`HV_REGISTER_REG_PAGE`)

The old kernel told the hypervisor: "for each VP, write intercept context to this physical address." After kexec, those physical addresses now belong to the new kernel's allocations (task structs, page tables, etc.). The hypervisor doesn't know a kexec happened — it keeps writing to those addresses every time it dispatches a VP to VTL2.

# What is "Intercept context"?
"Intercept context" = the guest VP's CPU register state at the moment it triggered an intercept.

When VTL0 does something that requires VTL2 emulation (MMIO access, port I/O, MSR read/write, CPUID, etc.), the hypervisor **intercepts** it — suspends the VTL0 VP and switches to VTL2 to handle it.

VTL2 needs to know what the guest was doing. For example:
- **MMIO write**: VTL2 needs RAX (the value being written), RIP (where to resume after emulation)
- **Port I/O `out dx, al`**: VTL2 needs RDX (port number), RAX (data), RCX (repeat count)
- **MSR write `wrmsr`**: VTL2 needs ECX (MSR index), EDX:EAX (value)

So the hypervisor **writes** the full register snapshot to the physical page:

```
Physical page 0xA000_2000 (configured as VP2's register page):

  offset 0x00: version, is_valid, dirty
  offset 0x08: RAX = 0x00000042        ← guest was writing 0x42
  offset 0x10: RCX = ...
  offset 0x18: RDX = ...
  ...
  offset 0x90: RIP = 0xfffff801234     ← instruction that caused the intercept
  offset 0x98: RFLAGS = ...
  offset 0xA0: XMM0-5
  offset 0x160: ES, CS, SS, DS, FS, GS
  offset 0x1C0: CR0, CR3, CR4, CR8, EFER, DR7
  offset 0x1F0: pending_interruption, interrupt_state
```

This is the "intercept context" — a snapshot of everything VTL2 needs to emulate what VTL0 was doing. Without the register page, VTL2 would have to issue individual `get_vp_registers` hypercalls (expensive) to read each register. The register page makes this a simple memory read from the pre-configured physical address.

**Symptom**: The scheduler crashes with a NULL pointer dereference — `try_to_wake_up()` finds `task->cpus_ptr == NULL` because the hypervisor zeroed/overwrote the task struct's memory through the stale register page.

```
BUG: kernel NULL pointer dereference, address: 0000000000000000
RIP: _find_first_bit+0x19/0x40   (RDI=NULL → cpumask pointer is NULL)
Call trace:
  try_to_wake_up → wake_up_process → cpu_stop_queue_work → stop_one_cpu_nowait
  → sched_balance_rq → sched_balance_domains → sched_balance_softirq
  → handle_softirqs → irq_exit_rcu → sysvec_hyperv_stimer0
  → mshv_vtl_idle
```

### 2. Stale SINT0 (Synthetic Interrupt Source 0)

The old kernel left SINT0 unmasked with a vector pointing to its interrupt handler. After kexec, no IDT entry exists for that vector. When the hypervisor delivers the interrupt, CPUs hang or crash.

**Symptom**: `cpuhp_setup_state` hangs waiting for a CPU (e.g., CPU 18) that's stuck handling a spurious interrupt with no registered handler.

The Hyper-V **SynIC (Synthetic Interrupt Controller)** is a per-VP interrupt routing mechanism that sits between the hypervisor and the VP's local APIC. It has 16 slots called **SINTs** (SINT0–SINT15).

Each SINT is configured via an MSR with three key fields:
- **vector** (8 bits): which IDT entry to invoke when this SINT fires
- **masked** (1 bit): if set, delivery is suppressed
- **auto_eoi** (1 bit): auto-acknowledge

**SINT0** specifically is the **interception SINT** (`HV_SYNIC_INTERCEPTION_SINT_INDEX = 0`). It's how the hypervisor tells VTL2: "a lower-VTL intercept happened on this VP and needs your attention."

The flow:
```
VTL0 guest triggers intercept (MMIO, port I/O, etc.)
  → Hypervisor suspends VTL0 VP
  → Hypervisor writes intercept message to SynIC message page[slot 0]
  → Hypervisor signals SINT0 on that VP
  → SINT0 config says: vector=0xF3, masked=false
  → Hypervisor injects interrupt vector 0xF3 into VP (in VTL2 context)
  → CPU jumps to IDT[0xF3] → sysvec_hyperv_callback()
  → Handler reads the intercept message, wakes Underhill thread to emulate
```

Other SINTs serve different purposes: SINT2 is VMBus messaging, SINT7 is VTL2-specific VMBus. Think of SINTs as "named doorbell channels" between the hypervisor and the guest kernel — each one signals a different class of event.

---

## Why This Wasn't a Problem Before

The old bzImage kexec path used `boot_cpus=0` on the command line, which booted **only VP0** during SMP init. Secondary VPs stayed dormant until the `mshv_vtl` driver explicitly started them later. At that point the driver configured fresh register pages and SINTs **before** calling `HVCALL_START_VP`. Stale state was overwritten before it could cause harm.

The stub path boots all 32 CPUs during SMP init (no `boot_cpus=0`). `hv_vtl_wakeup_secondary_cpu()` calls `HVCALL_START_VP` immediately — the hypervisor starts writing to stale register page addresses the moment each VP wakes. The corruption window is between SMP init (VPs start) and `mshv_vtl_init` (the driver configures proper state), which runs much later.

```
Timeline (vulnerable):

  SMP init                                    mshv_vtl_init
  ├── HVCALL_START_VP(VP1)                    ├── hv_vtl_setup_synic()
  │     └── VP1 wakes, hypervisor writes      │     └── configure register pages
  │         to STALE register page addr ⚠️    │         └── TOO LATE, damage done
  ├── HVCALL_START_VP(VP2)                    │
  │     └── VP2 wakes, same problem           │
  ...                                         │
  ├── "Brought up 1 node, 32 CPUs"            │
  │                                           │
  ├── kernel allocates memory at the old      │
  │   register page addresses                 │
  │   └── 💥 hypervisor overwrites them       │
  ...                                         ...
```

### Normal servicing (no kexec) — not affected

In host-driven servicing, the host tears down and recreates the Hyper-V partition entirely. All VPs start with clean hypervisor state — register pages are zero and SINTs are masked by default. No stale state exists.

### `boot_cpus=0` as an alternative

Using `boot_cpus=0` on the kexec command line avoids the need for kernel changes entirely — the `mshv_vtl` driver owns VP bringup and cleans up state before starting each VP. The kernel fix is preferred because it makes the all-CPUs boot path safe regardless of command line options.

---

## The Fix — Two Levels

### Level 1: Early cleanup in `hv_vtl.c` (prevents corruption)

New function `hv_vtl_cleanup_stale_vp_state(vp_index)` issues a single hypercall that sets two registers in one batch:

- `HV_REGISTER_REG_PAGE = 0` — disables the register page
- `SINT0 = masked` — prevents stale interrupts

```c
static void hv_vtl_cleanup_stale_vp_state(u32 vp_index)
{
    struct hv_register_assoc regs[2] = {};
    union hv_input_vtl vtl = { .as_uint8 = 0 };
    union hv_synic_sint sint;

    /* Disable stale register page */
    regs[0].name = HV_REGISTER_REG_PAGE;
    regs[0].value.reg64 = 0;

    /* Mask stale SINT0 (VTL interception SINT) */
    sint.as_uint64 = 0;
    sint.masked = true;
    regs[1].name = 0x000A0000; /* HV_REGISTER_SINT0 */
    regs[1].value.reg64 = sint.as_uint64;

    hv_call_set_vp_registers(vp_index, HV_PARTITION_ID_SELF,
                             2, vtl, regs);
}
```

Called at two points:

1. **Boot CPU (VP0)**: in `hv_vtl_early_init()` during `hyperv_init`, before any memory allocations that could be corrupted
2. **Each secondary VP**: in `hv_vtl_wakeup_secondary_cpu()`, immediately before `hv_vtl_bringup_vcpu()` / `HVCALL_START_VP`

This closes the corruption window entirely — stale state is cleaned before each VP ever starts.

### Level 2: Safety-net in `mshv_vtl_main.c` (defense-in-depth)

New function `mshv_vtl_cleanup_stale_state()` runs on every CPU via `on_each_cpu()` at the start of `hv_vtl_setup_synic()`, before any SynIC handlers are installed:

```c
static void mshv_vtl_cleanup_stale_state(void *info)
{
    union hv_synic_sint sint;
    struct hv_register_assoc reg_assoc = {};
    union hv_input_vtl vtl = { .as_uint8 = 0 };

    /* Mask stale SINT0 from previous kernel */
    sint.as_uint64 = hv_get_msr(HV_MSR_SINT0 + HV_SYNIC_INTERCEPTION_SINT_INDEX);
    if (!sint.masked) {
        sint.masked = true;
        hv_set_msr(HV_MSR_SINT0 + HV_SYNIC_INTERCEPTION_SINT_INDEX,
                   sint.as_uint64);
    }

    /* Disable stale register page */
    reg_assoc.name = HV_REGISTER_REG_PAGE;
    reg_assoc.value.reg64 = 0;
    hv_call_set_vp_registers(HV_VP_INDEX_SELF, HV_PARTITION_ID_SELF,
                             1, vtl, &reg_assoc);
}
```

Called as `on_each_cpu(mshv_vtl_cleanup_stale_state, NULL, 1)` in `hv_vtl_setup_synic()`.

This uses a different code path than Level 1 (per-CPU MSR reads vs. hypercall-by-VP-index), providing an additional safety layer.

---

## No-op on First Boot

Both fixes are harmless on first boot:

- Register page is already zero (hypervisor default)
- SINT0 is already masked (hypervisor default)

The hypercalls simply write the same values that are already set. No behavioral change for non-kexec scenarios.

---

## Why Both Levels Are Needed

Testing showed that fixing only one is insufficient:

| Fix applied | Result |
|---|---|
| SINT0 masking only (no register page cleanup) | Hang at CPU 17→18 during `cpuhp_setup_state` |
| Register page only (no SINT0 masking) | Spurious interrupts on uninitialized CPUs |
| Both | All 32 CPUs initialize successfully ✅ |

---

## What the Old Kernel's Kexec Handler Does NOT Clean Up

The existing `hv_kexec_handler()` only cleans VMBus SynIC state:

```
hv_kexec_handler()
  ├── hv_stimer_global_cleanup()
  ├── vmbus_initiate_unload(false)
  └── cpuhp_remove_state(hyperv_cpuhp_online)
        └── hv_synic_cleanup(cpu)
              ├── mask SINT2 (VMBUS_MESSAGE_SINT)
              ├── disable SIMP
              ├── disable SIEFP
              └── disable SCONTROL
```

Missing from this path:

| State | Why not cleaned | Effect after kexec |
|---|---|---|
| Register page (`HV_REGISTER_REG_PAGE`) | No mshv VTL kexec handler registered | Hypervisor writes to stale physical addresses |
| SINT0 (interception SINT) | `cpuhp` teardown callback is NULL | Unmasked with stale vector → spurious interrupts |
| Sidecar state | Not torn down during kexec | Stale from previous boot |

The `mshv_vtl` module registers no `hv_setup_kexec_handler()`, and its `cpuhp_setup_state()` call passes NULL as the teardown callback. Module exit (`mshv_vtl_exit()`) only runs on `rmmod`, never during kexec.

---

## Upstream Status and Upstreamability

The `mshv_vtl` driver was upstreamed ~4 months ago (`7bfe3b8ea6e30` "Drivers: hv: Introduce mshv_vtl driver" on `hyperv-next`). The upstream code has **zero kexec handling** for VTL-specific state — no `hv_setup_kexec_handler()`, no cpuhp teardown callback, no cleanup in the AP wakeup path.

### This is a real upstream bug, not specific to our use case

The stale VP state issue affects **anyone who does kexec in a Hyper-V VTL2 environment**. Evidence:

1. **Kexec already works in upstream VTL2** — nothing blocks it. `hv_machine_shutdown` (the kexec path) is registered unconditionally for all Hyper-V guests. VTL2's `hv_vtl_early_init()` only overrides `machine_ops.restart` and `.emergency_restart` (for triple-fault reboot), not `.shutdown`. There's no `CONFIG_KEXEC` guard specific to VTL mode.

2. **The upstream kexec cleanup chain has a gap**:

   | Step | What it cleans | Who registers it |
   |---|---|---|
   | `hv_kexec_handler()` | VMBus: stimers, SIMP, SIEFP, SCONTROL, SINT2 | `vmbus_drv.c` |
   | `cpuhp_remove_state(CPUHP_AP_HYPERV_ONLINE)` | VP Assist Pages | `mshyperv.c` |
   | **Nothing** | Register page, SINT0 | `mshv_vtl_main.c` — no handler |

3. **Reproducible without our stub**: Any upstream user who boots VTL2 with `mshv_vtl` loaded, does `kexec -e`, and boots all CPUs in the new kernel will hit the same crash. The kexec stub is irrelevant — it just happens to exercise the all-CPUs SMP boot path.

4. **The fix follows existing upstream patterns**:
   - `hv_machine_shutdown()` already calls `cpuhp_remove_state()` to tear down VP Assist Pages before kexec — analogous to our register page cleanup
   - `hv_vtl_bringup_vcpu()` already sets up full VP context before `HVCALL_START_VP` — adding cleanup before it is natural
   - The `on_each_cpu()` defense-in-depth pattern is used elsewhere in the kernel

### Upstream code examined (hyperv-next/hyperv-next)

```
arch/x86/hyperv/hv_vtl.c (283 lines):
  - hv_vtl_wakeup_secondary_cpu(): calls hv_vtl_bringup_vcpu() directly
    with NO cleanup of stale register page or SINT0
  - hv_vtl_early_init(): sets restart/emergency_restart only, no kexec hooks
  - No hv_setup_kexec_handler() call anywhere in the file

drivers/hv/mshv_vtl_main.c:
  - cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "hyperv/vtl:online",
                      mshv_vtl_alloc_context, NULL)  ← NULL teardown
  - No hv_setup_kexec_handler() registered
  - mshv_vtl_exit() only runs on rmmod, not during kexec

arch/x86/kernel/cpu/mshyperv.c:
  - machine_ops.shutdown = hv_machine_shutdown  ← applies to VTL2 too
  - hv_machine_shutdown() calls hv_kexec_handler() (VMBus only) +
    cpuhp_remove_state(CPUHP_AP_HYPERV_ONLINE) (VP Assist Pages only)
```

### Recommended upstream submission

**Patch 1** (critical fix, Cc: stable): `x86/hyperv/vtl: Clean up stale VTL VP state before kexec AP start`
- Add `hv_vtl_cleanup_stale_vp_state()` in `hv_vtl.c`
- Call from `hv_vtl_early_init()` (boot CPU) and `hv_vtl_wakeup_secondary_cpu()` (each AP)
- Disables register page and masks SINT0 before each VP starts
- No-op on first boot (register page already zero, SINT0 already masked)
- Fixes: kernel crash in scheduler after kexec in VTL2 (NULL deref in `try_to_wake_up`)

**Patch 2** (defense-in-depth): `Drivers: hv: mshv_vtl: Add defense-in-depth stale state cleanup`
- Add `mshv_vtl_cleanup_stale_state()` in `mshv_vtl_main.c`
- Call via `on_each_cpu()` at start of `hv_vtl_setup_synic()`
- Each CPU masks its own SINT0 and disables its register page via MSR/hypercall
- Redundant if Patch 1 ran correctly, but exercises different code path

**Commit message framing**: Should describe it as fixing a kexec crash in VTL2 environments, not as supporting the kexec stub. The bug exists regardless of whether the kexec target is bzImage, vmlinux, or stub-wrapped.

### Why it wasn't caught earlier

Upstream VTL2 kexec wasn't tested because:
- The `mshv_vtl` driver was only upstreamed recently (~4 months ago)
- Most VTL2 environments use host-driven servicing (partition recreated → clean state)
- Kexec in VTL2 is a new use case driven by servicing performance requirements
- The previous kexec path used `boot_cpus=0` which masked the bug by deferring VP startup
