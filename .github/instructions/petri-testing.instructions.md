---
description: "Use when writing, modifying, or debugging VMM integration tests using the petri framework, PetriVmBuilder, PipetteClient, openvmm_test/hyperv_test macros, or test artifacts."
applyTo: ["vmm_tests/**", "petri/**"]
---

# Petri Integration Test Framework

## Test Structure

Integration tests live in `vmm_tests/vmm_tests/tests/tests/` organized by architecture and feature:
- `x86_64.rs`, `aarch64.rs` — Architecture-specific tests
- `multiarch/` — Tests that run on both architectures
- Subfolders by feature: `tpm.rs`, `storage.rs`, etc.

## Test Macros

```rust
#[openvmm_test(x86_64, uefi)]           // OpenVMM backend, x86_64, UEFI guest
#[openvmm_test(aarch64, openhcl_uefi)]  // OpenVMM backend, aarch64, OpenHCL+UEFI
#[hyperv_test(x86_64, uefi)]            // Hyper-V backend
```

Parameters: `(arch, guest_type)` where:
- **arch**: `x86_64`, `aarch64`
- **guest_type**: `uefi`, `pcat`, `openhcl_uefi`, `openhcl_linux_direct`, `tmk`

## VM Builder API

```rust
async fn my_test(config: PetriVmBuilder<OpenVmmPetriBackend>) -> anyhow::Result<()> {
    // Configure before run
    let config = config.with_processor_topology(ProcessorTopology::new(4));

    // Launch VM + guest agent
    let (mut vm, agent) = config.run().await?;

    // Or launch without agent
    let mut vm = config.run_without_agent().await?;

    Ok(())
}
```

## Guest Agent (Pipette)

`PipetteClient` runs inside the guest for command execution and file I/O:

```rust
agent.ping().await?;
agent.write_file("/tmp/test.txt", data).await?;
let output = agent.unix_shell().run("uname -a").await?;
```

## VM Operations

```rust
vm.wait_for_vtl2_ready().await?;
let vtl2_agent = vm.wait_for_vtl2_agent().await?;
vm.reset().await?;
let value = vm.inspect_openhcl("some/path", None).await?;
```

## Artifacts

Test artifacts (guest images, binaries) declared in `vmm_tests/petri_artifacts_vmm_test/src/lib.rs`:
- `OPENVMM_NATIVE`, `OPENHCL_IGVM_UEFI_X64`, `GUEST_TEST_UEFI_X64`, etc.
- Use the existing artifact pattern when adding new test dependencies
