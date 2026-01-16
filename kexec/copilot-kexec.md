User: so how does kexec works? Does is also store the state of previous run or it just boots a new kenrel without saving the state by default?

GitHub Copilot: kexec is basically a “soft reboot” into a new Linux kernel: the currently running kernel loads a *new* kernel image (plus initramfs + cmdline) into RAM, then jumps directly to it—without going through BIOS/UEFI/firmware reset.

What it does **by default**
- It **does not save/restore OS runtime state** (no preserving processes, threads, open files, kernel subsystems, etc.). After the jump, the new kernel boots like a fresh boot.
- It **may leave RAM contents physically intact**, but the new kernel typically reinitializes memory management and devices and does not treat old-kernel RAM as a “restorable state” unless you build an explicit mechanism around it.

So: **kexec boots a new kernel without saving state by default.** It’s not like hibernate/suspend.

Common modes
- **Normal kexec** (`kexec -l …` then `kexec -e`): controlled switch to a new kernel for upgrades/fast reboot.
- **Crash kexec / kdump** (`kexec -p …`): preloads a “crash kernel” reserved in `crashkernel=…`; on panic it jumps to that kernel to dump memory. This is for diagnostics, not preserving execution state.

How this relates to your servicing flow
- The “saved state” we discussed (the blob sent to the host, then fetched by the new VTL2 instance) is **not provided by kexec**.
- Instead, servicing uses an explicit **save/restore protocol** (state serialization + transfer) and then uses a **kexec-like jump** (or equivalent) to boot the new VTL2 kernel; the new VTL2 then pulls the saved blob and restores higher-level state.

If you tell me whether you’re asking about Linux kexec in general, or specifically the VTL2 “jump to kernel” mechanism used during servicing in this repo, I can map it precisely to the relevant components.

User: I want to replace current servicing with kexec so I am trying to map what kexec should do to exactly match the servicing case

GitHub Copilot: I’m going to locate where the “jump to kernel / persisted state” messages come from in the repo, then map the exact responsibilities kexec would need to cover to be a drop-in replacement for today’s servicing behavior.


To “replace servicing with kexec” *and still match the servicing semantics you’re seeing in the logs*, the key thing to internalize is:

- **Servicing = (explicit save/restore protocol) + (a restart boundary for VTL2)**  
- **kexec by itself only gives you the restart boundary** (jump to a new kernel).  
- So if you swap in kexec, you must **keep (or reimplement) the save/restore protocol** that makes the new VTL2 instance continue where the old one left off.

In your logs, the proof is:
- Old VTL2: “Done writing saved state, awaiting host response” (state blob finished, host now has it)
- New VTL2: “VTL2 restart, getting servicing state from the host” (new instance fetches blob and restores)

## What “kexec must do” to match the current servicing case

### 1) Preserve the servicing *contract* (what must survive)
These are the invariants servicing relies on; if kexec violates any, it won’t be equivalent:

- **VTL0 should not do a full reboot.** You’re reloading VTL2, not rebooting the VM.
- **VTL2 must restore its own internal/emulation state** from the blob that was saved pre-restart.
- **VMBus / device state must come back consistently** (channels, offers, device units), or VTL0 will observe breakage.
- **VPs may be stopped temporarily but must resume** once the restart boundary is complete.

### 2) Keep the explicit save/restore protocol (kexec doesn’t replace this)
Plain Linux kexec does not checkpoint and restore kernel/userspace state. To match servicing you still need:

- **Pre-kexec save:** VTL2 serializes the state it needs post-restart and sends it to the host (GET/GED path).
- **Post-kexec restore:** the new VTL2 instance fetches that saved state from the host and runs `restore_units`-style restore.

In repo terms, that “saved state from host” restore behavior is explicit in the Underhill worker:  
worker.rs

### 3) Decide what you mean by “kexec”
There are two “kexec-like” things in play:

1) **Linux kexec (inside the VTL2 Linux kernel)**: load new kernel+initrd, then `kexec -e`. This is the classic kexec feature.
2) **OpenHCL bootloader jump-to-kernel** (not Linux kexec, but very similar conceptually): the OpenHCL boot path constructs boot params/device tree and then jumps to the kernel entry after uninitializing hypercalls. That’s what your log line comes from, and the code is here:  
main.rs

If your goal is “replace servicing restart mechanism”, it matters which one you’re trying to substitute in.

### 4) The minimal “kexec-equivalent servicing” sequence
If we treat kexec as *only* the restart boundary, the equivalent sequence is:

1. **Trigger servicing-save (same as today):**
   - Quiesce devices/VM, stop VPs as needed.
   - Serialize servicing state.
   - Send it to host (chunked GET/GED transfer).
2. **Host acknowledges it has the blob** (and holds it somewhere the new VTL2 can access: same mechanism as today).
3. **Restart boundary (this is the part you replace with kexec):**
   - Instead of host-driven IGVM reload / firmware reload, you do a kexec-style jump to the new VTL2 kernel.
4. **New VTL2 boot detects “servicing scenario”:**
   - Today this is surfaced via DPS (`is_servicing_scenario`) and/or persisted-state hints.
5. **New VTL2 fetches the blob from host and restores:**
   - Exactly what you see in the log: “getting servicing state from host” then device/unit restores.
6. **Resume VM execution** (VPs resumed, vmbus/device units started).

## The main “gotcha” if you literally replace servicing with Linux kexec
If by “replace current servicing” you mean “delete the save/restore protocol and just kexec into a new kernel” — that will *not* match servicing. You’ll get:
- new kernel boots (yes)
- but **no continuity** of VTL2 state (no), and likely device/VMBus state mismatch unless you rebuild the same restore machinery.

## Repo touchpoints you’ll want while implementing
- The kexec-like “jump to kernel” boot boundary (OpenHCL bootloader):  
  main.rs
- How “servicing restore mode” is detected via persisted-state header / sidecar disable:  
  mod.rs
- How the new VTL2 fetches the saved blob from the host after restart:  
  worker.rs

## One question that determines the design
When you say “replace current servicing with kexec”, do you mean:
- A) Replace **host-side IGVM reload** with **in-VTL2 Linux kexec**, while keeping the same save/restore blob?  
or
- B) You just want to standardize on the existing OpenHCL “jump to kernel” (which is already kexec-like) and remove host reload plumbing?

If you answer A vs B, I can give you a concrete step-by-step mapping of *exactly* what must move from host → guest (and what cannot).

User: my answer is A. We need to service using kexec that means once the kexec is done, we have to load the previous state so that there is no disruption to the VTL0

GitHub Copilot: Searched for regex `\bkexec\b|KEXEC|kexec_load|kexec_file_load|sys_kexec|crashkernel|kdump` (`**/openhcl/**`), no results

Searched for regex `\bkexec\b|kexec_load|kexec_file_load|sys_kexec`, 9 results

If you choose **A (guest-initiated kexec + restore previous state)**, then “kexec” is only the *restart mechanism for VTL2*. To match today’s servicing behavior (and keep VTL0 disruption bounded to the same blackout window), you keep the **same save/restore protocol** and replace only the **host-triggered VTL2 reload** with a **guest-triggered kexec reboot of VTL2**.

Here’s the exact mapping.

**1) What must remain identical to current servicing**
- You still must do **servicing_save_vtl2 → send saved-state blob to host** (the “Done writing saved state…” line in your log).
- You still must do **new VTL2 boot → fetch saved-state blob from host → restore_units → resume VM** (the “VTL2 restart, getting servicing state…” lines).
- Therefore: **kexec is not a replacement for save/restore**. It replaces only the “restart VTL2” trigger.

That “restore previous state” part is already implemented in the new VTL2 bring-up path when `dps.general.is_servicing_scenario` is true: see worker.rs.

**2) The kexec-equivalent servicing sequence (A)**
This is the sequence that best matches what you see in kexec/servicing_logs:

1. **Host stages the new VTL2 payload** (today this is “new IGVM”; for kexec it must become “new kernel+initrd+cmdline accessible inside VTL2”).  
2. **VTL2 enters servicing-save mode** (same as today):
   - Quiesce/stop VTL0 execution (VP stop), stop devices, flush logs.
   - Serialize servicing state.
   - Stream it to the host over GET/GED (the chunking + “Done writing saved state, awaiting host response”).
3. **Host commits to holding that saved-state blob** until the next VTL2 instance asks for it (same as today; this is why the later “get_saved_state_from_host” works).
4. **Instead of host-triggered VTL2 reload, VTL2 performs kexec**:
   - Load the *new* kernel+initrd+cmdline with `kexec_file_load`/`kexec_load` (or userspace `kexec -l`).
   - Execute `kexec -e` (or the reboot syscall for kexec).
5. **New VTL2 comes up and restores**:
   - On boot, it detects this is a servicing restore scenario (host advertises it via DPS).
   - It fetches the saved state from the host and restores units, then resumes the VM.

This is exactly the pattern your log demonstrates:
- Save finished: servicing_logs
- New VTL2 fetches blob: servicing_logs
- Restore units + resume: servicing_logs

**3) The biggest engineering constraint: “kexec must boot the right kind of VTL2”**
Right now, VTL2 boot goes through **OpenHCL bootloader** logic that sets up host params/device tree, then jumps into the kernel entry point: see main.rs. It also has restore-path behaviors like “found persisted state header” and “disabling sidecar…” in mod.rs.

If you do **Linux kexec from inside the running VTL2 Linux**, you will *not* naturally re-run `openhcl_boot`. That means your kexec path must ensure the new kernel still gets:
- the correct command line
- the correct initrd
- the equivalent device-tree / platform description that OpenHCL expects

In other words: you can absolutely do A, but you need to be deliberate about *what payload* you kexec into and how you construct its boot parameters so the new VTL2 instance is functionally identical to the host-booted one.

**4) What you should keep vs replace**
- Keep (must-have for “no disruption to VTL0”): the **saved-state blob** and **restore_units** flow (already implemented).
- Replace (your change): the **“restart VTL2” trigger** from “host reloads IGVM/firmware” → “VTL2 performs kexec into new kernel”.

If you tell me how you plan to deliver the “new VTL2 kernel+initrd” into the running VTL2 (pre-positioned on its rootfs vs streamed from host at service time), I can suggest the cleanest place in the existing servicing path to invoke kexec and what handshake you need with the host before you jump.