PS C:\Underhill> .\uhdiag-dev.exe $vmName kmsg -f
[0.000000] Linux version 6.18.37-microsoft-hcl+ (hargar@UbuntuVM-22) (gcc (Ubuntu 11.4.0-1ubuntu1~22.04.3) 11.4.0, GNU ld (GNU Binutils for Ubuntu) 2.38) #1 SMP PREEMPT_DYNAMIC Mon Oct  5 21:42:47 UTC 2026
[0.000000] Command line: loglevel=8 log_buf_len=128K printk.time=1 console_msg_format=syslog uio_hv_generic.no_mask=1 coredump_filter=0x33 cpufreq.off=1 cpuidle.off=1 cryptomgr.notests idle=halt initcall_blacklist=init_real_mode,sbf_init lpj=3000000 no_timer_check noxsave oops=panic panic_on_warn=0 panic_print=0 panic=-1 printk.devkmsg=on reboot=t rootfstype=tmpfs sysctl.vm.compaction_proactiveness=0 tsc=reliable unknown_nmi_panic=1 vfio_pci.ids=1414:00ba vfio.enable_unsafe_noiommu_mode=1 rdinit=/underhill-init OPENHCL_NVME_VFIO=1 hv_storvsc.storvsc_vcpus_per_sub_channel=2048 hv_storvsc.storvsc_max_hw_queues=2 hv_storvsc.storvsc_ringbuffer_size=0x8000 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off iommu=off pci=off swiotlb=1,1 console=ttynull OPENHCL_NVME_KEEP_ALIVE=host,privatepool boot_cpus=0,32 hv_vmbus.message_connection_id=0x800074 OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info RUST_BACKTRACE=full
[0.000000] KERNEL supported cpus:
[0.000000] Intel GenuineIntel
[0.000000] AMD AuthenticAMD
[0.000000] BIOS-provided physical RAM map:
[0.000000] BIOS-e820: [mem 0x0000000408000000-0x0000000408013fff] reserved
[0.000000] BIOS-e820: [mem 0x0000000408014000-0x0000000408dfffff] usable
[0.000000] BIOS-e820: [mem 0x0000000408e00000-0x0000000408e19fff] reserved
[0.000000] BIOS-e820: [mem 0x0000000408e1a000-0x000000040972cfff] usable
[0.000000] BIOS-e820: [mem 0x000000040972d000-0x0000000409758fff] reserved
[0.000000] BIOS-e820: [mem 0x0000000409759000-0x0000000409798fff] usable
[0.000000] BIOS-e820: [mem 0x0000000409799000-0x000000040989cfff] reserved
[0.000000] BIOS-e820: [mem 0x000000040989d000-0x00000004098b3fff] usable
[0.000000] BIOS-e820: [mem 0x00000004098b4000-0x0000000409bf5fff] reserved
[0.000000] BIOS-e820: [mem 0x0000000409bf6000-0x0000000417ffffff] usable
[0.000000] BIOS-e820: [mem 0x0000000418000000-0x00000004193fffff] reserved
[0.000000] software IO TLB: area num 1.
[0.000000] NX (Execute Disable) protection: active
[0.000000] APIC: Static calls initialized
[0.000000] Hypervisor detected: Microsoft Hyper-V
[0.000000] Hyper-V: privilege flags low 0xae7f, high 0x3b8030, ext 0x62, hints 0x9e0e24, misc 0xe0bed7b2
[0.000000] Hyper-V: Nested features: 0x0
[0.000000] Hyper-V: LAPIC Timer Frequency: 0x1e8480
[0.000000] Hyper-V: Using hypercall for remote TLB flush
[0.000000] clocksource: hyperv_clocksource_tsc_page: mask: 0xffffffffffffffff max_cycles: 0x24e6a1710, max_idle_ns: 440795202120 ns
[0.000000] clocksource: hyperv_clocksource_msr: mask: 0xffffffffffffffff max_cycles: 0x24e6a1710, max_idle_ns: 440795202120 ns
[0.000000] Linux runs in Hyper-V Virtual Trust Level 0
[0.000000] tsc: Detected 2000.001 MHz processor
[0.000002] e820: update [mem 0x00000000-0x00000fff] usable ==> reserved
[0.000005] e820: remove [mem 0x000a0000-0x000fffff] usable
[0.000008] last_pfn = 0x418000 max_arch_pfn = 0x1000000000
[0.000009] x86/PAT: PAT support disabled because CONFIG_X86_PAT is disabled in the kernel.
[0.000011] x86/PAT: Configuration [0-7]: WB  WT  UC- UC  WB  WT  UC- UC
[0.000013] last_pfn = 0x0 max_arch_pfn = 0x1000000000
[0.000016] Using GB pages for direct mapping
[0.000355] printk: log buffer data + meta data: 131072 + 458752 = 589824 bytes
[0.000356] printk: early log buf free: 13080(79%)
[0.000357] RAMDISK: [mem 0x408f06000-0x409729fff]
[0.000416] OF: reserved mem: Reserved memory: No reserved-memory node in the DT
[0.000559] NODE_DATA(0) allocated [mem 0x417f53900-0x417f5817f]
[0.000579] Zone ranges:
[0.000580] DMA      empty
[0.000581] DMA32    empty
[0.000581] Normal   [mem 0x0000000408014000-0x0000000417ffffff]
[0.000582] Device   empty
[0.000583] Movable zone start for each node
[0.000583] Early memory node ranges
[0.000583] node   0: [mem 0x0000000408014000-0x0000000408dfffff]
[0.000584] node   0: [mem 0x0000000408e1a000-0x000000040972cfff]
[0.000585] node   0: [mem 0x0000000409759000-0x0000000409798fff]
[0.000585] node   0: [mem 0x000000040989d000-0x00000004098b3fff]
[0.000586] node   0: [mem 0x0000000409bf6000-0x0000000417ffffff]
[0.000587] Initmem setup node 0 [mem 0x0000000408014000-0x0000000417ffffff]
[0.000609] On node 0, zone Normal: 20 pages in unavailable ranges
[0.000622] On node 0, zone Normal: 26 pages in unavailable ranges
[0.000623] On node 0, zone Normal: 44 pages in unavailable ranges
[0.000625] On node 0, zone Normal: 260 pages in unavailable ranges
[0.000957] On node 0, zone Normal: 834 pages in unavailable ranges
[0.000986] IMCR and PIC compatibility mode.
[0.001008] TSC deadline timer available
[0.001011] CPU topo: Max. logical packages:   1
[0.001011] CPU topo: Max. logical dies:       1
[0.001012] CPU topo: Max. dies per package:   1
[0.001014] CPU topo: Max. threads per core:   2
[0.001015] CPU topo: Num. cores per package:    32
[0.001016] CPU topo: Num. threads per package:  64
[0.001016] CPU topo: Allowing 64 present CPUs plus 0 hotplug CPUs
[0.001021] Cannot find an available gap in the 32-bit address range
[0.001021] PCI devices with unassigned 32-bit BARs may not work!
[0.001022] [mem 0x418100000-0x4184fffff] available for PCI devices
[0.001022] Booting paravirtualized kernel on Hyper-V
[0.001026] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
[0.001031] setup_percpu: NR_CPUS:2048 nr_cpumask_bits:64 nr_cpu_ids:64 nr_node_ids:1
[0.003216] percpu: Embedded 41 pages/cpu s127296 r8192 d32448 u262144
[0.003220] pcpu-alloc: s127296 r8192 d32448 u262144 alloc=1*2097152
[0.003222] pcpu-alloc: [0] 00 01 02 03 04 05 06 07 [0] 08 09 10 11 12 13 14 15
[0.003227] pcpu-alloc: [0] 16 17 18 19 20 21 22 23 [0] 24 25 26 27 28 29 30 31
[0.003231] pcpu-alloc: [0] 32 33 34 35 36 37 38 39 [0] 40 41 42 43 44 45 46 47
[0.003235] pcpu-alloc: [0] 48 49 50 51 52 53 54 55 [0] 56 57 58 59 60 61 62 63
[0.003251] Hyper-V: PV spinlocks enabled
[0.003253] PV qspinlock hash table entries: 256 (order: 0, 4096 bytes, linear)
[0.003257] Kernel command line: loglevel=8 log_buf_len=128K printk.time=1 console_msg_format=syslog uio_hv_generic.no_mask=1 coredump_filter=0x33 cpufreq.off=1 cpuidle.off=1 cryptomgr.notests idle=halt initcall_blacklist=init_real_mode,sbf_init lpj=3000000 no_timer_check noxsave oops=panic panic_on_warn=0 panic_print=0 panic=-1 printk.devkmsg=on reboot=t rootfstype=tmpfs sysctl.vm.compaction_proactiveness=0 tsc=reliable unknown_nmi_panic=1 vfio_pci.ids=1414:00ba vfio.enable_unsafe_noiommu_mode=1 rdinit=/underhill-init OPENHCL_NVME_VFIO=1 hv_storvsc.storvsc_vcpus_per_sub_channel=2048 hv_storvsc.storvsc_max_hw_queues=2 hv_storvsc.storvsc_ringbuffer_size=0x8000 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off iommu=off pci=off swiotlb=1,1 console=ttynull OPENHCL_NVME_KEEP_ALIVE=host,privatepool boot_cpus=0,32 hv_vmbus.message_connection_id=0x800074 OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info RUST_BACKTRACE=full
[0.003302] blacklisting initcall init_real_mode
[0.003304] blacklisting initcall sbf_init
[0.003320] Kernel: The 'panic_print' parameter is now deprecated. Please use 'panic_sys_info' and 'panic_console_replay' instead.
[0.003408] Unknown kernel command line parameters "OPENHCL_NVME_VFIO=1 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off OPENHCL_NVME_KEEP_ALIVE=host,privatepool OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info RUST_BACKTRACE=full", will be passed to user space.
[0.003418] random: crng init done
[0.003447] Dentry cache hash table entries: 32768 (order: 6, 262144 bytes, linear)
[0.003460] Inode-cache hash table entries: 16384 (order: 5, 131072 bytes, linear)
[0.004292] Fallback order for Node 0: 0
[0.004296] Built 1 zonelists, mobility grouping on.  Total pages: 64352
[0.004297] Policy zone: Normal
[0.004298] mem auto-init: stack:off, heap alloc:off, heap free:off
[0.004939] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=64, Nodes=1
[0.005262] Dynamic Preempt: none
[0.005494] rcu: Preemptible hierarchical RCU implementation.
[0.005494] rcu: <09>RCU restricting CPUs from NR_CPUS=2048 to nr_cpu_ids=64.
[0.005496] rcu: RCU calculated value of scheduler-enlistment delay is 10 jiffies.
[0.005496] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=64
[0.005557] Using NULL legacy PIC
[0.005557] NR_IRQS: 131328, nr_irqs: 1040, preallocated irqs: 0
[0.005618] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[0.005621] Persistent clock returned invalid value
[0.005754] APIC: Switch to symmetric I/O mode setup
[0.005760] x2apic enabled
[0.005785] APIC: Switched APIC routing to: physical x2apic
[0.005788] Hyper-V: Host Build 10.0.29627.1000-1-0
[0.005790] Hyper-V: enabling crash_kexec_post_notifiers
[0.005893] Hyper-V: Using IPI hypercalls
[0.005894] APIC: send_IPI() replaced with hv_send_ipi()
[0.005898] APIC: send_IPI_mask() replaced with hv_send_ipi_mask()
[0.005900] APIC: send_IPI_mask_allbutself() replaced with hv_send_ipi_mask_allbutself()
[0.005902] APIC: send_IPI_allbutself() replaced with hv_send_ipi_allbutself()
[0.005904] APIC: send_IPI_all() replaced with hv_send_ipi_all()
[0.005905] APIC: send_IPI_self() replaced with hv_send_ipi_self()
[0.005952] APIC: wakeup_secondary_cpu_64() replaced with hv_vtl_wakeup_secondary_cpu()
[0.005970] clocksource: tsc-early: mask: 0xffffffffffffffff max_cycles: 0x1cd42f1dc63, max_idle_ns: 440795248933 ns
[0.005975] Calibrating delay loop (skipped) preset value.. 600.00 BogoMIPS (lpj=3000000)
[0.006051] x86/cpu: User Mode Instruction Prevention (UMIP) activated
[0.006057] Last level iTLB entries: 4KB 0, 2MB 0, 4MB 0
[0.006057] Last level dTLB entries: 4KB 0, 2MB 0, 4MB 0, 1GB 0
[0.006066] mitigations: Enabled attack vectors: user_kernel, user_user, SMT mitigations: auto
[0.006068] Speculative Store Bypass: Vulnerable
[0.006069] Spectre V2 : Vulnerable
[0.006069] RETBleed: Vulnerable
[0.006070] Spectre V2 : User space: Vulnerable
[0.006070] Spectre V1 : Vulnerable: __user pointer sanitization and usercopy barriers only; no swapgs barriers
[0.006073] x86/fpu: x87 FPU will use FXSAVE
[0.007316] Freeing SMP alternatives memory: 20K
[0.007318] pid_max: default: 65536 minimum: 512
[0.007443] Mount-cache hash table entries: 512 (order: 0, 4096 bytes, linear)
[0.007445] Mountpoint-cache hash table entries: 512 (order: 0, 4096 bytes, linear)
[0.007678] smpboot: CPU0: Intel(R) Xeon(R) Platinum 8460Y+ (family: 0x6, model: 0x8f, stepping: 0x8)
[0.007839] Performance Events: unsupported CPU family 6 model 143 no PMU driver, software events only.
[0.007843] signal: max sigframe size: 1040
[0.007876] rcu: Hierarchical SRCU implementation.
[0.007877] rcu: <09>Max phase no-delay instances is 1000.
[0.007912] Timer migration: 2 hierarchy levels; 8 children per group; 2 crossnode level
[0.008242] smp: Bringing up secondary CPUs ...
[0.008316] smpboot: x86: Booting SMP configuration:
[0.008316] .... node  #0, CPUs:       #32
[0.014644] smp: Brought up 1 node, 2 CPUs
[0.014644] smpboot: Total of 2 processors activated (1200.00 BogoMIPS)
[0.014644] Memory: 217908K/257408K available (4386K kernel code, 477K rwdata, 1240K rodata, 912K init, 596K bss, 34644K reserved, 0K cma-reserved)
[0.014644] devtmpfs: initialized
[0.014644] x86/mm: Memory block size: 128MB
[0.014644] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
[0.014644] posixtimers hash table entries: 32768 (order: 7, 524288 bytes, linear)
[0.014644] futex hash table entries: 16384 (1048576 bytes on 1 NUMA nodes, total 1024 KiB, linear).
[0.014644] NET: Registered PF_NETLINK/PF_ROUTE protocol family
[0.014644] initcall sbf_init blacklisted
[0.014644] PCI: Fatal: No config space access function found
[0.014644] iommu: Default domain type: Translated
[0.014644] iommu: DMA domain TLB invalidation policy: lazy mode
[0.014644] SCSI subsystem initialized
[0.014644] hv_vmbus: VMBus is present in DeviceTree
[0.014644] hv_vmbus: Vmbus version:5.3
[0.014644] PCI: System does not support PCI
[0.014644] clocksource: Switched to clocksource tsc-early
[0.014644] NET: Registered PF_UNIX/PF_LOCAL protocol family
[0.014644] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x1cd42f1dc63, max_idle_ns: 440795248933 ns
[0.014644] clocksource: Switched to clocksource tsc
[0.014644] Unpacking initramfs...
[0.015997] workingset: timestamp_bits=56 max_order=16 bucket_order=0
[0.016113] io scheduler mq-deadline registered
[0.016306] Serial: 8250/16550 driver, 4 ports, IRQ sharing enabled
[0.027025] serial8250: ttyS2 at I/O 0x3e8 (irq = 4, base_baud = 115200) is a 16550A
[0.037151] serial8250: ttyS3 at I/O 0x2e8 (irq = 3, base_baud = 115200) is a 16550A
[0.037222] printk: legacy console [ttynull0] enabled
[0.037442] hv_vmbus: registering driver uio_hv_generic
[0.037628] VFIO - User Level meta-driver version: 0.3
[0.037638] vfio_pci: add [1414:00ba[ffffffff:ffffffff]] class 0x000000/00000000
[0.038068] NET: Registered PF_VSOCK protocol family
[0.038070] hv_vmbus: registering driver hv_sock
[0.038139] IPI shorthand broadcast: enabled
[0.038627] sched_clock: Marking stable (31915213, 5973482)->(44901988, -7013293)
[0.039637] Demotion targets for Node 0: null
[0.046123] clk: Disabling unused clocks
[0.127203] Freeing initrd memory: 8336K
[0.127492] Freeing unused kernel image (initmem) memory: 912K
[0.127496] Write protecting the kernel read-only data: 8192k
[0.128170] Freeing unused kernel image (text/rodata gap) memory: 1756K
[0.128450] Freeing unused kernel image (rodata/data gap) memory: 808K
[0.128572] Freeing unused kernel image (bss_decrypted gap) memory: 344K
[0.175986] Run /underhill-init as init process
[0.175987] with arguments:
[0.175987] /underhill-init
[0.175988] with environment:
[0.175988] HOME=/
[0.175989] TERM=linux
[0.175989] OPENHCL_NVME_VFIO=1
[0.175990] MIMALLOC_ARENA_EAGER_COMMIT=0
[0.175990] acpi=off
[0.175991] OPENHCL_NVME_KEEP_ALIVE=host,privatepool
[0.175991] OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release
[0.175992] UNDERHILL_DIAG=1
[0.175992] HVLITE_LOG=info
[0.175993] RUST_BACKTRACE=full
[0.177114] underhill_init: Initial process: crate_name=underhill_init, crate_revision=54b35025e40bcb9d919f80c88c2d7000f968384d, crate_branch=user/hargar/servicing-branch
[0.177126] underhill_init: Mounting filesystems
[0.177203] underhill_init: Command line args: Options { setup_script: [], underhill_args: [] }
[0.177205] underhill_init: Setting system resource limits and parameters
[0.177593] underhill_init: Using 64 bytes of entropy from the host
[0.177611] underhill_init: registered vfio-pci as driver for nvme
[0.177670] underhill_init: running KERNEL_BOOT_TIME="171103343" "/bin/openvmm_hcl" "--pid" "/run/underhill.pid"
[0.177832] underhill_init: loading kernel module /lib/modules/000/pci-hyperv-intf.ko:
[0.178590] underhill_init: load complete for /lib/modules/000/pci-hyperv-intf.ko
[0.178615] underhill_init: loading kernel module /lib/modules/001/pci-hyperv.ko:
[0.179418] hv_vmbus: registering driver hv_pci
[0.179494] underhill_init: load complete for /lib/modules/001/pci-hyperv.ko
[0.179531] underhill_init: loading kernel module /lib/modules/999/hv_storvsc.ko: storvsc_vcpus_per_sub_channel=2048 storvsc_max_hw_queues=2 storvsc_ringbuffer_size=0x8000
[0.179700] underhill_core: INFO  VMM process crate_name="build_info" crate_revision="54b35025e40bcb9d919f80c88c2d7000f968384d" openhcl_version=""
[0.179738] underhill_core: INFO  boot loader times start=0x9411f76 end=0x941a573 sidecar_start=0x94122d2 sidecar_end=0x9414683 elapsed=3.4301ms sidecar_elapsed=913.7<c2><b5>s
[0.179953] diag_server: INFO  control starting control_address=VmAddress(Address { cid: ffffffff, port: 1 })
[0.179968] diag_server: INFO  data starting data_address=VmAddress(Address { cid: ffffffff, port: 2 })
[0.180233] mesh_tracing: INFO  updated trace filter filter="info"
[0.180415] hv_vmbus: registering driver hv_storvsc
[0.180461] underhill_init: load complete for /lib/modules/999/hv_storvsc.ko
[0.182217] mesh_tracing: INFO  updated trace filter filter="info"
[0.182957] guest_emulation_transport::process_loop: INFO  [GET] version negotiated protocol=NICKEL_REV2
[0.183007] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.183269] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:  VTL2 restart, getting servicing state from the host
[0.183713] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:  received servicing state from host saved_state_len=0x1d89b
[0.183850] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  kernel boot time kernel_boot_time_ns=0xa32d46f
[0.185334] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] read_from_dt: persisted_state_header present=true, sidecar=Enabled { enable_logging: false, cpu_threshold: Some(100) }"
[0.185344] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] found persisted state header"
[0.185350] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] reading topology from persisted state"
[0.185356] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] decoding protobuf of size 424"
[0.185361] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] Bump allocator: allocated 992 bytes in 4 allocations (64544 bytes free)"
[0.185369] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] persisted state: cpus_with_mapped_interrupts_no_io=[], cpus_with_outstanding_io=[]"
[0.185373] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] reclaim device tree memory 409759000-409799000"
[0.185378] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] openhcl_boot: logging enabled"
[0.185383] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] serial configuration: None"
[0.185389] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] sidecar: created node index=0 base_vp=0 vp_count=32 vnode=0"
[0.185394] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] sidecar: created node index=1 base_vp=32 vp_count=32 vnode=0"
[0.185400] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] sidecar starting, 2 nodes, 64 cpus, 0x342000 total bytes"
[0.185405] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] sidecar: boot_cpus parameter: boot_cpus=0,32"
[0.185411] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] uninitializing hypercalls"
[0.185416] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  openhcl_boot log line="[INFO] about to jump to kernel"
[0.185656] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  memory allocation mode memory_allocation_mode=Host
[0.185663] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  vtl2 ram vtl2_ram="0x408000000-0x419400000"
[0.185672] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  overriding dps to enable default_boot_always_attempt
[0.185677] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  flush logs result duration_us=0xb8
[0.186014] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  vtl0 ram vtl0_ram="0x0-0xf8000000(0), 0x100000000-0x408000000(0)"
[0.186022] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  vtl0 mmio vtl0_mmio="0xf8000000-0x100000000, 0xfe0000000-0x1000000000"
[0.186027] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  shared pool shared_pool=""
[0.186077] vmgs::storage: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  vmgs_capacity=0x400000
[0.186088] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  enabling alias map alias_map=0x200000000000
[0.186296] sidecar_client: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  sidecar client initialized with 2 node(s) layout=[0..20, 20..40]
[0.186316] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.186444] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.189385] underhill_mem::init: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  Skipping VTL 1 memory map creation
[0.189397] openhcl_dma_manager: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  create dma manager shared_ranges=[] private_ranges=[MemoryRange { start: 418000000, end: 419400000 }] vtom=0x0 isolation_type=None
[0.189413] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.189423] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  starting guest memory self test
[0.189441] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  guest memory self test complete
[0.190429] underhill_core::dispatch::vtl2_settings_worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:new_initial_controllers:  Initial VTL2 settings vtl2_settings=None config_timeout_in_seconds=0x5
[0.190449] underhill_core::nvme_manager::manager: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  starting nvme manager vp_count=0x40 save_restore_supported=true saved_state=true
[0.190540] underhill_core::nvme_manager::manager: INFO nvme_manager_restore:nvme_manager_worker_restore:  nvme manager worker restored 0 devices
[0.191647] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  Underhill MNF enabled? enable_mnf=false
[0.191677] vmbus_server::channels: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:  Limiting VmBus connections to version version=MaxVersionInfo { version: 60000, feature_flags: FeatureFlags { guest_specified_signal_parameters: true, channel_interrupt_redirection: true, modify_connection: true, client_id: false, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: true } }
[0.191720] vmbus_client::saved_state: INFO  channel restored key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 state=Opened
[0.191734] vmbus_client::saved_state: INFO  channel restored key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 state=Offered
[0.191741] vmbus_client::saved_state: INFO  channel restored key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 state=Opened
[0.191747] vmbus_client::saved_state: INFO  channel restored key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 state=Opened
[0.191754] vmbus_client::saved_state: INFO  channel restored key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 state=Offered
[0.191763] vmbus_client::saved_state: INFO  channel restored key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 state=Opened
[0.191769] vmbus_client::saved_state: INFO  channel restored key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 state=Opened
[0.191781] vmbus_client::saved_state: INFO  channel restored key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 state=Offered
[0.191787] vmbus_client::saved_state: INFO  channel restored key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0 state=Opened
[0.191798] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0 state=Opened
[0.191803] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-1 state=Opened
[0.191809] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-2 state=Opened
[0.191814] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-3 state=Opened
[0.191820] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-4 state=Opened
[0.191838] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-5 state=Opened
[0.191844] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-6 state=Opened
[0.191849] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-7 state=Opened
[0.191903] vmbus_server::channels: INFO  new channel offer_id=OfferId(0) key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.191922] vmbus_server::channels: INFO  new channel offer_id=OfferId(1) key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.191935] vmbus_server::channels: INFO  new channel offer_id=OfferId(2) key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.191949] vmbus_server::channels: INFO  new channel offer_id=OfferId(3) key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.191968] vmbus_server::channels: INFO  new channel offer_id=OfferId(4) key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.191979] vmbus_server::channels: INFO  new channel offer_id=OfferId(5) key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.191990] vmbus_server::channels: INFO  new channel offer_id=OfferId(6) key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.192006] vmbus_server::channels: INFO  new channel offer_id=OfferId(7) key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.192027] vmbus_server::channels: INFO  new channel offer_id=OfferId(8) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.192041] vmbus_server::channels: INFO  new channel offer_id=OfferId(9) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-1 confidential_ring_buffer=false confidential_external_memory=false
[0.192055] vmbus_server::channels: INFO  new channel offer_id=OfferId(a) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-2 confidential_ring_buffer=false confidential_external_memory=false
[0.192070] vmbus_server::channels: INFO  new channel offer_id=OfferId(b) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-3 confidential_ring_buffer=false confidential_external_memory=false
[0.192080] vmbus_server::channels: INFO  new channel offer_id=OfferId(c) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-4 confidential_ring_buffer=false confidential_external_memory=false
[0.192090] vmbus_server::channels: INFO  new channel offer_id=OfferId(d) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-5 confidential_ring_buffer=false confidential_external_memory=false
[0.192106] vmbus_server::channels: INFO  new channel offer_id=OfferId(e) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-6 confidential_ring_buffer=false confidential_external_memory=false
[0.192122] vmbus_server::channels: INFO  new channel offer_id=OfferId(f) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-7 confidential_ring_buffer=false confidential_external_memory=false
[0.192788] vmbus_server::channels: INFO  new channel offer_id=OfferId(10) key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 confidential_ring_buffer=true confidential_external_memory=false
[0.192847] vmbus_relay_intercept_device: INFO  matching channel offered offer=OfferInfo { offer: OfferChannel { interface_id: 0e0b6031-5213-4934-818b-38d90ced39db, instance_id: b6650ff7-33bc-4840-8048-e0676786f393, rsvd: [0, 0, 0, 0], flags: OfferFlags { enumerate_device_interface: true, confidential_ring_buffer: false, confidential_external_memory: false, require_pinned_external_memory: false, named_pipe_mode: true, tlnpi_provider: false }, mmio_megabytes: 0, user_defined: UserDefinedData([040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000]), subchannel_index: 0, mmio_megabytes_optional: 0, channel_id: ChannelId(5), monitor_id: ff, monitor_allocated: 0, is_dedicated: 1, connection_id: 2005 }, guest_to_host_interrupt: Interrupt { inner: InterruptInner }, request_send: SenderCore(Ma
[0.192850] vmbus_relay_intercept_device: nuallyDrop { value: MaybeDangling(Queue { remote: OnceLock(<uninit>), local: Mutex { data: LocalQueue { messages: ErasedVecDeque { cap: 0, head: 0, len: 0 }, ports: [], waker: Some(Waker { data: 0x431320e02c0, vtable: 0x7f359a9ed698 }), remote: false, receiver_gone: false, remove_closed: false, new_handler: 0x7f3599aef5c0 } }, receiver: UnsafeCell { .. } }) }), revoke_recv: ManuallyDrop { value: MaybeDangling(OneshotReceiverCore(Slot { state: Mutex { data: Waiting(None) }, receiver: UnsafeCell { .. } })) } }
[0.193103] vmbus_client: INFO  opening channel on host channel_id=0x5 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[0.199480] vmbus_server::channels::saved_state: INFO  channel restored key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 state=Open
[0.199489] vmbus_server::channels::saved_state: INFO  channel restored key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 state=Closed
[0.199495] vmbus_server::channels::saved_state: INFO  channel restored key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 state=Open
[0.199500] vmbus_server::channels::saved_state: INFO  channel restored key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 state=Open
[0.199506] vmbus_server::channels::saved_state: INFO  channel restored key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 state=Open
[0.199514] vmbus_server::channels::saved_state: INFO  channel restored key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 state=Open
[0.199520] vmbus_server::channels::saved_state: INFO  channel restored key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 state=Closed
[0.199525] vmbus_server::channels::saved_state: INFO  channel restored key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0 state=Open
[0.199531] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0 state=Open
[0.199536] vmbus_server::channels::saved_state: INFO  channel restored key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 state=Open
[0.199541] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-1 state=Open
[0.199547] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-2 state=Open
[0.199552] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-3 state=Open
[0.199557] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-4 state=Open
[0.199562] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-5 state=Open
[0.199567] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-6 state=Open
[0.199575] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-7 state=Open
[0.199639] hyperv_ic_guest::shutdown: INFO  version negotiated framework_version=3.0 message_version=3.2
[0.222587] state_unit: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/restore{ correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19}:restore_units:state_change{ operation="restore"}:  state change complete duration=26.579177ms
[0.222761] underhill_core: INFO  vm worker started
[0.223206] state_unit: INFO state_change{ operation="start"}:  state change complete duration=509.842<c2><b5>s
[0.223227] underhill_core::dispatch: INFO  resuming VM correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19 blackout_time_ms=0xff blackout_time="255.6092ms"
[0.223246] underhill_core::dispatch: INFO  servicing blackout phases correlation_id=9cd66387-b7ad-4094-a1c0-d642079cca19 method="host" total=Some(255.6092ms) phases=Some([("stop", 1.6447ms), ("save", 1.6462ms), ("shutdown", 245.4<c2><b5>s), ("flush", 195.6<c2><b5>s), ("handoff", 16.8952ms), ("bootloader", 3.4301ms), ("kernel_init", 191.3485ms), ("settings", 239.1<c2><b5>s), ("state_read", 465.1<c2><b5>s), ("state_fixup", 99.4<c2><b5>s), ("vm_build", 12.1113ms), ("restore", 26.6853ms), ("restore_notify", 7.2<c2><b5>s), ("resume_wait", 37.1<c2><b5>s), ("start", 559<c2><b5>s)])
[0.281051] virt_mshv_vtl::processor: INFO  sidecar exit
[0.281082] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: false })
[0.281419] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2e
[0.281933] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2e
[0.356162] mshv_vtl_sidecar 409a55000.sidecar: cpu 46: removing from sidecar
[0.356239] mshv_vtl_sidecar 409a55000.sidecar: cpu 46: remove complete
[0.356243] smpboot: Booting Node 0 Processor 46 APIC 0x2e
[0.398351] virt_mshv_vtl::processor: INFO  sidecar exit
[0.398360] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 28, write: true })
[0.398674] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x7
[0.399474] underhill_core::vp: INFO  onlining sidecar VP cpu=0x7
[0.476222] mshv_vtl_sidecar 4098b4000.sidecar: cpu 7: removing from sidecar
[0.476296] mshv_vtl_sidecar 4098b4000.sidecar: cpu 7: remove complete
[0.476301] smpboot: Booting Node 0 Processor 7 APIC 0x7
[0.540603] virt_mshv_vtl::processor: INFO  sidecar exit
[0.540613] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3fa, write: false })
[0.540956] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2
[0.541755] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2
[0.636217] mshv_vtl_sidecar 4098b4000.sidecar: cpu 2: removing from sidecar
[0.636293] mshv_vtl_sidecar 4098b4000.sidecar: cpu 2: remove complete
[0.636296] smpboot: Booting Node 0 Processor 2 APIC 0x2
[0.689316] virt_mshv_vtl::processor: INFO  sidecar exit
[0.689327] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3fd, write: false })
[0.689685] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1c
[0.690178] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1c
[0.776260] mshv_vtl_sidecar 4098b4000.sidecar: cpu 28: removing from sidecar
[0.776336] mshv_vtl_sidecar 4098b4000.sidecar: cpu 28: remove complete
[0.776340] smpboot: Booting Node 0 Processor 28 APIC 0x1c
[1.363833] virt_mshv_vtl::processor: INFO  sidecar exit
[1.363844] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: false })
[1.364242] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xe
[1.365032] underhill_core::vp: INFO  onlining sidecar VP cpu=0xe
[1.436222] mshv_vtl_sidecar 4098b4000.sidecar: cpu 14: removing from sidecar
[1.436299] mshv_vtl_sidecar 4098b4000.sidecar: cpu 14: remove complete
[1.436302] smpboot: Booting Node 0 Processor 14 APIC 0xe
[1.452449] virt_mshv_vtl::processor: INFO  sidecar exit
[1.452460] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: true })
[1.452843] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1
[1.453317] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1
[1.556229] mshv_vtl_sidecar 4098b4000.sidecar: cpu 1: removing from sidecar
[1.556307] mshv_vtl_sidecar 4098b4000.sidecar: cpu 1: remove complete
[1.556312] smpboot: Booting Node 0 Processor 1 APIC 0x1
[1.557466] virt_mshv_vtl::processor: INFO  sidecar exit
[1.557473] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: false })
[1.557761] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2a
[1.558274] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2a
[1.666275] mshv_vtl_sidecar 409a55000.sidecar: cpu 42: removing from sidecar
[1.675992] mshv_vtl_sidecar 409a55000.sidecar: cpu 42: remove complete
[1.676004] smpboot: Booting Node 0 Processor 42 APIC 0x2a
[1.694523] virt_mshv_vtl::processor: INFO  sidecar exit
[1.694540] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 28, write: true })
[1.694889] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x31
[1.695385] underhill_core::vp: INFO  onlining sidecar VP cpu=0x31
[1.708139] virt_mshv_vtl::processor: INFO  sidecar exit
[1.708148] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: true })
[1.708489] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1a
[1.711698] virt_mshv_vtl::processor: INFO  sidecar exit
[1.711705] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: false })
[1.712084] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x10
[1.775816] virt_mshv_vtl::processor: INFO  sidecar exit
[1.775826] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: false })
[1.776340] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x9
[1.836158] mshv_vtl_sidecar 409a55000.sidecar: cpu 49: removing from sidecar
[1.836235] mshv_vtl_sidecar 409a55000.sidecar: cpu 49: remove complete
[1.836241] smpboot: Booting Node 0 Processor 49 APIC 0x31
[1.837480] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1a
[1.837544] virt_mshv_vtl::processor: INFO  sidecar exit
[1.837554] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 28, write: true })
[1.837988] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1e
[1.844298] virt_mshv_vtl::processor: INFO  sidecar exit
[1.844306] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 28, write: true })
[1.844758] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3
[1.855116] virt_mshv_vtl::processor: INFO  sidecar exit
[1.855124] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: false })
[1.855626] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x13
[1.956798] mshv_vtl_sidecar 4098b4000.sidecar: cpu 26: removing from sidecar
[1.957013] mshv_vtl_sidecar 4098b4000.sidecar: cpu 26: remove complete
[1.957022] smpboot: Booting Node 0 Processor 26 APIC 0x1a
[1.958859] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1e
[2.106243] mshv_vtl_sidecar 4098b4000.sidecar: cpu 30: removing from sidecar
[2.106319] mshv_vtl_sidecar 4098b4000.sidecar: cpu 30: remove complete
[2.106327] smpboot: Booting Node 0 Processor 30 APIC 0x1e
[2.107468] underhill_core::vp: INFO  onlining sidecar VP cpu=0x10
[2.146270] mshv_vtl_sidecar 4098b4000.sidecar: cpu 16: removing from sidecar
[2.146347] mshv_vtl_sidecar 4098b4000.sidecar: cpu 16: remove complete
[2.146353] smpboot: Booting Node 0 Processor 16 APIC 0x10
[2.147448] underhill_core::vp: INFO  onlining sidecar VP cpu=0x9
[2.147860] mshv_vtl_sidecar 4098b4000.sidecar: cpu 9: removing from sidecar
[2.147937] mshv_vtl_sidecar 4098b4000.sidecar: cpu 9: remove complete
[2.147943] smpboot: Booting Node 0 Processor 9 APIC 0x9
[2.149002] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3
[2.149347] mshv_vtl_sidecar 4098b4000.sidecar: cpu 3: removing from sidecar
[2.149423] mshv_vtl_sidecar 4098b4000.sidecar: cpu 3: remove complete
[2.149427] smpboot: Booting Node 0 Processor 3 APIC 0x3
[2.150491] underhill_core::vp: INFO  onlining sidecar VP cpu=0x13
[2.150826] mshv_vtl_sidecar 4098b4000.sidecar: cpu 19: removing from sidecar
[2.150901] mshv_vtl_sidecar 4098b4000.sidecar: cpu 19: remove complete
[2.150905] smpboot: Booting Node 0 Processor 19 APIC 0x13
[2.398869] virt_mshv_vtl::processor: INFO  sidecar exit
[2.398880] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: true })
[2.399205] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1b
[2.399675] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1b
[2.506307] mshv_vtl_sidecar 4098b4000.sidecar: cpu 27: removing from sidecar
[2.506385] mshv_vtl_sidecar 4098b4000.sidecar: cpu 27: remove complete
[2.506392] smpboot: Booting Node 0 Processor 27 APIC 0x1b
[3.620317] virt_mshv_vtl::processor: INFO  sidecar exit
[3.620331] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 28, write: true })
[3.620783] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x4
[3.621574] underhill_core::vp: INFO  onlining sidecar VP cpu=0x4
[3.746297] mshv_vtl_sidecar 4098b4000.sidecar: cpu 4: removing from sidecar
[3.746374] mshv_vtl_sidecar 4098b4000.sidecar: cpu 4: remove complete
[3.746381] smpboot: Booting Node 0 Processor 4 APIC 0x4
[3.792263] virt_mshv_vtl::processor: INFO  sidecar exit
[3.792275] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3fd, write: false })
[3.792593] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1f
[3.793067] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1f
[3.946276] mshv_vtl_sidecar 4098b4000.sidecar: cpu 31: removing from sidecar
[3.946353] mshv_vtl_sidecar 4098b4000.sidecar: cpu 31: remove complete
[3.946358] smpboot: Booting Node 0 Processor 31 APIC 0x1f
[4.015930] virt_mshv_vtl::processor: INFO  sidecar exit
[4.015941] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: true })
[4.016326] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x8
[4.016802] underhill_core::vp: INFO  onlining sidecar VP cpu=0x8
[4.126455] mshv_vtl_sidecar 4098b4000.sidecar: cpu 8: removing from sidecar
[4.126534] mshv_vtl_sidecar 4098b4000.sidecar: cpu 8: remove complete
[4.126541] smpboot: Booting Node 0 Processor 8 APIC 0x8
[6.713410] virt_mshv_vtl::processor: INFO  sidecar exit
[6.713424] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 28, write: true })
[6.713963] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x27
[6.714770] underhill_core::vp: INFO  onlining sidecar VP cpu=0x27
[6.846329] mshv_vtl_sidecar 409a55000.sidecar: cpu 39: removing from sidecar
[6.846410] mshv_vtl_sidecar 409a55000.sidecar: cpu 39: remove complete
[6.846421] smpboot: Booting Node 0 Processor 39 APIC 0x27
[6.848843] virt_mshv_vtl::processor: INFO  sidecar exit
[6.848867] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: true })
[6.849219] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2c
[6.849694] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2c
[6.966302] mshv_vtl_sidecar 409a55000.sidecar: cpu 44: removing from sidecar
[6.966380] mshv_vtl_sidecar 409a55000.sidecar: cpu 44: remove complete
[6.966387] smpboot: Booting Node 0 Processor 44 APIC 0x2c
[6.967731] virt_mshv_vtl::processor: INFO  sidecar exit
[6.967738] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3fd, write: false })
[6.968067] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2d
[6.968553] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2d
[7.126305] mshv_vtl_sidecar 409a55000.sidecar: cpu 45: removing from sidecar
[7.126384] mshv_vtl_sidecar 409a55000.sidecar: cpu 45: remove complete
[7.126391] smpboot: Booting Node 0 Processor 45 APIC 0x2d
[7.133009] virt_mshv_vtl::processor: INFO  sidecar exit
[7.133021] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 3f9, write: true })
[7.133373] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x16
[7.133872] underhill_core::vp: INFO  onlining sidecar VP cpu=0x16
[7.256301] mshv_vtl_sidecar 4098b4000.sidecar: cpu 22: removing from sidecar
[7.256378] mshv_vtl_sidecar 4098b4000.sidecar: cpu 22: remove complete
[7.256385] smpboot: Booting Node 0 Processor 22 APIC 0x16
[7.292779] virt_mshv_vtl::processor: INFO  sidecar exit
[7.292791] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 28, write: true })
[7.293200] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x5
[7.293992] underhill_core::vp: INFO  onlining sidecar VP cpu=0x5
[7.416323] mshv_vtl_sidecar 4098b4000.sidecar: cpu 5: removing from sidecar
[7.416402] mshv_vtl_sidecar 4098b4000.sidecar: cpu 5: remove complete
[7.416408] smpboot: Booting Node 0 Processor 5 APIC 0x5
[7.417766] virt_mshv_vtl::processor: INFO  sidecar exit
[7.417777] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 28, write: true })
[7.418142] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x11
[7.418935] underhill_core::vp: INFO  onlining sidecar VP cpu=0x11
[7.556306] mshv_vtl_sidecar 4098b4000.sidecar: cpu 17: removing from sidecar
[7.556383] mshv_vtl_sidecar 4098b4000.sidecar: cpu 17: remove complete
[7.556389] smpboot: Booting Node 0 Processor 17 APIC 0x11
