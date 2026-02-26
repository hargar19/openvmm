# Kexec Servicing — Q&A Log

---

## Q1: Why disable sidecar for kexec when original servicing already disables it?

**Q:** With the original servicing flow, sidecar is already disabled? So why do we need to disable it for kexec when we are following the same flow?

**A:** The key difference is that **openhcl_boot doesn't re-run during kexec**.

In **original servicing**:
1. Host reloads the entire VTL2 IGVM image
2. `openhcl_boot` runs again from scratch
3. It detects persisted state → sets `SidecarOptions::DisabledServicing`
4. Sidecar kernel is **never started** on the APs
5. All 32 CPUs boot via normal SMP
6. `SidecarClient::new()` naturally returns `None` because no sidecar device files exist

In **kexec**:
1. `openhcl_boot` does **not** re-run — kexec jumps directly to the new kernel
2. The sidecar kernel from the **first boot** is still running on the AP CPUs
3. The new kernel's `mshv_vtl_sidecar` module detects those sidecar CPUs and creates `/dev/mshv_vtl_sidecar*` device files
4. `SidecarClient::new()` would successfully open them and return `Some(client)`
5. VPs would be spawned as sidecar VPs — **wrong behavior**

The sidecar is "naturally" disabled in original servicing because openhcl_boot actively prevents it from starting. In kexec, that gate doesn't exist, so we compensate by explicitly skipping `SidecarClient::new()` and onlining all CPUs ourselves. The `skip_sidecar` flag fills the role that `SidecarOptions::DisabledServicing` plays in openhcl_boot during original servicing.

---

## Q2: Do we use SMP, and why is the sidecar kernel still running after servicing?

### Q2a: SMP vs our approach

**Q:** Does kexec servicing use SMP like original servicing does?

**A:** No, we don't use SMP. The mechanisms are different:

- **Original servicing**: openhcl_boot disables sidecar → sidecar kernel is **never started** → Linux kernel boots with all 32 CPUs available → `smp_init()` brings them all up via SMP (`Brought up 1 node, 32 CPUs`)
- **Kexec**: The new kernel boots with `boot_cpus=0` (only CPU 0) because the sidecar kernel is still holding the APs. We call `underhill_threadpool::set_cpu_online()` which writes to `/sys/devices/system/cpu/cpuN/online` to hotplug CPUs into Linux one-by-one. This is CPU **hotplug**, not SMP boot.

### Q2b: Why is the sidecar kernel still running after servicing?

**Q:** Servicing stops the VPs — shouldn't that also stop the sidecar kernel?

**A:** **Stopping VPs ≠ stopping the sidecar kernel.** They operate at different levels:

- **Guest VPs (VTL0)**: These are what get "stopped" during servicing. The servicing logs show `stopping vp 0x0` through `0x1f` — that's stopping the *guest execution* on those VPs.
- **Sidecar kernel (VTL2)**: This is a separate, minimal kernel running on the AP CPUs at **VTL2 level**. It's part of the paravisor infrastructure, not the guest. Stopping guest VPs doesn't terminate the sidecar kernel.

During kexec:
1. Guest VPs are stopped (servicing save)
2. The main VTL2 Linux kernel calls `kexec_load()` + `reboot(LINUX_REBOOT_CMD_KEXEC)`
3. kexec replaces the main kernel on **only the CPUs it controls** (CPU 0)
4. The APs running the sidecar kernel are **not under the main kernel's control** — they're running an entirely separate kernel
5. New kernel boots on CPU 0, sidecar kernel is still alive on the APs

In original servicing, this isn't a problem because the host reloads the entire IGVM → openhcl_boot runs fresh → it chooses not to start sidecar at all. With kexec, openhcl_boot doesn't re-run, so the sidecar from the first boot persists.

---

## Q3: How does the host know servicing is complete?

**Q:** How is the host notified that servicing is complete in the original flow, and what happens with kexec?

**A:** The kexec hook fires **before** `send_servicing_state()` is called. `try_kexec_after_servicing` calls `exec()` which **replaces the entire process** — so `send_servicing_state()` **never executes**.

**Original servicing flow:**
1. Save state → encode
2. **Send state to host** via `send_servicing_state()` (GET protocol)
3. Host receives it → host reloads VTL2 IGVM
4. New VTL2 calls `get_saved_state_from_host()` → host sends it back
5. Restore → Start → Resume → **Host sees VM running again → servicing command completes**

**Kexec flow:**
1. Save state → encode
2. **kexec fires** (exec replaces process) → `send_servicing_state()` never runs
3. New kernel boots → reads state from persistent memory → restores → starts → resumes
4. **Host never got the saved state, never got any acknowledgment** → servicing command hangs forever

The host is waiting for the GET protocol exchange to complete. It never does because kexec short-circuited it.

**Two options considered:**
- **Option A: Move kexec after `send_servicing_state()`** — Let the state go to the host first, then kexec. Risk: host may trigger its own VTL2 reload before kexec completes.
- **Option B: Send a "servicing complete" notification without the state blob** — After kexec restore, notify the host that VTL2 is up and servicing is done via a new GET protocol message.

---

## Q4: What happened when we sent an empty blob?

**Q:** We tried sending an empty blob via `send_servicing_state(vec![])` after kexec restore. What went wrong?

**A:** The sequence was:

1. **Kexec'd VTL2** boots, restores from persistent memory, sends empty blob via `send_servicing_state(vec![])`
2. **Host receives the empty blob** → interprets it as "save complete, time to reload VTL2"
3. **Host triggers a second VTL2 reload** (openhcl_boot runs again: `found persisted state header`, `disabling sidecar`, etc.)
4. **Second VTL2 crashes** trying to re-apply VTL2 protections that the kexec instance already set: `AccessDenied`

The fundamental problem: `send_servicing_state()` is part of the **save** protocol. Sending it tells the host "here's my state, now reload me." The host then triggers its standard VTL2 reload — but VTL2 already reloaded itself via kexec.

**Resolution:** The empty blob send was reverted. `send_servicing_state()` is the wrong message — it triggers the exact reload we're trying to avoid.

---

## Q5: What is the current state of host notification?

**Q:** How does the host-side servicing command get unblocked?

**A:** Currently, **it doesn't** — the servicing PowerShell command hangs/times out, but the VM itself is fully operational.

The core issue: `send_servicing_state()` is a "save" protocol message that tells the host "here's my state, now reload VTL2." There's no existing GET message that says "servicing is done, don't reload me." The protocol fundamentally ties save-completion to host-driven reload.

**Attempted solution (reverted):** A `KEXEC_SERVICING_COMPLETE` GET protocol notification was implemented across 5 files:

1. **get_protocol/src/lib.rs** — Added `KEXEC_SERVICING_COMPLETE = 10` to `HostNotifications` enum + `KexecServicingCompleteNotification` struct
2. **process_loop.rs** — Added `KexecServicingComplete` variant to `Msg` enum + match arm
3. **client.rs** — Added `notify_kexec_servicing_complete()` fire-and-forget call
4. **guest_emulation_device/src/lib.rs** — Handler that completes the pending save RPC to unblock the host
5. **worker.rs** — Calls `notify_kexec_servicing_complete()` after successful kexec restore

**Why it was reverted:** On real Hyper-V, vmwp.exe (the host process) doesn't understand custom GET notifications. The GED in the OpenVMM codebase would handle it, but the actual production host (vmwp.exe) ignores unknown notification types.

**Remaining options** (require protocol/host changes):
- A new GET protocol message that vmwp.exe understands (requires vmwp.exe changes)
- Host-side timeout/detection that VTL2 restarted itself
- Alternative signaling mechanism outside the GET protocol