# Kexec Sidecar Investigation Checkpoint

Status: experimental, 2026-10-05. This branch preserves the investigation;
it is not a working sidecar-enabled kexec implementation.

Branch: `user/hargar/kexec-2026-6.18-igvm-clean-sidecar`, created from
`user/hargar/kexec-2026-6.18-igvm-clean` at
`85f00351694dc75673947f85f5bfae0232f480f6`.

The checkpoint includes the remaining source changes, local configuration,
tests, notes, and captured logs. Earlier notes and model-generated proposals
are historical evidence, not authoritative descriptions of the current code.
In particular, [the original NVMe notes](notes.md) describe behavior that was
subsequently changed by the NVMe keepalive commit listed below.

## Goal and Existing Baseline

The goal is to reduce guest-driven kexec blackout by using a fresh incoming
sidecar and fresh CPU assignments, as host-driven servicing does. Retaining
the old sidecar would not satisfy the same-image update requirement.

The parent branch already contains:

| Commit | Change |
| --- | --- |
| `615253ebe` | Preserve and restore servicing state across kexec |
| `a411fdfd3` | Embed versioned raw kernel payloads in IGVM |
| `2e0cd43ec` | Receive bounded IGVM servicing transfers |
| `d6d83eb81` | Extract incoming payloads into owned memfds |
| `880c0e681` | Honor configured NVMe keepalive during kexec |
| `d5fa91816` | Account for blackout using phase timings |
| `85f003516` | Move kexec-specific progress logs to DEBUG |

NVMe keepalive reuses the existing servicing NVMe/DMA save and restore path;
it does not introduce a new controller implementation. The original commit
notes that active-controller preservation and data integrity need live
validation. Passing unit tests or logs restoring zero devices do not prove
those properties. This checkpoint does not change that qualification.

## Approaches Tried

### 1. Understand Host Servicing and Locate the Incoming Sidecar

Normal host boot or servicing enters `openhcl_boot`, initializes the incoming
sidecar before Linux, and constructs the CPU assignments and sidecar device
tree nodes. Linux then attaches its sidecar driver to the running sidecar.
The sidecar itself is a freestanding Rust kernel, not that Linux driver.

On supported x64 configurations, contiguous NUMA CPU groups are divided into
chunks of at most 32 VPs. Each chunk's base CPU starts Linux; eligible other
CPUs start sidecar. Saved device-related CPU requirements can override this
assignment. Initialization is conditional, not an unconditional servicing
feature. See [the servicing investigation](sidecar-work/how-sidecar-works.md).

The original guest-driven kexec path extracts Linux and initrd and enters
Linux directly, bypassing this boot-shim initialization.

We considered placing the sidecar in rootfs or obtaining it from the running
image's reserved memory. Neither is necessary for image delivery: the GET
receiver supplies the complete incoming IGVM, bounded at 128 MiB. Normal
loaded ELF segments are not guaranteed to preserve the exact source ELF,
so the implementation packages an additional raw payload from the same build.

### 2. Versioned Raw Payloads

The measured payload descriptor was extended without changing the original
V1 layout or the measured configuration/product-policy offsets:

- V1: raw Linux ELF and initrd, preserving the existing path.
- V2: adds an optional raw sidecar ELF.
- V3: also adds the incoming raw boot-shim ELF.

The builder imports sidecar and bootstrap bytes already available to it; no
rootfs packaging change is required. Extraction checks versions, bounds,
overlaps, page presence, and component ELF layouts. Artifact tests compare
the extracted bytes with the exact build inputs, including non-loadable file
contents. Early iterations intentionally rejected sidecar images before VM
stop after validating extraction; that was not a live handoff test.

### 3. Custom Bundle and Kernel Loader Changes

An initial implementation passed a private bundle to a modified kernel ELF
loader. The kernel loaded Linux, sidecar, and bootstrap, constructed a
physical handoff, and selected the bootstrap entry. The first executable
milestone required `OPENHCL_SIDECAR=off` and no active old sidecar.

This exposed an incorrect companion ELF alignment check. Valid sidecar and
bootstrap segments can begin at non-page-aligned addresses as long as file
offset and address satisfy ELF alignment congruence. That check was fixed
and exercised against actual build artifacts.

Live attempts still resulted in fresh initialization rather than servicing
restore. The custom bundle was then abandoned to avoid a sidecar-specific
kernel loader ABI. All investigation-added bundle parsing, header, and
selftest changes were removed from the companion kernel checkout. Those
removed kernel changes are not part of this Git snapshot.

### 4. Standard Composite ELF

The replacement is composed in OpenHCL userspace before VM stop. It uses the
branch's existing generic x64 ELF/DTB/VTL2 kexec support, not a new kernel
bundle parser. This does not imply the same loader is available in an
unmodified upstream kernel.

The composite is an ordinary x64 executable ELF with four outer load regions:
metadata/scratch, bootstrap, sidecar, and Linux. Outer regions are 2 MiB
aligned; inner component segment offsets and zero-filled memory are
preserved. A retained `OPENHCL` ELF note identifies a dedicated bootstrap
entry. Bootstrap derives the relocation bias, validates the relative layout,
constructs a private physical handoff, and jumps to Linux with its original
boot-parameter pointer in RSI.

Additional artifact-driven corrections were made:

- Linux startup code can be in a load segment without `PF_X`; Linux entry
  validation must not assume the companion executable-segment rule.
- Kernel-created boot parameters can be only 16-byte aligned.
- Relative layout validation must work above 4 GiB and reject overlap and
  arithmetic overflow.

The current staging path rejects an already-active sidecar, requires an
effective `OPENHCL_SIDECAR=off`, and requires a V3 bootstrap with the new ELF
note. V2 alone cannot provide the bootstrap. No-sidecar V1 images retain the
direct Linux path. Active sidecar CPU handoff is not enabled.

Composing the ELF also changes what must be authenticated: a signature over
the original Linux ELF alone does not authenticate the composite. No
signature-enforcement bypass or production composite-signing workflow was
added.

### 5. Bootstrap Probe and Failure Diagnostics

The [userspace execution probe](sidecar-work/test-composite-entry.c) loads an
exported composite at a 16 GiB virtual address. In private test memory it
replaces the privileged CR4 instructions, supplies synthetic boot parameters,
and traps at the Linux destination. It checks the entry, relocations, BSS
initialization, and preservation of the boot-parameter pointer.

The normal probe passed. Fault-injection modes also verified `KXLAYOUT` and
`KXLOADER` reports using the existing Hyper-V pre-OS crash-reporting format.
Explicit Rust validation failures now report stage tags before faulting.
These tags do not catch faults before Rust entry, arbitrary CPU exceptions,
or failures after Linux takes control. The crash control requests no crash
dump; host visibility depends on the deployed Hyper-V version.

The probe does not exercise actual CR4 state, page tables, kernel shutdown,
interrupts, DMA, Linux startup, or real kernel-created boot parameters.
The local account could not access `/dev/kvm`, so it did not provide a
privileged substitute for the live transition. See the
[diagnostic procedure and tag table](sidecar-work/composite-entry-debug.md).

## Observed Results and Remaining Failure

Earlier validation included affected-package unit tests, clippy, rustdoc,
bare-metal bootstrap builds, complete IGVM builds, exact-input artifact
tests, exported-ELF inspection, and the userspace probe. These establish
packaging and limited execution properties, not successful live servicing.

The latest supplied live capture reached the kernel ELF loader at
53.466804 seconds, disconnected the diagnostic stream, and then showed:

- No `OPENHCL_KEXEC_SERVICING=1` in the new boot command line.
- Fresh worker initialization and reloading UEFI into VTL0.
- No servicing restore or blackout-completion event.
- Boot-shim reference times near zero.

This is consistent with a failed transition followed by reset and normal
host boot. The exception, reset source, and exact failing stage remain
unknown. The ELF KASLR warning is informational. A disconnected diagnostic
stream alone can also occur during successful kexec. The host's
`is_servicing_scenario=false` flag does not by itself disprove local kexec,
which uses its own marker.

The DEBUG filter was active, but the supplied transition did not include the
expected staged-image, stop/save, and reboot milestones. Their absence could
reflect lost output; it does not prove the corresponding functions did not
execute. Host Worker/VMMS reset or crash events and the exact servicing
trigger command/output are still needed. Both supplied boots already had
no VTL0 boot devices, so a later correctness run also needs a bootable guest.

## Other Changes Preserved in This Checkpoint

- The release manifest explicitly sets `OPENHCL_SIDECAR=off` for the
  bootstrap-only experiment. This is not sidecar enablement.
- The release manifest's memory allocation is increased from 70 MiB to
  192 MiB, with the corresponding 16-VP DMA-hint entry updated. These local
  edits appeared during checkpoint preparation; their effect on the live
  failure has not been established.
- `UnderhillWorker::new` passes `boot_init=true` even on kexec. This is a
  pre-existing local experiment affecting memory initialization/protection,
  not instrumentation or an independently validated fix.
- The raw logs and earlier notes are retained without rewriting their
  historical conclusions. Model proposals are not evidence of execution.
- The separate kernel checkout is not committed here. Its remaining local
  change at checkpoint preparation was only the generated compiler-version
  text in its HCL x64 configuration.

## Build and Reproduce

With the companion kernel and module artifacts already built:

```bash
cargo xflowey build-igvm x64 --release --release-cfg \
  --custom-kernel ../OHCL-Linux-Kernel/out/build/native/bin/x64/vmlinux \
  --custom-kernel-modules ../OHCL-Linux-Kernel/out/build/native/bin/x64/modules \
  --custom-binary ../OHCL-Linux-Kernel/out/build/native/bin/x64/vmlinux \
  --with-sidecar
```

The default output is
`flowey-out/artifacts/build-igvm/ship/x64-custom/openhcl-x64-custom.bin`.
`--build-label` changes output names, not feature activation. Keep sidecar
off and start with no active sidecar client. The diagnostic additions need
an updated IGVM, not another kernel bundle patch. Never manually add
`OPENHCL_KEXEC_SERVICING=1` to a normal boot: it requires persisted state.

Use the [IGVM build guide](../Guide/src/dev_guide/getting_started/build_openhcl.md)
for exact-input artifact validation and export the composite with
`OPENHCL_TEST_COMPOSITE_ELF` for the probe. See the
[IGVM contract](../Guide/src/reference/architecture/openhcl/igvm.md) for
descriptor versions and the bootstrap handoff.

## Performance Findings and Next Work

Six earlier sidecar-off kexec captures reported blackouts of 368.0259,
408.2141, 430.6916, 437.6088, 439.9577, and 531.2530 ms, averaging
435.9585 ms. Those are baseline captures, not successful composite-sidecar
transitions. Most measured variation was in `handoff_kernel_init` and
`vm_build`, not SMP startup alone. Console output materially affected timing;
comparisons with different RAM, CPU, workload, and logging settings do not
isolate the sidecar benefit.

The remaining work is:

1. Localize the live failure and demonstrate bootstrap-only state restore
   with VTL0 continuity before enabling sidecar CPUs.
2. Implement safe retirement of the old sidecar and fresh CPU assignments,
   device-tree/E820 updates, and `boot_cpus` construction. A sidecar REMOVE
   acknowledgment alone does not prove its old code or stack is unused.
3. Validate NVMe controller and DMA continuity under real I/O, plus repeated
   update cycles and safe reuse of old sidecar memory.
4. Resolve composite signing requirements for the deployment environment.
5. Compare matched configurations using median, p95, maximum blackout, and
   first successful guest I/O after resume.

Preserve the existing pre-stop staging model: compose and validate before
blackout, and reuse the shared servicing state and keepalive paths. This
checkpoint is intended to make the failed approaches and current boundary
reproducible, not to claim those remaining steps are complete.
