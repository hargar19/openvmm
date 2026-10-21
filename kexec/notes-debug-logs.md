Phase	What it covers	Expected if purgatory is the cause
kexec_prep	Flush → reboot request: saving state to persisted memory and sidecar prep	~1–2 ms
kexec_jump	Reboot request → bootstrap entry: old-kernel shutdown, relocation, purgatory SHA-256	~90–110 ms
bootstrap	Bootstrap entry → jump to Linux, including starting the fresh sidecar	~3 ms
kernel_init	Jump to Linux → worker entry; directly comparable to host servicing's kernel_init	~175–190 ms
