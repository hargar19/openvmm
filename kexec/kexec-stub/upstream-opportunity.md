# Upstream Opportunity: x86 vmlinux handler for kexec_file_load

## The Gap

`kexec_file_load` on x86 only supports bzImage. Three other architectures already have ELF (vmlinux) handlers:

| Architecture | ELF handler | Lines | Generic infra used |
|---|---|---|---|
| riscv | `arch/riscv/kernel/kexec_elf.c` | 145 | `kexec_elf_probe()` / `kexec_elf_load()` |
| loongarch | `arch/loongarch/kernel/kexec_elf.c` | 105 | `kexec_elf_probe()` / `kexec_elf_load()` |
| s390 | `arch/s390/kernel/kexec_elf.c` | 138 | `kexec_elf_probe()` / `kexec_elf_load()` |
| **x86** | **None** | — | — |

The kernel already provides generic ELF loading infrastructure in `kernel/kexec_elf.c` (430 lines) with `kexec_elf_probe()` and `kexec_elf_load()`. x86 is the only major architecture that doesn't use it.

## Why the Gap Exists

1. **bzImage is the universal x86 format** — distros ship bzImage, GRUB expects bzImage, kexec-tools defaults to bzImage. vmlinux is a build artifact, not a deliverable.

2. **x86 boot protocol complexity** — the bzImage handler's `setup_boot_parameters()` handles e820, EFI, DTB, IMA, KHO, RNG seed, EDD, screen info, ACPI RSDP. On riscv/loongarch, boot is simpler (just pass a device tree pointer).

3. **The decompressor handles hard parts** — bzImage's embedded decompressor does KASLR, 5-level paging setup, EFI handoff, relocation. A vmlinux handler skips all of that.

4. **No one with the use case has submitted the patch** — users who need `kexec_file_load` (distros, crash dump) use bzImage. Users who have vmlinux (kernel devs, embedded, VM firmware) mostly use the older `kexec_load` syscall.

## Upstream Justification (Universal, Not OpenHCL-Specific)

- `kexec_file_load` is the modern path: signature verification, KHO (Kexec Handover), IMA measurement log
- `kexec_load` supports vmlinux via userspace parsing, but `kexec_file_load` does not on x86
- Embedded x86, VMs, and kernel developers all use vmlinux directly
- Other architectures already have this — x86 is the outlier
- KHO hooks are only in the `kexec_file_load` path; without a vmlinux handler, vmlinux users can't use KHO on x86

## Implementation Plan

### Files to Change

| Component | Lines | Description |
|---|---|---|
| `arch/x86/Kconfig` | ~2 | `select KEXEC_ELF` under `KEXEC_FILE` |
| `arch/x86/kernel/kexec-elf64.c` | ~150–200 | New handler using generic `kexec_elf_probe()` / `kexec_elf_load()` + shared `setup_boot_parameters()` |
| `arch/x86/kernel/kexec-bzimage64.c` | ~-50 | Extract `setup_boot_parameters()` into shared code |
| `arch/x86/kernel/kexec-common64.c` | ~60 | Shared `setup_boot_parameters()` (e820, EFI, DTB, IMA, KHO, RNG, EDD) |
| `arch/x86/kernel/machine_kexec_64.c` | ~1 | Add `&kexec_elf64_ops` to `kexec_file_loaders[]` |
| `arch/x86/kernel/Makefile` | ~1 | Build the new file |
| `arch/x86/include/asm/kexec-elf64.h` | ~5 | Header for the new ops struct |

**Total: ~200–250 lines of new code.**

### Handler Structure

```c
// arch/x86/kernel/kexec-elf64.c

static int elf64_probe(const char *buf, unsigned long len)
{
    // Use generic kexec_elf_probe() — validates ELF magic, class, endianness
    // Additional check: EM_X86_64
}

static void *elf64_load(struct kimage *image, char *kernel,
                        unsigned long kernel_len, char *initrd,
                        unsigned long initrd_len, char *cmdline,
                        unsigned long cmdline_len)
{
    // 1. Parse ELF with kexec_elf_load() — extracts PT_LOAD segments,
    //    calls kexec_add_buffer() for each
    // 2. Load purgatory
    // 3. Load initrd
    // 4. Allocate boot_params + cmdline buffer
    // 5. Set up purgatory entry64_regs:
    //      RIP = ELF entry point (NOT +0x200 like bzImage)
    //      RSI = boot_params address
    // 6. Call shared setup_boot_parameters() for e820/DTB/KHO/IMA/etc.
}

const struct kexec_file_ops kexec_elf64_ops = {
    .probe   = elf64_probe,
    .load    = elf64_load,
    .cleanup = elf64_cleanup,
};
```

### Key Difference from bzImage Handler

The bzImage handler jumps to `kernel_load_addr + 0x200` (the protected-mode entry after setup sectors). The ELF handler jumps directly to the ELF entry point. Everything else — boot_params, purgatory, initrd, setup_data chain — is identical.

### Registration

```c
// arch/x86/kernel/machine_kexec_64.c
const struct kexec_file_ops * const kexec_file_loaders[] = {
    &kexec_bzImage64_ops,
    &kexec_elf64_ops,       // NEW — tried after bzImage probe fails
    NULL
};
```

## Potential Review Concerns

### 1. KASLR

vmlinux loaded to its ELF link address bypasses KASLR. The bzImage decompressor randomizes the load address; the ELF handler would load at `p_paddr` from program headers.

**Mitigation:** riscv and loongarch handlers don't do KASLR either — there's precedent. Document that this disables randomization. Could add a `CONFIG_` guard or a warning in dmesg.

### 2. Signature Verification

bzImage uses PE signature verification (`kexec_kernel_verify_pe_sig`). vmlinux doesn't have PE signatures.

**Mitigation:** s390's ELF handler has no `verify_sig` callback — acceptable for v1. Could add ELF-based or module signing verification later.

### 3. Relocation

Requires `CONFIG_RELOCATABLE=y` for the kernel to be loadable at addresses other than its link address. Without it, the kernel must be loaded at its exact compiled address.

**Mitigation:** Follow riscv's pattern. Most x86 kernels have `CONFIG_RELOCATABLE=y` by default. The handler can check the ELF for `ET_DYN` (PIE) vs `ET_EXEC` and handle accordingly.

### 4. Decompressor Features Skipped

By loading vmlinux directly, these bzImage decompressor features are bypassed:
- 5-level paging early setup
- EFI stub handoff
- Early console
- Memory encryption (SEV) setup

**Mitigation:** The purgatory and kernel early boot code handle most of this. Document which configurations are supported.

## Submission Strategy

### Patch Series (3 patches)

1. **Patch 1: Refactor** — Extract `setup_boot_parameters()` and related helpers from `kexec-bzimage64.c` into `kexec-common64.c`. No functional change. This is the largest patch but easiest to review (pure code movement).

2. **Patch 2: Add ELF handler** — New `kexec-elf64.c` using generic `kexec_elf_probe()`/`kexec_elf_load()` + shared `setup_boot_parameters()`. Wire up in `machine_kexec_64.c` and `Kconfig`.

3. **Patch 3: Documentation** — Update `Documentation/arch/x86/boot.rst` and kexec man page notes.

### Mailing Lists and Reviewers

- `x86@kernel.org` (x86 maintainers)
- `kexec@lists.infradead.org` (kexec maintainers)
- KHO maintainers (confirm KHO works with ELF path)
- CC: authors of riscv/loongarch handlers (Liao Chang, Huacai Chen) for cross-reference

### Cover Letter Talking Points

- "x86 is the only major architecture without ELF support in kexec_file_load"
- "The generic kexec_elf infrastructure has been stable since 2016 (s390)"
- "This enables KHO and IMA for vmlinux users on x86"
- Reference commit `fc9c112f804ab` (loongarch ELF handler) as recent precedent

## Relation to OpenHCL / Kexec Stub

This upstream work does **not** directly replace the kexec stub. The stub solves a different problem: loading vmlinux from physical memory without a file descriptor. The upstream handler uses the standard fd-based interface.

However, if the upstream ELF handler is merged:

1. **Eliminate the stub for fd-based case** — `kexec_prepare.rs` can pass vmlinux fd directly to `kexec_file_load()` instead of building a synthetic bzImage. This removes the entire `kexec_stub` crate and the bzImage header construction.

2. **Simplify kexec_prepare.rs** — from ~240 lines of stub orchestration to ~20 lines (open vmlinux fd, open initrd fd, call `kexec_file_load`).

3. **Get proper boot_params** — the kernel builds authoritative boot_params with fresh DTB, KHO, IMA, e820 instead of the stub's manual copy-and-patch approach.

4. **VTL2 initrd placement issue remains** — the kernel's `kexec_add_buffer()` for initrd may place it at a GPA inaccessible to VTL2. This needs a separate fix (e.g., OpenHCL-specific `MIN_INITRD_LOAD_ADDR` adjustment).

5. **Establishes upstream credibility** — positions you for follow-up patches (physical-address-based loading, VTL2-aware memory placement).

## Future Follow-Up: Physical Address Loading

After the ELF handler is upstream, a second patch series could add support for loading from a physical address (IGVM-resident vmlinux) without a file descriptor. This would be a new `KEXEC_FILE_FROM_PHYS` flag or a separate interface. The ELF handler groundwork makes this easier because:
- The handler already exists and handles boot_params/KHO/DTB
- Only the buffer source changes (memremap vs kernel_read_file_from_fd)
- The justification builds on the accepted ELF handler
