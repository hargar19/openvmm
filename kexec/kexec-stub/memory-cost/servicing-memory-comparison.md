# Servicing Memory Cost Comparison

---

## How Current Servicing Uses vmlinux to Boot VTL2

### Build Time (Developer/CI machine)

The vmlinux file is a standard ELF binary (~14 MB) produced by building the Linux kernel. It contains:
- ELF headers describing the memory layout
- PT_LOAD segments: raw executable code (.text), initialized data (.data), and BSS size (.bss)
- Each PT_LOAD segment has a `phys_addr` field specifying where it must reside in physical memory to execute

`igvmfilegen` opens the vmlinux, reads only the PT_LOAD segments and their target physical addresses, splits the raw bytes into 4 KB pages, and emits one IGVM PageData directive per page. Each directive says: "write these 4096 bytes to GPA X." It also records the kernel entry point (from the ELF header) as the initial RIP for VP0.

After this, the vmlinux ELF file is no longer needed. The IGVM contains just the raw page content and target addresses — no ELF structure, no headers, no symbols.

### Host Disk (Deployment)

The `.igvm` file sits on the host's local storage. Inside it: ~3500 PageData directives (one per 4 KB page), each carrying raw kernel bytes and the GPA where they belong. This is the host's "spare copy" for reload — it costs only host disk space, not RAM.

### VM Creation / Servicing Reload

The host opens the IGVM file from disk and iterates the PageData directives. For each one, it asks the hypervisor to:
1. Allocate a host physical DRAM page (4 KB of real host DRAM)
2. Copy the directive's content into it
3. Map it into the guest's physical address space at the specified GPA

After all pages are loaded, the host sets VP0's initial registers (RIP = entry point, CR3 = page tables, etc.) and starts the VP.

### Result: Kernel in Guest Physical Memory (RAM)

The vmlinux content now resides in guest physical memory — real RAM backed by host DRAM. This is where the CPU will fetch and execute instructions. A CPU can only execute from RAM — there is no alternative. The kernel is always in RAM at runtime.

```
Guest physical memory after loading:

  GPA 0x1000000 ── 8 MB of .text + .rodata (executable code)
  GPA 0x1800000 ── 4 MB of .data (initialized globals)
  GPA 0x1C00000 ── 2 MB of .bss (zeroed)
                   ──────────────────────────
                   Total: ~14 MB of kernel image IN RAM

  These pages:
  ✓ Are real RAM (guest physical memory backed by host DRAM)
  ✓ Are directly accessible by the guest CPU (RIP points here, instructions execute from here)
  ✓ Are NOT managed by the kernel's memory allocator (placed before allocator existed)
  ✓ Exist outside the kernel's "quota" (not counted in MemAvailable)
  ✓ Cannot be freed, reclaimed, or paged out — they ARE the running kernel
```

The kernel boots and initializes its memory allocator with the REMAINING free pages (everything after the kernel image). The kernel never allocates or frees its own code/data pages — they were placed before it existed.

**Important**: The vmlinux content is in guest RAM in every approach, always. The 14 MB is an unavoidable baseline cost of having a kernel. The question for kexec is whether you need a SECOND copy in RAM for reload, since the first copy becomes dirty (modified .data/.bss, patched .text) during execution and cannot be reused as-is.

### During Servicing (Host-Driven Reload)

```
Step 1: VTL2 saves device/VMBus state, sends to host
Step 2: Host DESTROYS the partition
         → All guest physical pages freed (14 MB kernel + everything else)
         → Guest RAM cost goes to zero
Step 3: Host reads IGVM from disk (costs disk I/O, not RAM)
Step 4: Host creates NEW partition, loads pages fresh from IGVM
         → New 14 MB of clean kernel pages allocated and placed
Step 5: New VTL2 boots, restores saved state

At NO point are there two copies of the kernel in RAM simultaneously.
The spare copy is the IGVM file on disk — free from a RAM perspective.
```

### Memory Cost Summary

| Resource | Cost | Type |
|----------|------|------|
| Running kernel (always) | 14 MB | Guest physical memory (host DRAM) — outside kernel quota |
| Spare copy for reload | 0 | Host disk (IGVM file) |
| Extra RAM for servicing | 0 | Host frees old pages before allocating new ones |

Current servicing uses **zero extra RAM** for reload capability because the host acts as an external loader reading from its own disk. The guest never needs to store a spare kernel copy.

---

## Key Questions Answered

### Q1: Where is vmlinux stored for current servicing?

**On host disk** — inside the IGVM file. At build time, igvmfilegen extracts the vmlinux PT_LOAD content and bakes it into the IGVM as PageData directives. At servicing time, the host re-reads this file from its own storage and writes the pages into guest physical memory. The guest has no access to this file and doesn't know it exists.

### Q2: Why can't we use the same location (host disk) for bzImage/kexec?

Because kexec is **guest-driven**. The guest calls `kexec_file_load()` which needs a file descriptor the guest kernel can read from. The guest cannot read from host disk — it has no access to the IGVM file or any host filesystem. The host loads pages externally via hypervisor APIs; the guest cannot invoke those APIs on itself.

For kexec, the clean kernel copy must be **inside guest-accessible memory** so the guest can read it and stage it. That's the fundamental difference: current servicing is host-driven (host reads from its own disk), kexec is guest-driven (guest reads from its own memory).

### Q3: Can we put bzImage at the same kind of location as the running kernel (host-provisioned, outside kernel quota)?

**Yes.** The host can deposit bzImage at a reserved GPA at boot time — same mechanism as placing the boot kernel. It would be outside the kernel's allocatable pool (e820-reserved). At kexec time, userspace reads from that reserved GPA (via `/dev/mem` or a driver) into a transient memfd, then passes the memfd fd to `kexec_file_load`.

Cost: 3.5 MB of host-provisioned guest physical memory (outside kernel quota). Transient: 3.5 MB from kernel pool during kexec prep only (freed immediately after staging).

### Q4: What is the best alternative to the stub kernel? What is the simplest way to kexec directly?

**Answer: Put bzImage in rootfs and use standard `kexec_file_load`.** This is the simplest approach:

```
/boot/bzImage (3.5 MB in rootfs/tmpfs)
  → open("/boot/bzImage") gives you an fd
  → kexec_file_load(fd, initrd_fd, cmdline, 0)
  → kexec -e
  → kernel handles decompression and loading internally
```

**Why this is best:**
- No stub kernel needed
- No custom ELF parser needed
- No reserved GPA protocol needed
- No host changes needed
- Standard Linux kexec API — well-tested, handles all the complexity
- The kernel's built-in bzImage loader decompresses, places segments, sets up boot_params
- Works today with your existing kernel (needs only the stale VP state cleanup patches)

**Cost:** 3.5 MB permanently from kernel's MemAvailable. For a 256 MB VTL2 VM, that's 1.4%.

**When would you NOT use this?**
- If 3.5 MB from kernel pool is unacceptable → use reserved GPA approach (more complex, zero kernel pool cost)
- If you need zero transient memory during prep → use reserved GPA + stub (most complex)
- If you need OOM-proof servicing → use reserved GPA (reserved pages can't be affected by OOM)

### Summary: Recommendation

| Priority | Approach |
|----------|----------|
| Simplest, ship fastest | bzImage in rootfs + `kexec_file_load` (2a) |
| Zero kernel pool cost | bzImage in reserved GPA + memfd + `kexec_file_load` (hybrid) |
| Zero kernel pool + zero transient | Reserved GPA + stub (2b) — most complex |

For initial implementation: **2a (bzImage in rootfs) is the clear winner.** The stub kernel can be eliminated entirely — `kexec_file_load` with a bzImage fd handles everything the stub does, and more (boot_params, initrd, etc.), with zero custom code.

The stub only made sense when:
- You wanted to avoid having bzImage/vmlinux in rootfs (but you need SOMETHING in guest memory regardless)
- You wanted to boot vmlinux directly without decompression (saves ~50ms, not worth the complexity)
- You wanted to avoid the kernel's kexec staging memory (14 MB transient — freed after exec anyway)

---

Three approaches to VTL2 servicing, analyzed for memory impact.

## Key Distinction: Host-Provisioned Memory vs. Guest Kernel-Allocatable Memory

Two fundamentally different memory pools:

1. **Host-provisioned guest physical memory**: The host decides how much total physical memory VTL2 gets. The host backs each GPA page with real host DRAM. The host can increase this budget to accommodate kexec infrastructure — the guest kernel never knows.

2. **Guest kernel-allocatable memory**: The subset of guest physical memory that the Linux kernel sees and can use for its own allocations (page cache, task structs, buffers, etc.). This is what shows up in `MemAvailable`. If something consumes memory from this pool, the kernel has less for actual work.

The IGVM approach (2b) costs host-provisioned memory — the host gives VTL2 a larger physical address space. The guest kernel is unaffected.

The rootfs approach (2a) costs guest kernel-allocatable memory — the kernel has less RAM for its own use. The host doesn't provision anything extra.

---

## 1. Current Servicing (Host-Driven Partition Reload)

### How vmlinux is used

- **Build time**: `igvmfilegen` reads vmlinux, extracts PT_LOAD segments, emits IGVM PageData directives
- **VM creation**: Host parses IGVM, writes PageData into guest physical memory (GPAs)
- **VTL2 boots**: Kernel executes directly from those GPAs — the PT_LOAD pages ARE the running kernel
- **Servicing**: Host tears down partition, re-reads IGVM from host disk, writes pages fresh into guest physical memory again

### Memory used

| What | Size | Memory type | Who pays |
|------|------|-------------|----------|
| Running VTL2 kernel (.text + .data + .bss) | ~14 MB | Host-provisioned GPA | Host DRAM |
| Spare copy for reload | **0** | Host disk (IGVM file) | Host storage |
| Initrd/rootfs (no kernel binary in it) | ~20 MB | Host-provisioned GPA (tmpfs) | Host DRAM |

### Cost summary

- **Extra host DRAM for reload capability**: 0 — host loads from its own disk
- **Guest kernel-allocatable RAM lost**: 0
- **Peak transient during servicing**: 0 guest RAM — host does everything externally

### Why zero extra cost

The host acts as an external loader. It reads the IGVM from host-local storage and injects pages directly into guest physical memory via hypervisor APIs. The guest never stores or sees a "spare" kernel binary.

---

## 2a. Kexec with bzImage in Rootfs

### How bzImage is used

- **Build time**: bzImage (~3.5 MB compressed) added to rootfs via `rootfs.config`
- **Boot**: bzImage lands in initramfs → tmpfs → guest RAM at `/boot/bzImage`
- **Kexec prep** (guest running): Open `/boot/bzImage` fd → `kexec_file_load(fd, initrd_fd, ...)`
- **Kexec exec** (after state save): `kexec -e` → kernel decompresses bzImage internally and jumps

### Memory used

| What | Size | Memory type | Who pays | Lifetime |
|------|------|-------------|----------|----------|
| Running VTL2 kernel | ~14 MB | Host-provisioned GPA | Host DRAM | Always |
| bzImage in rootfs (tmpfs) | ~3.5 MB | **Guest kernel-allocatable** (tmpfs) | **Guest kernel pool** | Permanent |
| `kexec_file_load` staging (kimage) | ~14 MB | Guest kernel-allocatable (kmalloc) | Guest kernel pool | Transient (freed at `kexec -e`) |
| Initrd build (transient) | ~10-50 MB | Guest kernel-allocatable (memfd) | Guest kernel pool | Transient (freed after staging) |

### Cost summary

- **Extra host DRAM provisioned**: 0 (uses VTL2's existing memory budget)
- **Guest kernel-allocatable RAM lost permanently**: 3.5 MB
- **Peak transient from guest kernel pool**: +14-64 MB during prep (freed after)

### What "guest kernel pays" means

The 3.5 MB bzImage lives in tmpfs, which is backed by the **same** guest physical memory pool the kernel uses for everything else. The kernel has 3.5 MB less for page cache, task structs, driver allocations, etc. It counts against `MemAvailable`. Under memory pressure, this 3.5 MB is memory that could have been used for real work.

The host doesn't provision any extra memory — VTL2's total physical memory budget stays the same as today. The cost is absorbed entirely within the guest kernel's existing allocation.

---

## 2b. Kexec with IGVM PT_LOAD Loaded to Reserved GPA + Stub

### Concept

The IGVM already contains vmlinux PT_LOAD page data — it's what the host uses to boot VTL2. Currently, the guest has no awareness of the IGVM. The host loads the pages into guest physical memory, the kernel starts executing, and the IGVM is never referenced again.

**Proposal**: At boot time, the host loads the same PT_LOAD data from the IGVM to **two** locations in guest physical memory:

1. Normal boot addresses (as today — becomes the running kernel)
2. A **reserved GPA range** (clean spare copy for kexec)

No changes to the IGVM file contents. No duplication in the IGVM. The host loads the same kernel data to an additional address range that is marked as e820-reserved. The host increases VTL2's total physical memory budget by ~14 MB to accommodate this.

The guest is told (via device tree, IGVM parameter, or boot_params) where the reserved copy lives. A stub kernel reads from that known address at kexec time.

### Why a separate copy at a reserved address is needed

The boot copy (the running kernel) can't be reused because:
- `.data` and `.bss` are modified at runtime (global variables, init state)
- `.text` is modified by alternatives patching during boot
- The pages are actively mapped by the running kernel's page tables
- kexec overwrites physical memory — you can't read from pages you're writing to

### How it works

- **Build time**: No IGVM file changes needed (same PT_LOAD data as today)
- **Boot**: Host loads PT_LOAD pages to boot address (as today) AND to reserved GPA range. Host increases VTL2's physical memory budget by ~14 MB.
- **Guest boot**: Kernel sees reserved region in e820 as reserved — never allocates from it. The kernel's usable memory pool is unaffected.
- **Kexec prep**: Stage only the stub (~14 KB) via `kexec_file_load` — no bzImage/vmlinux fd needed
- **Kexec exec**: Stub runs → identity maps reserved region → copies PT_LOAD segments to their target boot addresses → jumps to entry point

### Memory used

| What | Size | Memory type | Who pays | Lifetime |
|------|------|-------------|----------|----------|
| Running VTL2 kernel | ~14 MB | Host-provisioned GPA | Host DRAM | Always |
| Reserved vmlinux copy | ~14 MB | **Host-provisioned reserved GPA** | **Host DRAM (extra budget)** | Permanent |
| Stub binary in rootfs | ~14 KB | Guest kernel-allocatable (tmpfs) | Guest kernel pool | Permanent (negligible) |
| `kexec_file_load` staging (stub only) | ~14 KB | Guest kernel-allocatable (kmalloc) | Guest kernel pool | Transient |

### Cost summary

- **Extra host DRAM provisioned**: +14 MB (host gives VTL2 more physical memory)
- **Guest kernel-allocatable RAM lost permanently**: 0
- **Peak transient from guest kernel pool**: ~14 KB (negligible)

### What "host pays" means

The host increases VTL2's total physical memory budget by 14 MB. This costs 14 MB of host DRAM dedicated to backing the reserved GPA range. However, the guest kernel's `MemAvailable` is **completely unaffected** — it has exactly the same amount of usable RAM as without kexec support.

The guest kernel never sees those 14 MB. They're e820-reserved, excluded from the memory allocator, and invisible to cgroups/OOM accounting. From the kernel's perspective, kexec infrastructure doesn't exist.

---

## Side-by-Side Comparison

| Metric | 1. Current (host-driven) | 2a. bzImage in rootfs | 2b. Reserved GPA + stub |
|--------|--------------------------|----------------------|-------------------------|
| **Extra host DRAM provisioned** | 0 | 0 | +14 MB |
| **Guest kernel MemAvailable lost** | 0 | -3.5 MB | 0 |
| **Peak transient (guest kernel pool)** | 0 | +14-64 MB | ~0 |
| **IGVM file size change** | 0 | 0 | 0 |
| **Host involvement at servicing time** | Yes (full reload) | No | No |
| **Servicing blackout time** | Seconds | ~ms | ~ms |
| **OOM can break kexec** | N/A | Yes (tmpfs in kernel pool) | No (reserved = untouchable) |
| **Reliability under memory pressure** | N/A | Moderate risk | High |
| **Implementation complexity** | Existing | Low | High |
| **Host changes required** | None | None | Load to 2nd address, increase budget, communicate GPA |
| **Guest changes required** | None | kexec_prepare userspace | Stub kernel + GPA discovery |
| **New kernel version at servicing** | Yes (host loads new IGVM) | Only if rootfs updated | Only if host deposits new copy |

---

## The Core Tradeoff

**2a**: Uses 3.5 MB from the guest kernel's own memory pool. No host changes. Simple. The kernel has slightly less RAM for work.

**2b**: Uses 14 MB of host-provisioned memory that the guest kernel never sees. The guest kernel operates with its full original memory budget. But requires host protocol changes and a stub.

The fundamental question: **which resource is more constrained — 3.5 MB of guest kernel pool, or 14 MB of host DRAM?**

- If VTL2 kernel memory is tight (small VMs, many competing allocations): 2b is better — zero impact on guest kernel working set
- If host DRAM budget per VM is tight: 2a is better — no extra provisioning
- If simplicity and time-to-ship matter: 2a wins

---

## Open Questions

1. **How does the guest discover the reserved region GPA?** — Options: device tree property, IGVM parameter page, hardcoded well-known address, boot_params extension.

2. **Initrd in approach 2b** — If the new kernel needs an initrd:
   - Host also deposits initrd in reserved region (more host DRAM)
   - Built by userspace into memfd, passed to stub (back to using guest kernel pool for transient)
   - No initrd — built-in modules only

3. **Versioning** — If servicing needs a DIFFERENT kernel version, the reserved copy is stale:
   - Host updates reserved region before signaling servicing (requires host involvement — loses the no-host benefit)
   - Accept that kexec reloads same version (sufficient for VTL2 restart without version change)

4. **Is 3.5 MB from guest kernel pool actually a problem?** — VTL2 typically has 256+ MB. 3.5 MB is ~1.4%. The peak transient (14-64 MB during prep) is the bigger concern, but it's freed immediately after staging.

5. **Can the host share pages without double-backing?** — If the hypervisor supports shared/CoW mappings, the reserved region could alias the same host physical pages (since the reserved copy is never written). This would make 2b cost ~0 extra host DRAM.
