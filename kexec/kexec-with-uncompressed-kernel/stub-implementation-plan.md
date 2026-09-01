# Kexec Stub Implementation Plan

## Goal

Implement a bare-metal Rust stub that receives control via kexec, loads an uncompressed vmlinux from memory, and boots it. This solves two problems:

1. **PT_LOAD conflict**: `kexec_file_load(vmlinux, ...)` fails because vmlinux's PT_LOAD segments (e.g., at 0x1000000) overlap with the running kernel's reserved memory. The stub sidesteps this — after kexec, the old kernel is gone and all physical memory is free for placing segments.

2. **Future transient IGVM mapping**: When the host-side mapping infrastructure is ready, the stub reads vmlinux + initrd from a host-mapped GPA range instead of from the kexec initrd. The stub's core logic (ELF parser, boot_params, kernel jump) stays the same.

## Architecture

```
underhill_core (userspace)          stub (bare-metal, after kexec)
──────────────────────────          ─────────────────────────────
1. Read vmlinux + initrd from rootfs
2. Construct packed blob (memfd):
   [header][vmlinux][pad][initrd]
3. Construct synthetic bzImage:
   [bzImage header][stub binary][packed blob]
   Patch pack_offset/pack_size into stub header
4. kexec_file_load(bzImage, flags=NO_INITRAMFS)
   ── pre-loaded, VM still running ──
5. Stop VM, reboot(KEXEC)
   ── old kernel gone ──            6. Entry: self-relocate, build page tables,
                                       zero BSS, set stack, call start()
                                    7. Read pack_offset/pack_size from stub header
                                       Compute packed_blob_addr = load_base + offset
                                    8. Validate pack header magic, extract vmlinux/initrd
                                    9. Parse vmlinux ELF → PT_LOAD segments + entry
                                    10. memcpy segments to target physical addresses
                                    11. Build new boot_params (e820 + vmlinux range,
                                        initrd phys addr, cmdline)
                                    12. Jump to vmlinux entry point
```

## Approach 1: Pack vmlinux in a synthetic bzImage (current — IMPLEMENTED)

**This approach is fully implemented and working.**

vmlinux and the kexec stub binary live in the rootfs. `kexec_prepare.rs` reads vmlinux and
initrd, packs them into a blob, appends that blob to the stub binary inside a synthetic
bzImage, and pre-loads it via `kexec_file_load` **before** the VM is stopped. When servicing
begins, only `reboot(KEXEC)` is needed.

### Pack format

The packed blob is embedded inside the synthetic bzImage after the stub binary:

```
Synthetic bzImage layout:
Offset  Field               Size
0x000   bzImage header      512 bytes (boot sector + setup_header fields)
0x200   stub flat binary    ~14 KB
        packed blob start   (= pack_offset, patched into stub header at bytes 8–23)
           0  magic         8 bytes ("KXSTUB\x01\x00")
           8  vmlinux_size  8 bytes (little-endian u64)
          16  initrd_size   8 bytes (little-endian u64)
          24  vmlinux data  vmlinux_size bytes
          24+V (page-aligned) initrd data  initrd_size bytes
        packed blob end     (= pack_offset + pack_size)
```

The stub reads `pack_offset` and `pack_size` from its own header (bytes 8–23 of the flat
binary), then computes `packed_blob_phys = kernel_load_addr + pack_offset`.

The `KEXEC_FILE_NO_INITRAMFS` flag (0x04) is passed to `kexec_file_load` since there is no
separate initrd file — it's inside the bzImage.

### What changes

| Component | Change |
|-----------|--------|
| **`openhcl/kexec_stub/`** | New `no_std` crate — bare-metal Rust PIE (~14 KB flat binary) |
| **`openhcl/kexec_sys/`** | Added `flags` param, `KEXEC_FILE_NO_INITRAMFS`, `memfd_create()` |
| **`kexec_prepare.rs`** | `prepare_kexec_stub()`: stream vmlinux+initrd to memfd, construct synthetic bzImage header, pre-load before VM stop |
| **`rootfs.config`** | Add `/boot/vmlinux` and `/boot/kexec_stub.bin` |
| **`openhcl-x64-release.json`** | VTL2 memory 128 MB → 256 MB (for kexec_file_load vmalloc) |
| **OHCL-Linux-Kernel** | Stale VP state cleanup in `hv_vtl.c` + `mshv_vtl_main.c` (see Kernel Changes Details) |
| **IGVM loader** | No change |
| **ShimParamsRaw** | No change |
| **DTB** | No change |

### Pros
- No IGVM/loader changes — works immediately
- Validates the stub (the hard part) independently
- Same rootfs pattern as bzImage today
- Kexec pre-loaded before VM stop — saves ~3.5s vs loading during stop

### Cons
- vmlinux in rootfs adds ~3 MB to IGVM (compressed in cpio.gz; temporary — removed when Approach 2 is ready)
- VTL2 memory increased to 256 MB for kexec_file_load vmalloc headroom
- Requires OOT kernel patch for stale VP state cleanup
- Not version-correct for servicing (loads from current rootfs, not new IGVM)

### Performance

End-to-end kexec servicing blackout time: **6.6s** (32 VPs)

| Comparison | Blackout time |
|------------|---------------|
| Stub kexec (this approach) | 6.6s |
| Normal servicing (no kexec) | 6.7s |
| Old bzImage kexec | 14.5s |

---

## Approach 2: Embed vmlinux as IGVM PageData, stub reads from known GPA

Once the stub is validated with Approach 1, swap the vmlinux source from rootfs to IGVM PageData embedded at a known GPA. This is a stepping stone toward the final transient IGVM mapping.

### What changes (on top of Approach 1)

| Component | Change |
|-----------|--------|
| **`vm/loader/src/paravisor.rs`** | Import vmlinux blob as PageData after initrd |
| **`loader_defs/src/shim.rs`** | Add `kexec_kernel_offset`/`kexec_kernel_size` to `ShimParamsRaw` |
| **`openhcl_boot`** | Parse new ShimParams fields, expose via DTB memory type |
| **`kexec_prepare.rs`** | Read vmlinux from `/dev/mem` at DTB-discovered GPA instead of rootfs |
| **`rootfs.config`** | Remove `/boot/vmlinux` |
| **Stub** | No change — still reads from packed blob in ramdisk |

### Pros
- No tmpfs overhead (vmlinux is in IGVM GPA space, not rootfs)
- Validates the IGVM embedding path

### Cons
- Permanent ~12 MB in VTL2 GPA space
- Not version-correct (vmlinux baked at VM creation time)

---

## Final target: Stub + Transient IGVM Mapping

See [bzimage-in-igvm-plan.md](./bzimage-in-igvm-plan.md) for the full design. When the host-side mapping infrastructure is built:

1. Host maps new IGVM into VTL2 GPA at servicing time
2. `kexec_prepare.rs` reads vmlinux + initrd from the mapped GPA
3. Packs and passes to stub via kexec (same as Approach 1)
4. After boot, mapping is released — zero permanent overhead

The stub code is identical across all three approaches. Only the **source of vmlinux bytes** changes.

---

## Stub Design Details

### Crate structure

```
openhcl/kexec_stub/
├── Cargo.toml          # no_std, depends on minimal_rt
├── build.rs            # minimal_rt_build::init()
├── link.x              # Custom linker script (.text.entry first, rela markers)
└── src/
    ├── main.rs         # stub_main: parse packed blob, load ELF, boot
    ├── rt.rs           # start() → stub_main(), stack, BSS
    ├── elf.rs          # Minimal ELF64 parser (PT_LOAD only)
    ├── boot_params.rs  # Build x86_64 boot_params struct
    └── arch/
        └── x86_64/
            └── entry.S # Entry: self-relocate, page tables, BSS zero, stack, call start
```

### What the stub does (x86_64)

1. **entry.S — self-relocation**: Compute `load_base = lea _start[rip]`. Iterate `.rela.dyn`
   entries (marked by `__rela_start`/`__rela_end` in `link.x`). For each `R_X86_64_RELATIVE`:
   write `*(base + r_offset) = base + r_addend`. Fixes up GOT entries and vtable pointers.
2. **entry.S — page tables**: Allocate page table pages at `page_align(_end)`. Build identity
   mapping 0–512 GB using 1 GB huge pages (PML4 → PDPT, 512 entries). Load via `mov cr3`.
3. **entry.S — BSS + stack**: Zero BSS segment, set stack pointer, call `start(boot_params_ptr)`.
4. **Parse pack header**: Read `pack_offset`/`pack_size` from stub header (bytes 8–23 of flat
   binary at `load_base`). Compute `packed_blob_phys = load_base + pack_offset`. Validate magic.
5. **Parse inherited boot_params**: Extract e820 map and command line pointer.
6. **ELF parser**: Iterate PT_LOAD segments from vmlinux, find entry point.
7. **Copy segments**: `memcpy` each PT_LOAD to its `p_paddr` (safe — old kernel is gone).
8. **Build new boot_params**:
   - Copy e820 map from inherited boot_params
   - Add `E820_TYPE_RAM` entry covering vmlinux physical range (`phdr_min..phdr_max`)
   - Set `ramdisk_image` → page-aligned physical address of real initrd (inside packed blob)
   - Set `cmd_line_ptr` → reuse inherited command line
   - Set `type_of_loader = 0xff`
9. **Jump**: `kernel_entry(0, &new_boot_params)` — standard x86_64 Linux 64-bit boot protocol

### Page tables

The stub builds its own identity-mapped page tables at runtime in `entry.S`. Page table pages
are allocated at `page_align(_end)` (after the stub's BSS). The mapping covers 0–512 GB using
1 GB huge pages (PML4 → PDPT with 512 entries). This is required because VTL2 memory starts
at 0x408000000 (above 4 GB), outside the inherited kexec purgatory mapping (0–4 GB).

### Serial output

Debug prints (COM1 0x3F8 via port I/O) are available but disabled in the committed code.
Enable by uncommenting `serial_print`/`serial_print_hex` calls in `main.rs` and the
assembly debug blocks in `entry.S`.

### Memory safety

The stub must avoid overwriting:
- **Persisted servicing state** (first 2 MB of VTL2 memory) — contains serialized state from the old underhill instance
- **Its own code + stack + packed blob** — don't copy PT_LOAD segments over the stub itself
- vmlinux PT_LOAD segments are at high addresses (typically 0x1000000+), well above the stub

---

## Implementation Order (all completed)

1. ✅ **Create `openhcl/kexec_stub/` crate** — entry.S, minimal main that prints to serial and halts
2. ✅ **Add to rootfs** — build the stub, add to rootfs.config
3. ✅ **Test kexec into stub** — verify stub receives control, prints to serial
4. ✅ **Implement ELF parser** — parse PT_LOAD segments from vmlinux in memory
5. ✅ **Implement boot_params builder** — e820, initrd, cmdline
6. ✅ **Implement full boot** — copy segments, build boot_params, jump to kernel
7. ✅ **Modify `kexec_prepare.rs`** — pack vmlinux + initrd, load stub
8. ✅ **Add vmlinux to rootfs** — temporary, for testing
9. ✅ **Fix stale VTL state** — kernel changes for register page + SINT0 cleanup
10. ✅ **End-to-end test** — kexec → stub → vmlinux boots → underhill starts → VM resumes

---

## Implementation

### Created Files

| File | Purpose |
|------|---------|
| `openhcl/kexec_stub/Cargo.toml` | Crate definition — depends on `minimal_rt`, `loader_defs`, `zerocopy` |
| `openhcl/kexec_stub/build.rs` | Calls `minimal_rt_build::init()` |
| `openhcl/kexec_stub/src/main.rs` | Main logic: parse packed blob, load vmlinux ELF segments, build boot_params, jump |
| `openhcl/kexec_stub/src/rt.rs` | Stack, start(), panic handler, stack cookie verification |
| `openhcl/kexec_stub/src/elf.rs` | Minimal ELF64 parser (PT_LOAD segments + entry point) |
| `openhcl/kexec_stub/src/arch/x86_64/entry.S` | Entry point: zero BSS, set stack, enable SSE, call start |
| `openhcl/kexec_stub/src/arch/x86_64/mod.rs` | `global_asm!` for entry.S |
| `openhcl/kexec_stub/src/arch/mod.rs` | Architecture module |

### Modified Files (OpenVMM)

| File | Change |
|------|--------|
| `Cargo.toml` | Added `kexec_stub` to workspace members |
| `Cargo.lock` | Auto-generated |
| `openhcl/kexec_sys/src/lib.rs` | Added `flags` param to `kexec_file_load()`, `KEXEC_FILE_NO_INITRAMFS`, `memfd_create()` |
| `openhcl/underhill_core/src/dispatch/kexec_prepare.rs` | New stub path: `prepare_kexec_stub()` + `construct_bzimage_header()`, with bzImage fallback. Kexec is now pre-loaded before VM stop. |
| `openhcl/rootfs.config` | Added `/boot/vmlinux` and `/boot/kexec_stub.bin` |
| `vm/loader/manifests/openhcl-x64-release.json` | Increased VTL2 memory from 128 MB to 256 MB (32768 → 65536 pages) |

### Modified Files (OHCL-Linux-Kernel)

| File | Change |
|------|--------|
| `arch/x86/hyperv/hv_vtl.c` | Added `hv_vtl_cleanup_stale_vp_state()` — early per-VP cleanup of register page + SINT0 before VP start |
| `drivers/hv/mshv_vtl_main.c` | Added `mshv_vtl_cleanup_stale_state()` — safety-net cleanup via `on_each_cpu()` during SynIC init |

### Kernel Changes Details

After kexec, the hypervisor retains per-VP state from the old kernel that the new kernel
does not expect. Two pieces of state cause crashes:

1. **Register page (`HV_REGISTER_REG_PAGE`)**: The old kernel configured the hypervisor to
   write VP intercept context to a shared memory page at a specific physical address. After
   kexec, that physical address is reused by the new kernel for other purposes (e.g., task
   structs, page tables). When the hypervisor dispatches a VP to VTL2, it writes to the stale
   address, corrupting new kernel data structures. This manifests as NULL pointer dereferences
   in the scheduler (`cpus_ptr == NULL` in `try_to_wake_up`).

2. **SINT0 (Synthetic Interrupt Source 0)**: The old kernel left SINT0 unmasked with a vector
   pointing to its interrupt handler. After kexec, no IDT handler exists for that vector.
   When the hypervisor delivers the interrupt, it either crashes or causes CPUs to hang
   during `cpuhp_setup_state` in `mshv_vtl_init`.

**Why this wasn't needed with the old bzImage kexec path**: The old path used `boot_cpus=0`
on the command line, which booted only 1 CPU during kernel SMP init. Secondary VPs stayed
dormant until the `mshv_vtl` driver explicitly initialized them later, at which point proper
state setup overwrote any stale values. The stub path boots all 32 CPUs during SMP init
(no `boot_cpus=0`), which exposes the stale state before the driver can clean it up.

**Fix — two levels of cleanup:**

**Level 1: Early cleanup (`hv_vtl.c`)** — runs BEFORE each VP starts, preventing corruption:

- `hv_vtl_cleanup_stale_vp_state(vp_index)` issues a single `hv_call_set_vp_registers()`
  hypercall that sets two registers in one batch:
  - `HV_REGISTER_REG_PAGE = 0` (disable register page)
  - `SINT0 = masked` (prevent stale interrupts)
- Called from `hv_vtl_early_init()` for the boot CPU (VP 0)
- Called from `hv_vtl_wakeup_secondary_cpu()` for each secondary VP, right before
  `hv_vtl_bringup_vcpu()` / `HVCALL_START_VP`
- Both calls are no-ops on first boot (register page is zero, SINT0 is masked by default)

**Level 2: Safety-net cleanup (`mshv_vtl_main.c`)** — defense-in-depth during driver init:

- `mshv_vtl_cleanup_stale_state()` runs on every CPU via `on_each_cpu()` at the start of
  `hv_vtl_setup_synic()`, before any SynIC handlers are installed
- Each CPU reads its own SINT0 via MSR; if unmasked, masks it
- Each CPU disables its own register page via `hv_call_set_vp_registers(HV_VP_INDEX_SELF)`
- Uses the MSR path (per-CPU) rather than the hypercall-by-VP-index path, exercising a
  different code path as the Level 1 fix

**Upstream status**: The `mshv_vtl` driver was upstreamed ~4 months ago (visible in
`torvalds/linux` master). Jork Loeser's commit `5170a82` ("x86/hyperv: Skip LP/VP creation
on kexec", merged Apr 22, 2026) shows active upstream kexec work for Hyper-V, but addresses
the root partition (mshv_root) LP/VP creation path, not the VTL2 stale register page/SINT0
problem. The stale state fix should be submitted upstream separately.

### Boot Sequence: `boot_cpus=0` vs All-CPUs Boot

The `boot_cpus=0` kernel command line parameter controls how many CPUs are brought up during
the kernel's SMP initialization phase. This has critical implications for kexec because it
determines whether stale hypervisor state from the old kernel can cause corruption.

#### With `boot_cpus=0` (old bzImage kexec path — no kernel changes needed)

```
┌─────────────────────────────────────────────────────────────────────┐
│ Kernel Boot (VP0 only)                                              │
│                                                                     │
│  SMP init                                                           │
│  ├── "Bringing up secondary CPUs..."                                │
│  │     └── boot_cpus=0 → skip all secondary VPs                    │
│  │     └── "Brought up 1 node, 1 CPU"     ◄── only VP0 running     │
│  │                                                                  │
│  ├── Driver init, initramfs unpack, /underhill-init                 │
│  ├── hv_vmbus init                                                  │
│  └── Module load: hv_storvsc, pci-hyperv                            │
│                                                                     │
│  mshv_vtl_init (initcall)                                           │
│  ├── hv_vtl_setup_synic()                                           │
│  │     └── cpuhp_setup_state("hyperv/vtl:online",                   │
│  │           mshv_vtl_alloc_context)                                │
│  │                                                                  │
│  └── For each secondary VP (1..31):                                 │
│        ├── mshv_vtl_alloc_context(cpu)                              │
│        │     └── Sets up register page, SINTs ◄── CLEAN setup       │
│        └── hv_vtl_bringup_vcpu(cpu)                                 │
│              └── HVCALL_START_VP                                    │
│                    └── VP wakes with CLEAN state                    │
│                        (driver just configured it)                  │
│                                                                     │
│  Result: All 32 VPs running, properly initialized ✅                │
└─────────────────────────────────────────────────────────────────────┘
```

**Key insight**: The `mshv_vtl` driver **owns** VP bringup. It configures the register page
and SINTs for each VP *before* calling `HVCALL_START_VP`. Any stale state from the old kernel
is overwritten before the VP ever executes in the new kernel. The stale state is harmless
because nothing reads it before the driver replaces it.

**Timing from logs** (this log, `logs-jan2.txt`):
- `[0.580452]` — `smp: Bringing up secondary CPUs...`
- `[0.580930]` — `smp: Brought up 1 node, 1 CPU` (0.5ms — no secondary VPs started)
- `[~1.0s]` — `mshv_vtl_init` starts secondary VPs with proper state setup

#### Without `boot_cpus=0` (stub kexec path — kernel changes required)

```
┌─────────────────────────────────────────────────────────────────────┐
│ Kernel Boot (All 32 VPs)                                            │
│                                                                     │
│  SMP init                                                           │
│  ├── "Bringing up secondary CPUs..."                                │
│  │     └── For each secondary VP (1..31):                           │
│  │           └── hv_vtl_wakeup_secondary_cpu(cpu)                   │
│  │                 └── hv_vtl_bringup_vcpu(cpu)                     │
│  │                       └── HVCALL_START_VP                        │
│  │                             └── VP wakes with STALE state! ⚠️    │
│  │                                   │                              │
│  │                                   ├── Register page → old phys   │
│  │                                   │   addr → hypervisor writes   │
│  │                                   │   to reused memory           │
│  │                                   │                              │
│  │                                   └── SINT0 unmasked → stale     │
│  │                                       vector → spurious IRQs     │
│  │                                                                  │
│  │     └── "Brought up 1 node, 32 CPUs" ◄── all VPs running        │
│  │                                                                  │
│  ├── ... kernel allocates memory, schedules tasks ...               │
│  │     └── 💥 Hypervisor writes to stale register page addresses    │
│  │     └── 💥 Corrupts task_structs, page tables                    │
│  │                                                                  │
│  ├── Driver init, initramfs                                         │
│  └── mshv_vtl_init                                                  │
│        └── hv_vtl_setup_synic()  ◄── TOO LATE, damage done          │
│              └── 💥 Hang at cpuhp_setup_state (CPU 18 unresponsive) │
│              └── 💥 NULL deref in scheduler (cpus_ptr == NULL)      │
└─────────────────────────────────────────────────────────────────────┘
```

**The problem**: `hv_vtl_wakeup_secondary_cpu()` in the vanilla kernel just calls
`HVCALL_START_VP` — it does NOT clean up register pages or SINTs first. On first boot this
is fine because VPs have no prior state (register page = 0, SINTs masked). After kexec,
the hypervisor still has the old kernel's register page addresses configured and starts
writing to them the moment each VP is dispatched to VTL2.

The corruption window is between SMP init (VPs start) and `mshv_vtl_init` (driver runs).
During this window, the hypervisor actively writes intercept data to physical addresses that
the new kernel has already allocated for its own data structures.

#### With kernel fix applied (stub kexec path — safe)

```
┌─────────────────────────────────────────────────────────────────────┐
│ Kernel Boot (All 32 VPs, with stale state cleanup)                  │
│                                                                     │
│  SMP init                                                           │
│  ├── "Bringing up secondary CPUs..."                                │
│  │     └── For each secondary VP (1..31):                           │
│  │           └── hv_vtl_wakeup_secondary_cpu(cpu)                   │
│  │                 ├── hv_vtl_cleanup_stale_vp_state(vp)  ◄── NEW   │
│  │                 │     ├── HV_REGISTER_REG_PAGE = 0               │
│  │                 │     └── SINT0 = masked                         │
│  │                 └── hv_vtl_bringup_vcpu(cpu)                     │
│  │                       └── HVCALL_START_VP                        │
│  │                             └── VP wakes with CLEAN state ✅     │
│  │                                                                  │
│  │     └── "Brought up 1 node, 32 CPUs"                             │
│  │                                                                  │
│  ├── ... kernel runs normally, no corruption ...                    │
│  │                                                                  │
│  └── mshv_vtl_init                                                  │
│        └── hv_vtl_setup_synic()                                     │
│              ├── on_each_cpu(cleanup_stale_state)  ◄── safety net   │
│              └── cpuhp_setup_state(alloc_context)  ◄── succeeds     │
│                                                                     │
│  Result: All 32 VPs running, properly initialized ✅                │
└─────────────────────────────────────────────────────────────────────┘
```

Boot CPU (VP0) cleanup happens even earlier — in `hv_vtl_early_init()` during `hyperv_init`,
before SMP init begins.

#### Normal servicing (no kexec — no `boot_cpus=` needed)

In the standard host-driven servicing path, the host tears down and recreates the Hyper-V
partition entirely. All VPs start with clean hypervisor state — register pages are zero and
SINTs are masked by default. There is no stale state to worry about.

The normal servicing command line has **no `boot_cpus=`** parameter at all, and all 32 CPUs
boot during SMP init:

```
Command line: loglevel=8 ... console=ttyS2,115200 hv_vmbus.message_connection_id=0x800074 ...
                                                   ^^^ no boot_cpus= anywhere

smpboot: x86: Booting SMP configuration:
.... node  #0, CPUs:   #2  #4  #6  #8 #10 #12 #14 #16 #18 #20 #22 #24 #26 #28 #30
                        #1  #3  #5  #7  #9 #11 #13 #15 #17 #19 #21 #23 #25 #27 #29 #31
smp: Brought up 1 node, 32 CPUs
smpboot: Total of 32 processors activated (19200.00 BogoMIPS)
```

This works because the partition is fresh — the hypervisor has no stale register page or
SINT configuration for any VP. The first-boot codepath in `hv_vtl_wakeup_secondary_cpu()`
is safe because VPs wake with default (clean) hypervisor state.

The `boot_cpus=0` parameter is only set by the `openhcl_boot` shim on first boot when
**sidecar** is enabled. With sidecar, the shim generates `boot_cpus=0,4,8,12,...` (one VP
per NUMA node) so that the sidecar kernel handles VP bringup. On servicing restore, sidecar
is disabled (`"sidecar: disabled because this is a servicing restore"`), so no `boot_cpus=`
is added and all CPUs SMP-boot normally.

#### `boot_cpus=0` as an alternative to the kernel fix

Using `boot_cpus=0` on the kexec command line avoids the need for kernel changes entirely.
The current stub path strips `boot_cpus=` from the command line in `build_cmdline()` (in
`kexec_prepare.rs`). Re-adding it would make the kernel changes unnecessary.

| Path | `boot_cpus=` | VPs during SMP init | Stale state? | Safe? |
|------|-------------|---------------------|--------------|-------|
| Normal servicing (host-driven) | not set | 32 | No — fresh partition | ✅ |
| First boot + sidecar | `0,4,8,...` | subset | No — fresh partition | ✅ |
| Kexec + `boot_cpus=0` | `0` | 1 (VP0 only) | Yes, but avoided | ✅ |
| Kexec, all CPUs, no fix | stripped | 32 | Yes — **crashes** | ❌ |
| Kexec, all CPUs + kernel fix | stripped | 32 | Yes — cleaned up | ✅ |

**Tradeoff**: `boot_cpus=0` delays secondary VP startup until `mshv_vtl_init`, whereas
booting all CPUs during SMP init gets them running earlier. For servicing blackout time,
the difference is negligible since the VP register restore (~2.3s bottleneck) happens
after all CPUs are up regardless. The kernel fix is preferred for defense-in-depth —
it makes the all-CPUs boot path safe after kexec regardless of command line options.

### Build & Test Steps

1. Build the stub for `x86_64-unknown-none`:
   ```bash
   MINIMAL_RT_BUILD=1 cargo build --target x86_64-unknown-none -p kexec_stub --release
   ```
2. Convert to flat binary (output to `target/kexec_stub.bin` — this is the path `rootfs.config` expects):
   ```bash
   objcopy -O binary target/x86_64-unknown-none/release/kexec_stub target/kexec_stub.bin
   ```
   The custom linker script (`openhcl/kexec_stub/link.x`) ensures `_start` is at offset 0 in the
   flat binary, which is where kexec's purgatory jumps after the 0x200-byte startup_32 padding.
3. Both steps must run **before** rootfs generation. `rootfs.config` picks up the binary from
   `target/kexec_stub.bin` and installs it as `/boot/kexec_stub.bin` in the rootfs.
4. Boot VTL2 → kexec should use the stub path → serial shows "kexec stub: alive" → vmlinux boots

### Issues Encountered

#### 1. `#![forbid(unsafe_code)]` in underhill_core

`underhill_core` uses `#![forbid(unsafe_code)]`, so the initial `memfd_create` implementation
using raw `libc::memfd_create` + `OwnedFd::from_raw_fd` caused a build error.

**Error**: `error: usage of an unsafe block` in `kexec_prepare.rs`

**Fix**: Moved the unsafe `memfd_create` wrapper into `kexec_sys` (which already has
`#![allow(unsafe_code)]` and exists specifically to isolate unsafe syscalls) as a safe
`pub fn memfd_create(name: &CStr) -> io::Result<OwnedFd>` API. `underhill_core` calls the
safe wrapper.

#### 2. ENOSPC writing temp files to tmpfs

The initial implementation wrote the synthetic bzImage and packed blob (vmlinux + initrd, ~22 MB)
to `/tmp` as temp files before passing them to `kexec_file_load`. VTL2's tmpfs is very limited,
and by the time `kexec_prepare` runs, `/tmp/initramfs.gz` (~8 MB) plus the rootfs contents
(now including the 14 MB vmlinux) already consume most of the available space.

**Error**: `failed to write packed blob: No space left on device (os error 28)`

**Fix**: Replaced tmpfs temp files with `memfd_create` — a Linux syscall that creates anonymous
in-memory file descriptors backed by kernel page cache, not tmpfs. The fd is passed directly to
`kexec_file_load`. This avoids consuming any tmpfs space.

```
Before: write data → /tmp/kexec_packed.blob (tmpfs) → open → fd → kexec_file_load
After:  memfd_create → fd → write data to fd → kexec_file_load
```

#### 3. OOM during kexec_file_load

With VTL2 at 128 MB, the kernel's `kexec_file_load` triggered an OOM panic while trying to
`vmalloc` ~22 MB to read the packed blob from the memfd. The kernel was using `kernel_read_file`
which vmallocs the entire file into kernel memory.

Peak userspace memory was also excessive because `std::fs::read(vmlinux)` (14 MB) +
`std::fs::read(initrd)` (8 MB) + `construct_packed_blob()` (22 MB copy) were all alive
simultaneously (~44 MB in userspace heap alone).

**Error**: `Kernel panic - not syncing: Out of memory: system-wide panic_on_oom is enabled`
(only 444 KB free with 147 MB managed)

**Fix (userspace)**: Refactored `prepare_kexec_stub()` to stream vmlinux and initrd directly
from their files to the memfd using `io::copy`, avoiding large heap allocations. Only the 24-byte
pack header is constructed in memory. Peak userspace overhead: ~0 bytes (just the `io::copy`
buffer).

**Fix (kernel-side)**: Temporarily increased VTL2 memory from 128 MB to 256 MB (32768 → 65536
pages in `openhcl-x64-release.json`). The `kexec_file_load` syscall still does a ~22 MB kernel
vmalloc that needs room. This can be reverted to 128 MB once Approach 2 (IGVM PageData) removes
vmlinux from the packed blob, shrinking it from ~22 MB to ~8 MB (initrd only).

#### 4. vmlinux size in rootfs

The rootfs.config vmlinux path must point to the **stripped** vmlinux (`build/vmlinux`, ~14 MB),
not the unstripped build output (`build/linux/vmlinux`, ~97 MB with debuginfo). Both paths
resolve from `${OPENHCL_MODULES_PATH}/../../build/`. The stripped version compresses to ~3 MB
in the cpio.gz rootfs, adding ~3 MB to the final IGVM (from ~24 MB to ~27 MB).

#### 5. `cmdline_size` at wrong offset in synthetic bzImage header

The synthetic bzImage header in `construct_bzimage()` wrote the `cmdline_size` field to
absolute offset **0x228** instead of the correct **0x238**. This is because the
`setup_header` struct is packed (no padding) and starts at offset 0x1F1 within the boot sector.
A manual count of preceding fields was off by one field.

**Effect**: Offset 0x228 is actually `cmd_line_ptr`, which got set to 0xFFFF (garbage pointer).
The real `cmdline_size` at 0x238 stayed zero, so the kernel compared `cmdline_len > 0` → true
and rejected the bzImage.

**Error**: `kexec-bzImage64: Kernel command line too long` → `EINVAL`

**Fix**: Changed `header[0x228..0x22C]` to `header[0x238..0x23C]`. Verified all other header
field offsets against the kernel's `struct setup_header` (in
`arch/x86/include/uapi/asm/bootparam.h`) — they were all correct; only `cmdline_size` was off
by 16 bytes (0x10), corresponding to the `cmd_line_ptr` (4) + `initrd_addr_max` (4) +
`kernel_alignment` (4) + `relocatable_kernel` (1) + `min_alignment` (1) + `xloadflags` (2)
fields between them.

#### 6. `_start` not at offset 0 in flat binary

The default linker layout placed `_start` at offset 0x2c8c within the ELF, but the kexec
purgatory jumps to byte 0 of the flat binary (after the 0x200-byte startup_32 padding).

**Fix**: Added a custom linker script (`openhcl/kexec_stub/link.x`) that places `.text.entry`
first, and added `.section .text.entry, "ax"` to `entry.S` so `_start` lands in that section.
Also needed to keep relro sections (`.dynamic`, `.got`) contiguous to satisfy the linker.

#### 7. PIE binary with unresolved relocations → hang after kexec

After fixing all the above, `kexec_file_load` succeeded and the kernel printed
`kexec_core: Starting new kernel`, but execution hung — no serial output from the stub.

**Root cause**: The stub was compiled as a PIE (Position-Independent Executable) with 41
`R_X86_64_RELATIVE` relocations. When `objcopy -O binary` produces the flat binary, these
relocations are baked in assuming a base address of 0. But the kexec handler loads the PM
kernel at an arbitrary `kernel_load_addr`, so all relocated addresses (function pointers,
vtables, panic handler addresses) pointed to wrong locations. The stub's code itself uses
RIP-relative addressing (which works at any load address), but any indirect call through a
relocated pointer would jump to garbage.

**Error**: Silent hang after `kexec_core: Starting new kernel` — no serial output, no panic,
just a crash into unmapped memory on the first indirect call.

**Fix**: Added `-no-pie` linker flag in `build.rs`:
```rust
println!("cargo:rustc-link-arg=-no-pie");
```
This produces a static `EXEC` binary instead of a `DYN` PIE, eliminating all 41 relocations.
The resulting flat binary works correctly at any load address since all code uses RIP-relative
addressing and there are no absolute address references to fix up.

#### 8. `static_mut_refs` lint error

Rust 2024 compatibility lints flagged `&mut NEW_BOOT_PARAMS` as deprecated.

**Fix**: Changed to `&raw mut NEW_BOOT_PARAMS` + `core::ptr::copy_nonoverlapping` +
`&mut *ptr` pattern.

#### 9. `unused-qualifications` warnings

Used `std::path::Path::new(...)` when `Path` was already imported at the top of the file.

**Fix**: Changed to `Path::new(...)`.

#### 10. GOT entries with absolute addresses → hang persists after `-no-pie` fix

After applying `-no-pie` (Issue #7), the binary had zero `.rela.dyn` relocations and was type
EXEC, yet it **still hung** after `Starting new kernel`. Investigation revealed:

- The ELF still had a `.got` section with 13 entries containing absolute base-0 addresses
  (e.g., `0x0a60`, `0x0060`, `0x0720` — function addresses within the stub)
- Disassembly showed GOT-indirect calls: `jmp *0x256d(%rip)` reading from `.got`
- The **compiler** (specifically pre-compiled `core` library for `x86_64-unknown-none`) generates
  PIC code that uses the GOT for function pointers, even when the linker produces a non-PIE binary
- `-no-pie` only controls the **linker** output type; it doesn't change how `core`/`alloc` were
  compiled (they ship as PIC for `x86_64-unknown-none`, which defaults to `code-model: kernel`)
- Tried `-Crelocation-model=static` via both `RUSTFLAGS` and `.cargo/config.toml` — this only
  affects the user crate, not the pre-compiled `core`, so the GOT persisted

**Root cause**: The GOT entries were filled with base-0 absolute addresses by the linker. When
kexec loads the flat binary at a high physical address (e.g., `0x416bf...`), any indirect call
through the GOT reads address `0x0a60` instead of `load_base + 0x0a60`, jumping to unmapped or
wrong memory. Unlike the `.rela.dyn` approach (which the dynamic linker would process), a static
EXEC binary has no mechanism to fix up its GOT at load time.

**Fix**: Self-relocation at startup. Reverted `-no-pie` (allowing the PIE build with its 41
`R_X86_64_RELATIVE` relocations) and added runtime relocation processing:

1. **`link.x`**: Added `__rela_start`/`__rela_end` markers around `.rela.dyn` so the relocation
   table is preserved in the flat binary and addressable at runtime
2. **`entry.S`**: Added a self-relocation loop **before** BSS zeroing:
   - Computes `load_base = lea _start[rip]` (linked at 0, so runtime addr = load delta)
   - Iterates all `Elf64_Rela` entries (24 bytes each: r_offset, r_info, r_addend)
   - For each `R_X86_64_RELATIVE` (type 8): writes `*(base + r_offset) = base + r_addend`
   - Skips if loaded at address 0 (no fixup needed)
3. **`build.rs`**: Removed `-no-pie` to keep the PIE build with its relocation table

Result: The flat binary grew from ~10 KB to ~12 KB (the `.rela.dyn` data is included). At
runtime, the 41 GOT and vtable entries are correctly patched before any Rust code executes.

#### 11. Packed blob at VTL0 GPA → stub can't read initrd

The initial approach passed the packed blob (vmlinux + initrd) as the kexec "initrd", so
`kexec_file_load` placed it at a physical address chosen by the kernel. After kexec, the stub
read `boot_params.ramdisk_image` to find the packed blob. However, the blob ended up at a VTL0
GPA (below 0x408000000) — memory that VTL2 cannot access after kexec because the old kernel's
page tables are gone and the stub's identity mapping only covers VTL2-accessible ranges.

**Error**: Stub printed `packed blob at 0x...` with a low VTL0 address, then hung or read
garbage when trying to validate the pack header magic.

**Fix**: Embed the packed blob **inside the kernel file** (the synthetic bzImage) instead of
passing it as the initrd. `construct_bzimage()` appends the packed blob after the stub binary,
and patches the blob's offset and size into the stub's binary header at bytes 8–23
(`pack_offset` at byte 8, `pack_size` at byte 16). The stub reads these values at runtime via
`entry.S` symbols and computes `packed_blob_addr = kernel_load_addr + pack_offset`.

The `KEXEC_FILE_NO_INITRAMFS` flag (0x04) was added to `kexec_file_load` flags so the kernel
doesn't try to interpret the packed blob as an initrd.

| Component | Change |
|-----------|--------|
| `entry.S` | Added `pack_offset` (byte 8) and `pack_size` (byte 16) with a `jmp` at byte 0 to skip over them |
| `main.rs` | Reads pack info from `kernel_load_addr + 8/16` instead of `boot_params.ramdisk_image` |
| `kexec_prepare.rs` | Patches stub binary bytes 8–23 with offset/size; passes `KEXEC_FILE_NO_INITRAMFS` |
| `kexec_sys/src/lib.rs` | Added `flags` parameter to `kexec_file_load`, `KEXEC_FILE_NO_INITRAMFS = 0x04` |

#### 12. `ext_ramdisk_image` at wrong boot_params offset

The stub's `boot_params.rs` wrote the `ext_ramdisk_image` (upper 32 bits of 64-bit ramdisk
address) and `ext_ramdisk_size` fields at offsets **0x1C0** and **0x1C4**. The correct offsets
are **0x0C0** and **0x0C4** within `struct setup_header` (which starts at byte 0x1F1 in the
boot sector, but these fields are measured from the start of `setup_header`, not the boot
sector).

**Effect**: The kernel saw `ext_ramdisk_image = 0`, treated the initrd address as 32-bit only,
and placed it at the wrong physical address (or address 0).

**Error**: Kernel printed `RAMDISK: [mem 0x00000000-0x...]` with wrong addresses, or failed to
find the initrd entirely.

**Fix**: Changed offsets from `0x1C0`/`0x1C4` to `0x0C0`/`0x0C4` in `boot_params.rs`.

#### 13. Non-page-aligned initrd → `free_init_pages` WARNING

The real initrd was packed immediately after the vmlinux inside the blob, at a non-page-aligned
offset. When the kernel tried to free initrd memory after unpacking, the non-page-aligned start
address triggered a `WARN_ON` in `free_init_pages`.

**Error**: `WARNING: CPU: 0 PID: 1 at mm/page_alloc.c:... free_reserved_area+0x.../0x...`
during `Freeing initrd memory`.

**Fix**: Page-aligned the initrd within the packed blob. In `kexec_prepare.rs`,
`construct_packed_blob()` pads the vmlinux with zeros to the next 4K boundary before writing
the initrd:
```rust
let vmlinux_padded_end = (pack_header_size + vmlinux_size + 0xFFF) & !0xFFF;
// seek to vmlinux_padded_end, then write initrd
```
The stub's `main.rs` computes the initrd physical address as:
`packed_blob_phys + page_align(24 + vmlinux_size)` — ensuring it's page-aligned in physical
memory.

#### 14. vmlinux physical range not in e820 map

The vmlinux PT_LOAD segments are copied to physical addresses (e.g., 0x8000000–0x8C15FFF) that
were not included in the e820 memory map passed to the new kernel. The kernel warned about
`.text`, `.data`, and `.bss` not being in any e820 region.

**Error**: `Warning: .text .data .bss are not within e820 RAM areas` during early boot.

**Fix**: The stub adds an explicit e820 `E820_TYPE_RAM` entry covering the vmlinux physical
range (`phdr_min..phdr_max`, where `phdr_max` accounts for `p_memsz` including BSS). This is
done in `main.rs` after loading all PT_LOAD segments, using `add_e820_entry()` in
`boot_params.rs`.

#### 15. Page table coverage — stub builds runtime identity map

The inherited page tables from kexec purgatory only covered 0–4 GB. VTL2 memory starts at
0x408000000 (above 4 GB), so the stub couldn't access the packed blob, boot_params, or any
VTL2 memory.

**Fix**: The stub builds its own page tables at runtime in `entry.S`. It allocates page table
pages at `page_align(_end)` (after the stub's BSS) and constructs an identity mapping covering
0–512 GB using 1 GB huge pages (PML4 → PDPT with 512 entries). The page tables are loaded
via `mov cr3` before jumping to Rust code.

#### 16. Kernel hang in `mshv_vtl_init` → stale VTL hypervisor state after kexec

After all the above fixes, the kernel booted fully (32 CPUs, drivers, initramfs unpacked) but
hung during the `mshv_vtl_init` initcall — specifically at `hv_vtl_setup_synic` →
`cpuhp_setup_state(mshv_vtl_alloc_context)`. The per-CPU callback would complete for CPUs 0–17
then hang waiting for CPU 18.

**Root cause**: The old kernel's `hv_kexec_handler()` only cleans up VMBus SynIC state (SINT2,
SIMP, SIEFP, SCONTROL) via `cpuhp_remove_state(hyperv_cpuhp_online)` → `hv_synic_cleanup()` →
`hv_synic_disable_regs()`. It does **not** clean up the mshv VTL-specific state because:

1. **No mshv VTL kexec handler**: The mshv_vtl module registers no `hv_setup_kexec_handler()`
2. **cpuhp teardown = NULL**: `cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "hyperv/vtl:online",
   mshv_vtl_alloc_context, NULL)` — the teardown callback is `NULL`
3. **`mshv_vtl_exit()` never called**: Module exit only runs on `rmmod`, not during kexec

This leaves the following stale on every CPU after kexec:

| State | Register/Mechanism | Effect |
|-------|--------------------|--------|
| **SINT0** (interception SINT) | `HV_MSR_SINT0 + HV_SYNIC_INTERCEPTION_SINT_INDEX` | Unmasked with old vector → spurious interrupts fire on CPUs that haven't installed handlers yet |
| **Register page** | `HV_REGISTER_REG_PAGE` hypercall | Hypervisor writes intercept data to old kernel's (now invalid) memory page |
| **Sidecar state** | Not torn down | Stale state from previous boot |

The **SINT0** being unmasked with a stale vector causes the hang: when the new kernel boots and
starts routing interrupts, the stale SINT fires on a CPU (e.g., CPU 18) that hasn't set up its
handler yet, causing the cpuhp IPI to that CPU to never complete.

**Normal kexec cleanup path** (for comparison):
```
hv_kexec_handler()               ← VMBus only, no mshv VTL cleanup
  ├── hv_stimer_global_cleanup()
  ├── vmbus_initiate_unload(false)   ← sends CHANNELMSG_UNLOAD to host
  ├── mb()
  └── cpuhp_remove_state(hyperv_cpuhp_online)
        └── hv_synic_cleanup(cpu)     ← per-CPU
              ├── hv_stimer_legacy_cleanup()
              └── hv_synic_disable_regs(cpu)
                    ├── mask SINT2 (VMBUS_MESSAGE_SINT)
                    ├── disable SIMP
                    ├── disable SIEFP
                    └── disable SCONTROL
```

**Fix**: Added `mshv_vtl_cleanup_stale_state()` function in `mshv_vtl_main.c`, called via
`on_each_cpu()` at the start of `hv_vtl_setup_synic()` **before** installing any handlers:

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

Called as: `on_each_cpu(mshv_vtl_cleanup_stale_state, NULL, 1);`

**Critical finding**: Both the SINT masking **and** the register page cleanup are required.
Testing with only the SINT masking (without register page cleanup) still hangs at CPU 17→18.
The register page cleanup is essential because the hypervisor continues to use the old kernel's
register page for delivering intercepts, and this interferes with cross-CPU IPI delivery.

With both cleanups, all 32 CPUs initialize successfully and `mshv_vtl_init` completes all 12
steps (mshv_setup_vtl_func, get_vsm_regs, apicid_to_cpuid_mapping, hv_vtl_setup_synic,
misc_register ×3, sidecar_init, mem_dev alloc, init_redirected_intr, init_memory, set_idle).

#### 17. Scheduler NULL pointer dereference — stale register page corruption

After kexec, the scheduler intermittently crashes with a NULL pointer dereference
in `_find_first_bit` called from `try_to_wake_up` → `select_task_rq` →
`cpumask_any(p->cpus_ptr)` where `p->cpus_ptr == NULL`.

```
BUG: kernel NULL pointer dereference, address: 0000000000000000
RIP: _find_first_bit+0x19/0x40   (RDI=NULL → cpumask pointer is NULL)
Call trace:
  try_to_wake_up → wake_up_process → cpu_stop_queue_work → stop_one_cpu_nowait
  → sched_balance_rq → sched_balance_domains → sched_balance_softirq
  → handle_softirqs → irq_exit_rcu → sysvec_hyperv_stimer0
  → mshv_vtl_idle
```

Affects different CPUs each time (CPU 1, CPU 19 observed). Always the same pattern:
the scheduler's load balancer tries to wake a stopper/migration thread that has a
corrupted `cpus_ptr` field (NULL instead of pointing to `cpus_mask`).

**Root cause**: After kexec, the old kernel's per-VP register pages (`HV_REGISTER_REG_PAGE`)
remain configured with stale physical addresses pointing into now-reallocated memory.
When `hv_vtl_bringup_vcpu()` starts a secondary VP via `HVCALL_START_VP`, the hypervisor
dispatches the VP to VTL2 and writes intercept context to the stale register page address.
This write corrupts whatever the new kernel allocated at that physical address (e.g., a
migration thread's `task_struct`, zeroing its `cpus_ptr` field).

The late cleanup in `mshv_vtl_init` (via `on_each_cpu(mshv_vtl_cleanup_stale_state)`) runs
AFTER SMP init has already started secondary CPUs, so the damage occurs before cleanup.

**Fix**: Move register page and SINT0 cleanup to run BEFORE each secondary CPU is started.
Added `hv_vtl_cleanup_stale_vp_state()` in `arch/x86/hyperv/hv_vtl.c`:
- Called from `hv_vtl_early_init()` for the boot CPU (runs during `hyperv_init`)
- Called from `hv_vtl_wakeup_secondary_cpu()` for each secondary VP (before `HVCALL_START_VP`)
- Uses `hv_call_set_vp_registers()` with explicit VP index to disable register page and
  mask SINT0 for the target VP from the boot CPU — no `on_each_cpu` needed
- The late cleanup in `mshv_vtl_main.c` is retained as a safety net

#### 18. End-to-end kexec servicing — WORKING

With the stale VTL state cleanup (issue #16 + #17) in place, the full kexec servicing path
completes successfully:

1. **Kernel boot**: All 32 CPUs initialized, `mshv_vtl_init` completes all steps
2. **underhill-init**: Starts, mounts filesystems, sets resource limits, loads kernel modules
   (`pci-hyperv-intf.ko`, `pci-hyperv.ko`, `hv_storvsc.ko`)
3. **Servicing state restore**: `payload_len=0x12913` read from persisted region
4. **GET protocol**: Negotiated `NICKEL_REV2`, DPS received with full device configuration
5. **Memory setup**: VTL2 RAM `0x408000000-0x418000000`, VTL0 RAM configured, alias map enabled
6. **NVMe VFIO**: PCI device `1751:00:00.0` [1414:00a9] enumerated, VFIO bound, NVMe driver
   restored with admin queue + 1 IO queue + 1 namespace (104857600 sectors)
7. **VMBus**: 13 channels restored (server + client), 4 GPADLs, connection version `0x60000`
8. **State units**: All devices restored and started — vmtime, ioapic, chipset, vmbus,
   vmbus_relay, partition, scsi, shutdown_ic, uefi, serial, pm, rtc, etc.
9. **VM resumed**: `blackout_time="6.5964899s"` — total time from kexec to guest VP resume
10. **Guest running**: Watchdog operating normally post-resume

**Blackout time breakdown** (6.60s total, 32 VPs):

| Phase | Duration | Notes |
|-------|----------|-------|
| VM stop (pre-kexec) | ~965ms | Device teardown, VP stop, VMBus close |
| kexec + stub execution | ~50ms | kexec_core → stub → vmlinux load |
| Kernel boot → init | ~946ms | SMP bringup (32 CPUs), driver init |
| Underhill init → GET | ~36ms | Filesystems, kernel modules, GET negotiation |
| Device setup + NVMe VFIO | ~910ms | PCI enumeration, VFIO bind, NVMe restore |
| Partition restore (VP regs) | **2,300ms** | Largest contributor — restores all 32 VP register states |
| State start (vmtime + devices) | ~478ms | vmtime sync, device start ordering |

**Key fix summary** — stale VTL state cleanup at two levels:
1. **Early cleanup** (`hv_vtl_cleanup_stale_vp_state` in `hv_vtl.c`):
   - Boot CPU: cleaned in `hv_vtl_early_init()` during `hyperv_init`
   - Secondary VPs: cleaned in `hv_vtl_wakeup_secondary_cpu()` before `HVCALL_START_VP`
   - Prevents register page corruption of new kernel memory during SMP init
2. **Late cleanup** (`mshv_vtl_cleanup_stale_state` in `mshv_vtl_main.c`):
   - Safety net via `on_each_cpu()` in `hv_vtl_setup_synic()`
   - Masks SINT0 and disables register page on all CPUs
