`#![no_std]` tells the Rust compiler: **don't link the standard library (`std`)**.

Normally Rust programs get `std`, which provides heap allocation (`Vec`, `String`, `Box`), file I/O, threads, networking, etc. — all things that require an operating system underneath.

The kexec stub runs in bare-metal mode — after kexec, the old kernel is gone and the new kernel hasn't started yet. There's no OS, no syscalls, no memory allocator, no file system. `std` would fail to link because there's nothing implementing its OS abstractions.

With `#![no_std]`, the stub only gets `core` — the subset of Rust that needs zero OS support:
- Primitive types, slices, arrays
- `core::ptr` (raw pointer operations — what the stub uses to copy ELF segments)
- `core::slice::from_raw_parts` (to view physical memory as byte slices)
- `core::mem::transmute` (to cast the ELF entry point to a function pointer)
- `core::fmt` (for the panic handler's serial output)
- `core::array::from_fn` (to initialize the segment array)
- `assert!`, `panic!`

No heap, no I/O, no threads — just computation on memory the stub can directly address via identity-mapped physical addresses.

This is the same constraint as `openhcl_boot` (the boot shim) and `sidecar` (the AP kernel), which is why they all depend on `minimal_rt` — it provides the bare minimum runtime: stack setup, panic handler wiring, and serial port access.


Read [](file:///home/hargar/underhill/openvmm/openhcl/kexec_stub/Cargo.toml)

The Cargo.toml has two dependencies:

**`minimal_rt`** (runtime dependency) — Provides the bare-metal runtime primitives the stub needs at execution time:
- `minimal_rt::arch::Serial` — serial port I/O for the panic handler (writes to `0x3F8`)
- `minimal_rt::arch::InstrIoAccess` — raw x86 `in`/`out` port instructions
- `minimal_rt::arch::fault()` — halts the CPU on panic
- The `#[cfg(minimal_rt)]` conditional — the crate sets this cfg flag when building for `x86_64-unknown-none`, which gates `#![no_std]`/`#![no_main]` and the assembly entry point. Without it (e.g. `cargo check` on the host), the crate compiles as a normal binary with an empty `main()` for IDE support.

**`minimal_rt_build`** (build dependency) — Used in `build.rs` to:
- Call `minimal_rt_build::init()` which detects the target triple and sets the `minimal_rt` cfg flag
- Emit the custom linker script (`link.x`) via `cargo:rustc-link-arg`

Both crates are workspace dependencies (`.workspace = true`), meaning their paths/versions are defined in the root Cargo.toml. They're the same crates used by `openhcl_boot` and `sidecar` — the other bare-metal components in OpenHCL. No external crates are pulled in; the stub stays entirely within the OpenHCL ecosystem.

The root Cargo.toml defines them as workspace dependencies:

```toml
minimal_rt = { path = "openhcl/minimal_rt" }
minimal_rt_build = { path = "openhcl/minimal_rt_build" }
```

When the kexec_stub's Cargo.toml says `minimal_rt.workspace = true`, it means "use the path defined in the workspace root." The actual crates live at:
- minimal_rt — the runtime library
- minimal_rt_build — the build-time helper

These already existed before the kexec stub — they're shared by `openhcl_boot`, `sidecar`, and opentmk. The kexec stub didn't need to add them to the workspace dependency table; it just referenced them.

## build.rs

Read [](file:///home/hargar/underhill/openvmm/openhcl/kexec_stub/build.rs)

The build script does two things:

1. **`minimal_rt_build::init()`** — Detects whether the target is a bare-metal platform (`x86_64-unknown-none`). Returns `true` only for the actual target build, `false` for host builds or `cargo check`. This prevents the custom linker script from being applied when just checking code on the host.

2. **Custom linker script** — When building for real, it tells rustc to use `link.x` via `-T`. This is critical because `link.x` places `_start` at offset 0 of the flat binary, orders sections properly, and keeps `.rela.dyn` in a loadable segment.

The comment block explains the **PIE decision**: the build deliberately does *not* disable PIE (no `-C relocation-model=static`). This is intentional — PIE mode causes the compiler to emit `R_X86_64_RELATIVE` relocations for GOT entries and vtable pointers. The stub's `entry.S` walks these relocations at runtime to fix up addresses after being loaded at whatever physical address kexec chose. Without PIE, absolute addresses would be baked in at link time and wrong at runtime.

The `cargo:rustc-check-cfg=cfg(nightly)` line registers a custom cfg so `#[cfg(nightly)]` doesn't trigger an "unexpected cfg" warning — this is a standard pattern in the OpenVMM codebase.