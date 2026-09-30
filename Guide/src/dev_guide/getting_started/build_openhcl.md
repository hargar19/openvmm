# Building OpenHCL

This page explains how to build and customize OpenHCL IGVM firmware images.

**Prerequisites:**

- [Getting started on Linux / WSL2](./linux.md).

Reminder: OpenHCL cannot currently be built on Windows hosts!

* * *

An OpenHCL IGVM firmware image is composed of several distinct binaries and
artifacts. For example: the `openvmm_hcl` usermode binary, the OpenHCL boot
shim, the OpenHCL Linux kernel and initrd, etc....

Some of these components are built directly out of the OpenVMM repo, whereas
others must be downloaded as pre-built artifacts from other associated repos.
Various tools and scripts will then transform, package, and re-package these
artifacts into a final OpenHCL IGVM firmware binary.

Fortunately, we don't expect you do to all those steps manually!

All the complexity of installing the correct system dependencies, building the
right binaries, downloading the right artifacts, etc... is neatly encapsulated
behind a single `cargo xflowey build-igvm` command, which orchestrates the
entire end-to-end OpenHCL build process.

Using the `build-igvm` flow is as simple as running:

```bash
cargo xflowey build-igvm [RECIPE]
```

The first build will take some time as all the dependencies are
installed/downloaded/built.

**Note: At this time, OpenHCL can only be built on Linux (or WSL2)!**

A "recipe" corresponds to one of the pre-defined IGVM SKUs that are actively
supported and tested in OpenVMM's build infrastructure.

A single recipe encodes _all_ the details of what goes into an individual IGVM
file, such as what build flags `openvmm_hcl` should be built with, what goes
into a VTL2 initrd, what `igvmfilegen` manifest is being used, etc...

- e.g: `x64`, for a "standard" x64 IGVM
- e.g: `aarch64`, for a "standard" aarch64 IGVM
- e.g: `x64-cvm`, for a x64 CVM IGVM
- e.g: `x64-test-linux-direct`, for x64 IGVM booting a test linux direct image
- _for a full list of available recipes, please run `cargo xflowey build-igvm --help`_

New recipes can be added by modifying the `build-igvm` source code.

Build output is then binplaced to: `flowey-out/artifacts/build-igvm/{release-mode}/{recipe}/openhcl-{recipe}.bin`

So, for example:

```bash
cargo xflowey build-igvm x64-cvm
# output: flowey-out/artifacts/build-igvm/debug/x64-cvm/openhcl-x64-cvm.bin

cargo xflowey build-igvm x64 --release
# output: flowey-out/artifacts/build-igvm/release/x64/openhcl-x64.bin
```

```admonish warning
`cargo xflowey build-igvm` is designed to be used as part of the
developer inner-loop, and does _NOT_ have a stable CLI suitable for CI or any
other form of production automation!

In-tree pipelines and automation should interface with the underlying `flowey`
infrastructure that powers `cargo xflowey build-igvm`, _without_ relying on
the details of its CLI.
```

## Building ohcldiag-dev

`ohcldiag-dev` is typically built as a Windows binary.

This can be done directly from Windows, or using
[cross-compilation from WSL2](../getting_started/cross_compile.md).

The command to build `ohcldiag-dev` is simply:

```sh
# you may need to run `rustup target add x86_64-pc-windows-msvc` first
cargo build -p ohcldiag-dev --target x86_64-pc-windows-msvc
```

**Note:** Thanks to x86 emulation built into Windows, `ohcldiag-dev.exe` that is
built for x64 Windows will work on Aarch64 Windows as well.

## Troubleshooting

This section documents some common errors you may encounter while building
OpenHCL.

If you are still running into issues, consider filing an issue on the OpenVMM
GitHub Issue tracker.

### Help! The build failed due to a missing dependency

If you don't mind having `xflowey` install some dependencies globally
on your machine (i.e: via `apt install`, or `rustup toolchain add`),
you can pass `--install-missing-deps` to your invocation of
`build-igvm`:

```bash
cargo xflowey build-igvm x64 --install-missing-deps
```

This will automatically install all required dependencies, including
the .NET SDK, Rust toolchains, Node.js, and any necessary system
packages.

Alternatively - `build-igvm` _should_ emit useful human-readable
error messages when it encounters a dependency that isn't installed,
with a suggestion on how to install it.

If it doesn't - please file an Issue!

### Help! Everything is rebuilding even though I only made a small change

Cargo's target triple handling can be a bit buggy. Try running with:

```bash
CARGO_BUILD_TARGET=x86_64-unknown-linux-gnu cargo build-igvm [RECIPE]
```

or adding the below to your .bashrc:

```bash
export CARGO_BUILD_TARGET=x86_64-unknown-linux-gnu
```

## Build Customization

Aside from building IGVM files corresponding the the built-in IGVM recipes,
`build-igvm` also offers a plethora of customization options for developers who
wish to build specialized custom IGVM files for local testing.

Some examples of potentially useful customization include:

- `--override-manifest`: Override the recipe's `igvmfilegen` manifest file
    via, in order to tweak different kernel command line options, different VTL0
    boot configuration, or different VTL2 memory sizes.

- `--custom-openvmm-hcl`: Specify a pre-built `openvmm_hcl` binary. This is
    useful in case you have already built it with some custom settings, e.g.:

    ```bash
    cargo build --target x86_64-unknown-linux-musl -p openvmm_hcl --features myfeature
    cargo xflowey build-igvm x64 --custom-openvmm-hcl target/x86_64-unknown-linux-musl/debug/openvmm_hcl
    ```

- Specify a custom VTL2 kernel `vmlinux` / `Image`, instead of using the
    packaged main kernel.

    ```bash
    cargo xflowey build-igvm x64 --custom-kernel path/to/my/prebuilt/vmlinux
    ```

    The packaged dev kernel variants are disabled by default. A custom kernel
    remains available for local kernel development without enabling those
    variants.

For a full list of available customizations, refer to `build-igvm --help`.

### Experimental IGVM kexec payloads

You can build an IGVM carrying a custom kernel and its matching initrd for
in-place OpenHCL servicing. See the
[IGVM servicing contract](../../reference/architecture/openhcl/igvm.md#experimental-kexec-servicing)
for the payload layout and delivery protocol.

```admonish warning
This experimental path supports only x64, non-isolated VMs and requires a
compatible custom Hyper-V delivery implementation installed on the host.
Both the bootstrap image and every successor image must be built from the
same compatible implementation of the payload descriptor and userspace
restore contract. Unmodified Mayank images are not supported.

This repository does not provide the compatible host delivery source or a
host trigger CLI. Use the procedure supplied with your compatible host
implementation; building an IGVM alone does not initiate servicing.
```

Use a raw, uncompressed x64 ELF `vmlinux` and the modules tree from that same
kernel build. Pass the **same file** as both `--custom-kernel` and
`--custom-binary`; a compressed kernel or `bzImage` is not a substitute:

```bash
cargo xflowey build-igvm x64 \
     --custom-kernel path/to/vmlinux \
     --custom-kernel-modules path/to/modules \
     --custom-binary path/to/vmlinux
```

The modules directory must contain the installed `kernel/drivers/...` tree,
not just the kernel build directory's `drivers/...` files. The custom kernel
resolver also expects `kernel_build_metadata.json` beside `vmlinux`.

Add `--with-sidecar` to include the sidecar. Without `--release`, the output
for this customized recipe is
`flowey-out/artifacts/build-igvm/debug/x64-custom/openhcl-x64-custom.bin`.
Use `--build-label x64-kexec-a` to select a different output label and
preserve each build's output before building the next image.

The custom binary is included as normal measured IGVM `PageData`, alongside
the existing initrd. Servicing extracts the kernel and initrd from the
incoming image; it neither reads a kernel from the running root filesystem
nor rebuilds a CPIO archive.

#### Validate the artifacts and transition

Before host testing, run the focused parser and kexec tests:

```bash
cargo nextest run --profile agent -p underhill_core -E 'test(kexec)'
```

To run the production extractor against your generated artifact without
invoking kexec, run the opt-in artifact test:

```bash
OPENHCL_TEST_SERVICING_IGVM=path/to/openhcl.bin \
OPENHCL_TEST_VMLINUX=path/to/vmlinux \
OPENHCL_TEST_INITRD=path/to/openhcl.cpio.gz \
cargo nextest run --profile agent -p underhill_core --run-ignored only \
    -E 'test(servicing_igvm_artifact_matches_inputs)'
```

This test checks exact extracted bytes, but does not validate the host
delivery path or execute the transition. For end-to-end validation:

1. Inspect each built IGVM's `KexecPayloadDescriptor` and extract its kernel
    and initrd ranges using their exact byte lengths, excluding page padding.
    Compare the extracted kernel byte-for-byte with the input `vmlinux`, and
    the extracted initrd with the initrd supplied to that image's IGVM build.
    Inspect the initrd's file listing and confirm that it contains no
    `boot/vmlinux` (the guest path `/boot/vmlinux`). Keep these checks with
    the corresponding image so that later builds cannot overwrite the inputs.
2. Establish a same-build baseline: boot image A, then deliver that same
    image through the compatible host implementation. Confirm ordinary
    OpenHCL start completion and that VTL0 resumes and remains responsive,
    without a guest reboot.
3. Build image B with distinguishable kernel and OpenHCL userspace build
    identities, retaining the matching restore contract. Boot A and deliver
    B. Verify the running VTL2 kernel release and userspace build identity
    match B, rather than relying only on a successful notification or the
    image's filename. Verify VTL0 remains alive and its workload continues.
4. Repeat servicing cycles, checking completion and VTL0 liveness each time.
    Exercise sidecar-enabled images and active VTL0 network and storage I/O;
    check that connections and storage operations recover and continue
    without data corruption.

### Advanced

Depending on what you're doing, you may need to build the individual components
that go into an OpenHCL IGVM build.

Our `flowey`-based pipelines handle the complexities of properly invoking and
orchestrating the various individual build tools / scripts used to construct
IGVM files, but a sufficiently motivated user can go through these steps
manually.

Please consult the source code for `cargo xflowey build-igvm` for a breakdown of
all build steps and available customization options.

Note that the canonical "source of truth" for how to build end-to-end OpenHCL
IGVM files are these build scripts themselves, and the specific flow is subject
to change over time!
