# Composite Entry Investigation

The 2026-10-05 capture shows the ELF loader at 49.141928 seconds, followed by
a disconnected diagnostic stream and another fresh initialization. The second
boot lacks `OPENHCL_KEXEC_SERVICING=1`, reloads UEFI into VTL0, and has boot-shim
reference times near zero. This is not a successful servicing restore. The
capture is consistent with a failed transition followed by a VM reset, but
does not identify the exception or reset source.

## Local Execution Probe

Run against an ELF exported by the production artifact test using
`OPENHCL_TEST_COMPOSITE_ELF`:

```bash
cc -O0 -g -Wall -Wextra -Werror \
    kexec/sidecar-work/test-composite-entry.c -o /tmp/test-composite-entry
/tmp/test-composite-entry path/to/composite.elf
```

The probe loads the actual composite at a 16 GiB virtual address, supplies a
synthetic Linux boot-parameter page, and executes its entry. In private test
memory only, it replaces the bootstrap's two privileged CR4 instructions and
places an INT3 at the Linux destination. Success requires reaching that exact
destination with RSI unchanged and RDI zero. The input artifact is not edited.

The recorded composite build and the current `ship/x64-custom` build both
passed. This exercises assembly dispatch, BSS clearing, ELF relocation, layout
validation, and the transfer of boot parameters. It does not exercise privileged
CPU state, real page tables, kernel shutdown, interrupts, DMA, Linux startup,
or the actual contents and lifetime of kernel-created boot parameters. It is
not an end-to-end kexec test or evidence that the live failure is fixed.

The probe can also inject validation failures into a diagnostic bootstrap:

```bash
/tmp/test-composite-entry path/to/composite.elf layout
/tmp/test-composite-entry path/to/composite.elf loader
```

These modes intercept only the Hyper-V crash-register WRMSR instructions in
the probe's signal handler. They check the emitted `KXLAYOUT` or `KXLOADER`
tag and message. The normal success mode does not intercept crash reporting.

## Diagnostic Bootstrap

Bootstrap validation failures now use the existing Hyper-V pre-OS crash
reporting mechanism before faulting, instead of executing an unexplained
UD2 directly. This requires a rebuilt incoming IGVM, but no kernel change.
It does not enable sidecar or fix the unresolved live reset.

Host crash parameter P0 identifies `BOOTSHIM`; P1 contains the eight-byte
stage tag in big-endian order. A message is supplied in the existing crash
message buffer. How the host exposes it depends on the deployed host build.

| Tag | Failed check |
| --- | --- |
| `KXBASE01`, `KXBASE02` | Composite base calculation or alignment |
| `KXLAYOUT` | Composite metadata or boot-parameter range |
| `KXHAND01`, `KXHAND02` | Handoff pointer or allocation validation |
| `KXIMAGE0` | Bootstrap allocation bounds |
| `KXLOADER` | Linux kexec loader marker in boot parameters |
| `KXCPUS00`, `KXSTART0`, `KXOFF000` | Sidecar parameters/start result/disabled state |

These tags cover explicit validation failures after the assembly entry. They
do not catch arbitrary CPU exceptions, faults before Rust entry, or failures
after Linux takes control. A missing tag does not prove the bootstrap was
not involved. Privileged execution still needs a VM test; the local account
does not have access to `/dev/kvm`.

## Next Capture

For one diagnostic run, use this userspace trace filter:

```text
HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug
```

Capture kmsg before triggering the update. Relevant existing messages are:

- `incoming IGVM staged before VM stop`
- State-unit stop/save completion or a servicing-save error
- `triggering guest-driven kexec reboot`
- On a real successor, `OPENHCL_KEXEC_SERVICING=1` in the command line and
  `restored servicing state from persisted memory`

Collect the host's Hyper-V Worker/Admin and VMMS/Admin events for this VM
around the disconnect, including any reset reason, crash parameters, or dump
location. The diagnostic connection closing is expected during a successful
restart too; it is not itself proof of a crash. The new cold-boot command line
and firmware reload distinguish this capture from a successful restore.

Also record the exact host command used to initiate servicing and its output.
The latest debug capture confirms the trace filter is active, but includes
only the two initialization sequences, not the request/staging/stop/save/reboot
interval. `is_servicing_scenario=false` alone is not evidence against kexec:
that is the host-servicing flag; local kexec restore uses its own command-line
marker.

For a recent attempt, collect the host events in PowerShell:

```powershell
$logs = @(
  'Microsoft-Windows-Hyper-V-Worker-Admin',
  'Microsoft-Windows-Hyper-V-VMMS-Admin'
)
Get-WinEvent -FilterHashtable @{
  LogName = $logs
  StartTime = (Get-Date).AddMinutes(-10)
} | Format-List TimeCreated, Id, ProviderName, Message
```

Keep `OPENHCL_SIDECAR=off`. Do not add `OPENHCL_KEXEC_SERVICING=1` manually to
the release manifest: it is added only to the staged successor command line,
and a cold boot has no servicing state to restore.

Both supplied boots report `uefi boot: no boot devices`. There is no running
VTL0 OS in this capture whose uptime could demonstrate preservation. This does
not explain the VTL2 reset; firmware state should still be restored. A later
correctness test should include a bootable VTL0 workload.
