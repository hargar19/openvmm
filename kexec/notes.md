In servicing there are 4 stages:
{ operation="stop"}:  state change complete duration=10.561632ms
{ operation="save"}:  state change complete duration=1.789714ms
{ operation="restore"}:  state change complete duration=228.159517ms
{ operation="start"}:  state change complete duration=33.762487ms



Requirement:
Enable the existing NVMe keepalive save/restore path for kexec.
Preserve queue/DMA memory and interrupt state.
Ensure old-kernel shutdown and new-kernel VFIO initialization do not reset the VF.


**Yes, for active NVMe VFs, our current kexec path pays shutdown costs before the jump and normal device initialization costs afterward.** It preserves logical VM state, but does not currently use live NVMe controller preservation.

**Current Behavior**
| Requirement | What this branch does |
|---|---|
| Save NVMe keepalive state | Disabled by the empty capability flags passed by kexec. |
| Preserve queue/DMA ownership | NVMe saved state and DMA-manager saved state are omitted. Memory physically surviving is not sufficient to restore allocation/queue ownership. |
| Preserve NVMe interrupt bookkeeping | Without NVMe saved state, the keepalive interrupt-state lists are empty. |
| Avoid controller shutdown | Not avoided: the driver closes queues, clears `CC.EN`, waits for `CSTS.RDY` to clear, and drops VFIO. See `driver.rs:650`. |
| Reattach using saved state | Not taken: absent saved state selects fresh device creation rather than `NvmeDriver::restore()`. See `device.rs:149`. |

**After Kexec**
When an NVMe device is needed, the new instance must reopen VFIO, establish mappings/interrupts, enable the controller, and initialize queues and namespaces.

One correction to the earlier explanation: **PCI FLR is not necessarily performed.** Fresh creation normally tries `NoReset` first, with FLR as fallback or when configured. Nevertheless, avoiding FLR does **not** avoid the NVMe controller/queue reconstruction described above.

The incoming environment may again enable `save_restore_supported`, but that only enables the capability for future operations. It cannot recover state omitted by the outgoing instance.

**Blackout Impact**
- `shutdown`: outgoing controller shutdown and readiness waits.
- `handoff_kernel_init`: kernel transition, kernel/driver startup, early userspace.
- `vm_build`/`restore`: device initialization needed before resume. Deferred initialization may occur afterward.

Other reconstruction, such as Underhill managers, memory mappings, and VMBus channel objects, also occurs, but much of it is common to host servicing. MANA keepalive is likewise disabled on this kexec path.

**For your supplied log, NVMe restored zero devices**, so it does not demonstrate a per-controller NVMe penalty. We need a kexec run with active NVMe devices to measure that cost; we cannot assign milliseconds from this log alone.
