Here's the full picture:

**VMBus (existing kexec-aware driver) — `drivers/hv/hv.c`:**

| Register | Enable (`hv_synic_enable_regs`) | Disable (`hv_synic_disable_regs`) | How cleared |
|---|---|---|---|
| SINT2 | read → set vector, unmask → write | read → mask → write | `hv_set_msr` |
| SIMP | read → enable=1, set GPA → write | read → enable=0, clear GPA → write | `hv_set_msr` |
| SIEFP | read → enable=1, set GPA → write | read → enable=0, clear GPA → write | `hv_set_msr` |
| SCONTROL | read → enable=1 → write | read → enable=0 → write | `hv_set_msr` |

**Key observation:** VMBus uses MSR-based access (`hv_get_msr`/`hv_set_msr`) because these are SynIC registers for the **current CPU only**.

**mshv_vtl — what we need to clear:**

| Register | How set (`mshv_vtl_configure_reg_page`) | What we do in cleanup |
|---|---|---|
| HV_REGISTER_REG_PAGE | `hv_call_set_vp_registers` (hypercall) | `hv_call_set_vp_registers` with value=0 |
| SINT0 | `hv_set_msr(HV_MSR_SINT0 + index, ...)` | `hv_set_msr` to mask it (in mshv_vtl_main.c), or `hv_call_set_vp_registers` (in hv_vtl.c) |

**The difference:**
- VMBus teardown uses **MSRs** — runs on the local CPU, clears its own registers
- Our `hv_vtl_cleanup_stale_vp_state` (in hv_vtl.c) uses **`hv_call_set_vp_registers` hypercall** — can target any VP by index remotely, which is why it works from the boot CPU before secondaries start
- Our `mshv_vtl_cleanup_stale_state` (in mshv_vtl_main.c) uses **MSR** (`hv_get_msr`/`hv_set_msr`) via `on_each_cpu` — same pattern as VMBus

So the mshv_vtl_main.c path already matches VMBus exactly. The hv_vtl.c path uses the hypercall because it needs to clean up **other VPs remotely** before they start — there's no VMBus equivalent for that because VMBus never starts VPs itself.
