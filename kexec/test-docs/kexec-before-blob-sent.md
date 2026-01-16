Yes — with the current “pre_send” hook, the kexec happens before send_servicing_state(), so the GET/GED “More data?” logs won’t appear and the blob isn’t sent. saved_state_len only means the buffer was prepared, not transferred. If you want the write to complete, the kexec must move after send_servicing_state() (or after a host ACK).
//////////////////////////////
Yes — when the hook was after send_servicing_state(), the host reload typically preempted the process, so the kexec never ran. It wasn’t a kexec failure; the process got replaced before the hook executed.
/////////////////////////////////////
Yes—but only if the host side cooperates. You need a host “hold reload” option or ACK handshake so VTL2 can send the blob, get an explicit “do not reload yet” response, then kexec. Without that, the host reload can still preempt you.

Two viable patterns:

Add a host option that defers reload until VTL2 signals “ready to reload” (new RPC/flag).
Keep the hook after send_servicing_state() and have the host wait for an explicit guest signal before reload.
If you want, I can wire option (1) end-to-end.

////////////////////////////////////////////////////////////////////
Yes. Today, once VTL2 finishes sending the blob, the host is free to trigger the reload immediately, which can preempt any guest-side kexec that happens afterward. The “ACK” is essentially just the completion of the send_servicing_state() transfer (GET/GED), not a separate “hold reload” handshake. There’s no explicit host promise to wait for a guest signal before reloading.

Correct. There’s no explicit ACK/hold today. If you want kexec after the blob is sent, we need to add a host “hold reload” option plus a guest “ready” signal so the host waits until VTL2 tells it to proceed. If you want, I can implement that flow.