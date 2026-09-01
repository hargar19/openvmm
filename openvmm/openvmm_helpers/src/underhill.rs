// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Helpers for managing the Underhill firmware.

use anyhow::Context;
use get_resources::ged::GuestEmulationRequest;
use get_resources::ged::GuestServicingFlags;
use mesh::rpc::RpcSend;
use openvmm_defs::rpc::VmRpc;

/// Save the running state of Underhill and stage the new version.
pub async fn save_underhill(
    vm_send: &mesh::Sender<VmRpc>,
    send: &mesh::Sender<GuestEmulationRequest>,
    flags: GuestServicingFlags,
    file: std::fs::File,
) -> anyhow::Result<()> {
    // Stage the IGVM file in the VM worker.
    tracing::debug!("staging new IGVM file");
    vm_send
        .call_failable(VmRpc::StartReloadIgvm, file)
        .await
        .context("failed to stage new IGVM file")?;

    // Block waiting for the guest to send saved state.
    //
    // TODO: make this event driven instead so that other operations are not
    // blocked while waiting for the guest.
    tracing::debug!("waiting for guest to send saved state");
    let r = send
        .call_failable(GuestEmulationRequest::SaveGuestVtl2State, flags)
        .await
        .context("failed to save VTL2 state");

    if r.is_err() {
        // Clear the staged IGVM file.
        tracing::debug!(?r, "save state failed, clearing staged IGVM file");
        let _ = vm_send.call(VmRpc::CompleteReloadIgvm, false).await;
    }

    r
}

/// Restore Underhill from a previously saved state. This should always be called after save_underhill.
pub async fn restore_underhill(
    vm_send: &mesh::Sender<VmRpc>,
    send: &mesh::Sender<GuestEmulationRequest>,
) -> anyhow::Result<()> {
    // Reload the IGVM file and reset VTL2 state.
    tracing::debug!("reloading IGVM file");
    vm_send
        .call_failable(VmRpc::CompleteReloadIgvm, true)
        .await
        .context("failed to reload VTL2 firmware")?;

    // Wait for VTL0 to start.
    //
    // TODO: event driven, cancellable.
    tracing::debug!("waiting for VTL0 to start");
    send.call_failable(GuestEmulationRequest::WaitForVtl0Start, ())
        .await
        .context("vtl0 start failed")?;

    Ok(())
}

/// Kexec-based servicing: trigger the guest to save state and kexec internally.
///
/// Unlike normal servicing, no IGVM staging or host-driven reload is performed.
/// The guest saves state to a persisted memory region, then does `kexec -e` to
/// boot the new kernel. The new VTL2 instance reads the persisted state directly
/// from memory — no state flows through the host.
pub async fn kexec_service_underhill(
    send: &mesh::Sender<GuestEmulationRequest>,
    flags: GuestServicingFlags,
) -> anyhow::Result<()> {
    send.call(GuestEmulationRequest::ClearVtl0Start, ()).await?;

    // Fire-and-forget: send a save notification to trigger the guest-side
    // servicing + kexec flow. The guest will serialize state, persist it to
    // the reserved memory region, and exec into the new kernel. It will never
    // send state back to the host, so we use Rpc::detached() to avoid waiting.
    tracing::info!("kexec: sending save notification to trigger guest-side kexec");
    send.send(GuestEmulationRequest::SaveGuestVtl2State(
        mesh::rpc::Rpc::detached(flags),
    ));

    // Wait for the new VTL2 instance (booted via kexec) to report VTL0 started.
    tracing::info!("kexec: waiting for new VTL2 to report VTL0 started");
    send.call_failable(GuestEmulationRequest::WaitForVtl0Start, ())
        .await
        .context("kexec: new VTL2 failed to start VTL0")?;

    tracing::info!("kexec: servicing complete — new VTL2 is running");
    Ok(())
}
