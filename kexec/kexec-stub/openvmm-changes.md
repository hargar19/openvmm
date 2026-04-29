# OpenVMM Changes for the Kexec Stub

All changes on branch `user/hargar/kexec-2026` (HEAD `135749e4`) relative to base `d341ae53`.

**16 files changed, ~1039 lines added.**

```
 Cargo.lock                                         |   8 +
 Cargo.toml                                         |   1 +
 openhcl/kexec_stub/Cargo.toml                      |  16 ++
 openhcl/kexec_stub/build.rs                        |  22 ++
 openhcl/kexec_stub/link.x                          |  61 ++++++
 openhcl/kexec_stub/src/arch/mod.rs                 |   5 +
 openhcl/kexec_stub/src/arch/x86_64/entry.S         | 122 +++++++++++
 openhcl/kexec_stub/src/arch/x86_64/mod.rs          |  14 ++
 openhcl/kexec_stub/src/boot_params.rs              | 124 +++++++++++
 openhcl/kexec_stub/src/elf.rs                      | 132 ++++++++++++
 openhcl/kexec_stub/src/main.rs                     | 229 ++++++++++++++++++++
 openhcl/kexec_stub/src/rt.rs                       |  48 +++++
 openhcl/kexec_sys/src/lib.rs                       |  28 ++-
 openhcl/rootfs.config                              |   6 +
 openhcl/underhill_core/src/dispatch/kexec_prepare.rs | 240 +++++++++++++++++++--
 vm/loader/manifests/openhcl-x64-release.json       |   2 +-
```

---

## Design Overview

The stub exists because `kexec_file_load` on x86 only has a bzImage handler — no vmlinux handler. Rather than adding a kernel patch, the stub wraps vmlinux in a fake bzImage. The kernel's kexec handler loads it like a normal bzImage, then the stub unpacks vmlinux and jumps to it. Everything the new kernel needs (e820, cmdline, DTB, KHO metadata) is inherited via the `boot_params` copy.

---

## 1. New Crate: `openhcl/kexec_stub/`

A `#![no_std]` Rust binary targeting `x86_64-unknown-none` that runs after kexec with **no OS, no allocator, no kernel** — just raw physical memory and identity mapping.

### Cargo.toml

Depends only on `minimal_rt` (OpenHCL's bare-metal runtime crate, also used by `openhcl_boot` and `sidecar`).

### build.rs

Tells rustc to use a custom linker script and keep PIE mode enabled (so the binary contains `R_X86_64_RELATIVE` relocations for self-relocation at runtime).

### link.x

Custom linker script that:
- Places `_start` at the very beginning (offset 0) of the flat binary
- Keeps `.rela.dyn` in a LOAD segment so `objcopy -O binary` preserves the relocation table
- Orders sections: `.text.entry` → `.text` → `.rodata` → `.dynamic` → `.got` → `.rela.dyn` → `.data` → `.bss`
- Exports `__rela_start`, `__rela_end`, `__bss_start`, `_end` symbols for assembly

### src/arch/x86_64/entry.S (122 lines)

Assembly entry point. Does five things in order:

1. **Saves RSI** (boot_params pointer from kexec purgatory)
2. **Self-relocation** — computes load delta from RIP-relative `_start` address, walks `.rela.dyn` entries, applies `R_X86_64_RELATIVE` fixups to GOT/vtable pointers
3. **Zeros BSS** — required because post-kexec memory has stale data
4. **Builds identity-mapped page tables** — 2-level (PML4 → PDPT with 1GB pages) covering 0–512 GB, because the purgatory's page tables may not cover the packed blob's location
5. **Sets up stack + SSE**, then jumps to Rust

The first 24 bytes contain a jump instruction followed by two patched-in u64 values: `pack_offset` and `pack_size` (written by `kexec_prepare.rs`).

### src/boot_params.rs (124 lines)

Minimal `boot_params` accessor using raw byte offsets instead of a full 4096-byte struct definition. Provides getters/setters for: ramdisk address/size, hardware_subarch, type_of_loader, e820 entries, setup_data chain. Keeps the crate dependency-free.

### src/elf.rs (132 lines)

`no_std` ELF64 parser. Validates magic/class/endianness, extracts up to 16 `PT_LOAD` segments (file_offset, phys_addr, file_size, mem_size), and the entry point. Used to parse the vmlinux embedded in the packed blob.

### src/main.rs (229 lines)

The core logic in `stub_main()`:

1. Reads inherited `boot_params` (has e820, cmdline, DTB setup_data chain from the running kernel)
2. Locates the packed blob via patched `pack_offset`/`pack_size` in the stub header
3. Validates pack magic `"KXSTUB\x01\x00"`, extracts vmlinux and initrd sizes
4. Parses the vmlinux ELF, copies each `PT_LOAD` segment to its target physical address, zeros BSS portions
5. Copies inherited `boot_params` to a static `NEW_BOOT_PARAMS`, updates ramdisk to point at the real initrd, sets `hardware_subarch=1` (disables VTL0-memory probing), adds kernel physical range to e820
6. Jumps to vmlinux entry point with `RDI=0, RSI=&new_boot_params`

### src/rt.rs (48 lines)

32 KB stack, stack cookie verification, `start()` entry called from assembly, panic handler that writes to serial port (`0x3F8`) via `minimal_rt`.

---

## 2. Modified: `openhcl/kexec_sys/src/lib.rs` (+28 lines)

Three additions to the kexec syscall wrapper crate:

- **`KEXEC_FILE_NO_INITRAMFS`** (0x04) — new flag constant so the stub path can call `kexec_file_load` without a separate initrd fd
- **`flags` parameter** added to `kexec_file_load()` — was hardcoded to 0, now caller-provided
- **`memfd_create()`** — new wrapper around `libc::memfd_create` returning `OwnedFd`. Used to construct the synthetic bzImage in memory without touching tmpfs

---

## 3. Modified: `openhcl/underhill_core/src/dispatch/kexec_prepare.rs` (+240/−18 lines)

The orchestrator that runs at servicing time (before kexec). Previously just loaded bzImage + initrd. Now has two paths.

### `prepare_kexec()` (modified)

Checks if `/boot/kexec_stub.bin` and `/boot/vmlinux` exist. If yes → stub path. If no → fallback to original bzImage path.

### `prepare_kexec_stub()` (new, ~100 lines)

Builds a synthetic bzImage in a memfd:

```
[1024-byte header][0x200 startup_32 pad][stub binary][zero pad to pack_start][packed blob]
```

Where packed blob = `[magic:8][vmlinux_size:8][initrd_size:8][vmlinux bytes][pad to page][initrd bytes]`

Key details:
- Patches `pack_offset` and `pack_size` into the stub binary at bytes 8–24
- `init_size` = total memory the kernel's kexec loader must reserve (covers stub + BSS + page tables + packed blob)
- 64 KB headroom past the stub binary for BSS and runtime page tables
- Initrd is page-aligned within the blob so `free_initrd_mem()` works
- Streams vmlinux and initrd from disk via `io::copy()` to avoid buffering ~30 MB in memory
- Calls `kexec_file_load` with `KEXEC_FILE_NO_INITRAMFS` flag (initrd is embedded, not separate)

### `prepare_kexec_bzimage()` (refactored)

The original path, now a named function with the `flags` parameter forwarded.

### `construct_bzimage_header()` (new, ~50 lines)

Builds a minimal 1024-byte bzImage header with:
- `setup_sects=1`
- `boot_flag=0xAA55`
- `HdrS` magic, protocol 2.15
- `LOADED_HIGH`
- `XLF_KERNEL_64 | XLF_CAN_BE_LOADED_ABOVE_4G | XLF_5LEVEL`
- `relocatable_kernel=1`
- `kernel_alignment=0x1000`
- `init_size` (total PM kernel memory)

---

## 4. Modified: `openhcl/rootfs.config` (+6 lines)

Adds two new files to the initrd:

- `/boot/vmlinux` — uncompressed kernel ELF (~30 MB, compressed in the cpio archive)
- `/boot/kexec_stub.bin` — the flat binary (~14 KB) built from `kexec_stub` via `objcopy -O binary`

---

## 5. Modified: `vm/loader/manifests/openhcl-x64-release.json` (1 line)

`memory_page_count`: 32768 → 65536 (128 MB → 256 MB). The extra memory accommodates the larger initrd (which now includes vmlinux) and the runtime memory needed for the packed blob in VTL2.

---

## 6. Workspace: `Cargo.toml` + `Cargo.lock`

Registers `openhcl/kexec_stub` as a workspace member and adds its lock entries (depends on `minimal_rt` and `minimal_rt_build`).

---

## Memory Layout at kexec Time

```
bzImage file in memfd (passed to kexec_file_load):
┌─────────────────────────────────┐  offset 0
│  1024-byte bzImage header       │  (setup_sects=1)
├─────────────────────────────────┤  offset 0x400
│  0x200 bytes startup_32 pad     │  (all zeros)
├─────────────────────────────────┤  offset 0x600 (PM kernel starts here)
│  stub flat binary (~14 KB)      │  entry.S + Rust code
│    bytes 8-16: pack_offset      │  (patched)
│    bytes 16-24: pack_size       │  (patched)
├─────────────────────────────────┤
│  zero padding (64 KB headroom)  │  for BSS + runtime page tables
├─────────────────────────────────┤  PM offset = pack_start (page-aligned)
│  PACKED BLOB:                   │
│    [magic "KXSTUB\x01\x00"]    │  8 bytes
│    [vmlinux_size: u64 LE]       │  8 bytes
│    [initrd_size: u64 LE]        │  8 bytes
│    [vmlinux ELF bytes]          │  ~30 MB
│    [padding to page boundary]   │
│    [initrd (compressed cpio)]   │  ~5 MB
└─────────────────────────────────┘  offset = init_size
```

## Execution Flow

```
1. kexec_prepare.rs builds synthetic bzImage → kexec_file_load()
2. Kernel's kexec-bzImage64 handler:
   - Allocates init_size bytes for PM kernel in VTL2 memory
   - Copies PM kernel (stub + packed blob) there
   - Builds boot_params (e820, cmdline, DTB setup_data, KHO if active)
   - Stages purgatory jump to kernel_load_addr + 0x200
3. reboot(KEXEC) → purgatory → entry.S:
   - Self-relocation (fix GOT entries)
   - Zero BSS
   - Build identity-mapped page tables (0–512 GB)
   - Set up stack + SSE
   - Jump to Rust stub_main()
4. stub_main():
   - Find packed blob via pack_offset/pack_size
   - Validate magic, parse vmlinux ELF
   - Copy PT_LOAD segments to target physical addresses
   - Build new boot_params (inherit e820/cmdline/DTB, update ramdisk)
   - Jump to vmlinux entry point
5. Linux kernel boots with full boot_params
   - Sees DTB in setup_data → /sys/firmware/fdt
   - Sees initrd at ramdisk_addr → unpacks rootfs
   - Underhill starts, reads persisted servicing state
```

## Key Design Decisions

| Decision | Rationale |
|---|---|
| Fake bzImage wrapper | Only format supported by `kexec_file_load` bzImage64 handler on x86 |
| Self-relocation via `.rela.dyn` | Stub is loaded at arbitrary kexec address; PIE relocations fix GOT entries |
| Identity-mapped 1GB pages | Purgatory page tables may not cover packed blob at high GPAs |
| Embedded packed blob (not separate initrd) | Separate initrd placed at low GPA by kexec → inaccessible in VTL2 |
| `memfd_create` for bzImage | Avoids writing ~35 MB to tmpfs; stays in memory |
| Streaming `io::copy` for vmlinux/initrd | Avoids buffering ~30 MB of vmlinux in userspace heap |
| `hardware_subarch=1` | Prevents kernel from probing VTL0 ROM/BIOS memory during boot |
| Kernel range added to e820 | Required for `free_initmem()` — without it, kernel hangs freeing __init pages |
| Fallback to bzImage path | Graceful degradation if stub or vmlinux missing from rootfs |
