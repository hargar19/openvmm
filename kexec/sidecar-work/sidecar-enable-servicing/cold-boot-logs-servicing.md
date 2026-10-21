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
[0.000003] e820: update [mem 0x00000000-0x00000fff] usable ==> reserved
[0.000005] e820: remove [mem 0x000a0000-0x000fffff] usable
[0.000009] last_pfn = 0x418000 max_arch_pfn = 0x1000000000
[0.000010] x86/PAT: PAT support disabled because CONFIG_X86_PAT is disabled in the kernel.
[0.000012] x86/PAT: Configuration [0-7]: WB  WT  UC- UC  WB  WT  UC- UC
[0.000014] last_pfn = 0x0 max_arch_pfn = 0x1000000000
[0.000017] Using GB pages for direct mapping
[0.000362] printk: log buffer data + meta data: 131072 + 458752 = 589824 bytes
[0.000364] printk: early log buf free: 13080(79%)
[0.000364] RAMDISK: [mem 0x408f06000-0x409729fff]
[0.000424] OF: reserved mem: Reserved memory: No reserved-memory node in the DT
[0.000570] NODE_DATA(0) allocated [mem 0x417f53900-0x417f5817f]
[0.000592] Zone ranges:
[0.000592] DMA      empty
[0.000593] DMA32    empty
[0.000594] Normal   [mem 0x0000000408014000-0x0000000417ffffff]
[0.000595] Device   empty
[0.000595] Movable zone start for each node
[0.000596] Early memory node ranges
[0.000596] node   0: [mem 0x0000000408014000-0x0000000408dfffff]
[0.000597] node   0: [mem 0x0000000408e1a000-0x000000040972cfff]
[0.000598] node   0: [mem 0x0000000409759000-0x0000000409798fff]
[0.000598] node   0: [mem 0x000000040989d000-0x00000004098b3fff]
[0.000599] node   0: [mem 0x0000000409bf6000-0x0000000417ffffff]
[0.000599] Initmem setup node 0 [mem 0x0000000408014000-0x0000000417ffffff]
[0.000622] On node 0, zone Normal: 20 pages in unavailable ranges
[0.000636] On node 0, zone Normal: 26 pages in unavailable ranges
[0.000637] On node 0, zone Normal: 44 pages in unavailable ranges
[0.000640] On node 0, zone Normal: 260 pages in unavailable ranges
[0.000964] On node 0, zone Normal: 834 pages in unavailable ranges
[0.000993] IMCR and PIC compatibility mode.
[0.001015] TSC deadline timer available
[0.001018] CPU topo: Max. logical packages:   1
[0.001019] CPU topo: Max. logical dies:       1
[0.001019] CPU topo: Max. dies per package:   1
[0.001021] CPU topo: Max. threads per core:   2
[0.001022] CPU topo: Num. cores per package:    32
[0.001023] CPU topo: Num. threads per package:  64
[0.001023] CPU topo: Allowing 64 present CPUs plus 0 hotplug CPUs
[0.001029] Cannot find an available gap in the 32-bit address range
[0.001029] PCI devices with unassigned 32-bit BARs may not work!
[0.001029] [mem 0x418100000-0x4184fffff] available for PCI devices
[0.001030] Booting paravirtualized kernel on Hyper-V
[0.001035] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
[0.001040] setup_percpu: NR_CPUS:2048 nr_cpumask_bits:64 nr_cpu_ids:64 nr_node_ids:1
[0.003156] percpu: Embedded 41 pages/cpu s127296 r8192 d32448 u262144
[0.003160] pcpu-alloc: s127296 r8192 d32448 u262144 alloc=1*2097152
[0.003162] pcpu-alloc: [0] 00 01 02 03 04 05 06 07 [0] 08 09 10 11 12 13 14 15
[0.003166] pcpu-alloc: [0] 16 17 18 19 20 21 22 23 [0] 24 25 26 27 28 29 30 31
[0.003170] pcpu-alloc: [0] 32 33 34 35 36 37 38 39 [0] 40 41 42 43 44 45 46 47
[0.003174] pcpu-alloc: [0] 48 49 50 51 52 53 54 55 [0] 56 57 58 59 60 61 62 63
[0.003190] Hyper-V: PV spinlocks enabled
[0.003192] PV qspinlock hash table entries: 256 (order: 0, 4096 bytes, linear)
[0.003197] Kernel command line: loglevel=8 log_buf_len=128K printk.time=1 console_msg_format=syslog uio_hv_generic.no_mask=1 coredump_filter=0x33 cpufreq.off=1 cpuidle.off=1 cryptomgr.notests idle=halt initcall_blacklist=init_real_mode,sbf_init lpj=3000000 no_timer_check noxsave oops=panic panic_on_warn=0 panic_print=0 panic=-1 printk.devkmsg=on reboot=t rootfstype=tmpfs sysctl.vm.compaction_proactiveness=0 tsc=reliable unknown_nmi_panic=1 vfio_pci.ids=1414:00ba vfio.enable_unsafe_noiommu_mode=1 rdinit=/underhill-init OPENHCL_NVME_VFIO=1 hv_storvsc.storvsc_vcpus_per_sub_channel=2048 hv_storvsc.storvsc_max_hw_queues=2 hv_storvsc.storvsc_ringbuffer_size=0x8000 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off iommu=off pci=off swiotlb=1,1 console=ttynull OPENHCL_NVME_KEEP_ALIVE=host,privatepool boot_cpus=0,32 hv_vmbus.message_connection_id=0x800074 OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info RUST_BACKTRACE=full
[0.003241] blacklisting initcall init_real_mode
[0.003243] blacklisting initcall sbf_init
[0.003258] Kernel: The 'panic_print' parameter is now deprecated. Please use 'panic_sys_info' and 'panic_console_replay' instead.
[0.003343] Unknown kernel command line parameters "OPENHCL_NVME_VFIO=1 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off OPENHCL_NVME_KEEP_ALIVE=host,privatepool OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info RUST_BACKTRACE=full", will be passed to user space.
[0.003354] random: crng init done
[0.003383] Dentry cache hash table entries: 32768 (order: 6, 262144 bytes, linear)
[0.003397] Inode-cache hash table entries: 16384 (order: 5, 131072 bytes, linear)
[0.004231] Fallback order for Node 0: 0
[0.004235] Built 1 zonelists, mobility grouping on.  Total pages: 64352
[0.004236] Policy zone: Normal
[0.004237] mem auto-init: stack:off, heap alloc:off, heap free:off
[0.004836] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=64, Nodes=1
[0.005177] Dynamic Preempt: none
[0.005412] rcu: Preemptible hierarchical RCU implementation.
[0.005413] rcu: <09>RCU restricting CPUs from NR_CPUS=2048 to nr_cpu_ids=64.
[0.005415] rcu: RCU calculated value of scheduler-enlistment delay is 10 jiffies.
[0.005415] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=64
[0.005476] Using NULL legacy PIC
[0.005477] NR_IRQS: 131328, nr_irqs: 1040, preallocated irqs: 0
[0.005540] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[0.005543] Persistent clock returned invalid value
[0.005671] APIC: Switch to symmetric I/O mode setup
[0.005680] x2apic enabled
[0.005708] APIC: Switched APIC routing to: physical x2apic
[0.005711] Hyper-V: Host Build 10.0.29627.1000-1-0
[0.005713] Hyper-V: enabling crash_kexec_post_notifiers
[0.005818] Hyper-V: Using IPI hypercalls
[0.005820] APIC: send_IPI() replaced with hv_send_ipi()
[0.005824] APIC: send_IPI_mask() replaced with hv_send_ipi_mask()
[0.005826] APIC: send_IPI_mask_allbutself() replaced with hv_send_ipi_mask_allbutself()
[0.005828] APIC: send_IPI_allbutself() replaced with hv_send_ipi_allbutself()
[0.005829] APIC: send_IPI_all() replaced with hv_send_ipi_all()
[0.005831] APIC: send_IPI_self() replaced with hv_send_ipi_self()
[0.005888] APIC: wakeup_secondary_cpu_64() replaced with hv_vtl_wakeup_secondary_cpu()
[0.005906] clocksource: tsc-early: mask: 0xffffffffffffffff max_cycles: 0x1cd42f1dc63, max_idle_ns: 440795248933 ns
[0.005910] Calibrating delay loop (skipped) preset value.. 600.00 BogoMIPS (lpj=3000000)
[0.005985] x86/cpu: User Mode Instruction Prevention (UMIP) activated
[0.005991] Last level iTLB entries: 4KB 0, 2MB 0, 4MB 0
[0.005992] Last level dTLB entries: 4KB 0, 2MB 0, 4MB 0, 1GB 0
[0.006000] mitigations: Enabled attack vectors: user_kernel, user_user, SMT mitigations: auto
[0.006002] Speculative Store Bypass: Vulnerable
[0.006003] Spectre V2 : Vulnerable
[0.006004] RETBleed: Vulnerable
[0.006004] Spectre V2 : User space: Vulnerable
[0.006005] Spectre V1 : Vulnerable: __user pointer sanitization and usercopy barriers only; no swapgs barriers
[0.006007] x86/fpu: x87 FPU will use FXSAVE
[0.007498] Freeing SMP alternatives memory: 20K
[0.007500] pid_max: default: 65536 minimum: 512
[0.007622] Mount-cache hash table entries: 512 (order: 0, 4096 bytes, linear)
[0.007624] Mountpoint-cache hash table entries: 512 (order: 0, 4096 bytes, linear)
[0.007875] smpboot: CPU0: Intel(R) Xeon(R) Platinum 8460Y+ (family: 0x6, model: 0x8f, stepping: 0x8)
[0.008040] Performance Events: unsupported CPU family 6 model 143 no PMU driver, software events only.
[0.008045] signal: max sigframe size: 1040
[0.008079] rcu: Hierarchical SRCU implementation.
[0.008080] rcu: <09>Max phase no-delay instances is 1000.
[0.008118] Timer migration: 2 hierarchy levels; 8 children per group; 2 crossnode level
[0.008456] smp: Bringing up secondary CPUs ...
[0.008531] smpboot: x86: Booting SMP configuration:
[0.008531] .... node  #0, CPUs:       #32
[0.014655] smp: Brought up 1 node, 2 CPUs
[0.014655] smpboot: Total of 2 processors activated (1200.00 BogoMIPS)
[0.014655] Memory: 217908K/257408K available (4386K kernel code, 477K rwdata, 1240K rodata, 912K init, 596K bss, 34644K reserved, 0K cma-reserved)
[0.014655] devtmpfs: initialized
[0.014655] x86/mm: Memory block size: 128MB
[0.014655] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
[0.014655] posixtimers hash table entries: 32768 (order: 7, 524288 bytes, linear)
[0.014655] futex hash table entries: 16384 (1048576 bytes on 1 NUMA nodes, total 1024 KiB, linear).
[0.014655] NET: Registered PF_NETLINK/PF_ROUTE protocol family
[0.014655] initcall sbf_init blacklisted
[0.014655] PCI: Fatal: No config space access function found
[0.014655] iommu: Default domain type: Translated
[0.014655] iommu: DMA domain TLB invalidation policy: lazy mode
[0.014655] SCSI subsystem initialized
[0.014655] hv_vmbus: VMBus is present in DeviceTree
[0.014655] hv_vmbus: Vmbus version:5.3
[0.014655] PCI: System does not support PCI
[0.014655] clocksource: Switched to clocksource tsc-early
[0.014655] NET: Registered PF_UNIX/PF_LOCAL protocol family
[0.014655] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x1cd42f1dc63, max_idle_ns: 440795248933 ns
[0.014655] clocksource: Switched to clocksource tsc
[0.014655] Unpacking initramfs...
[0.026877] workingset: timestamp_bits=56 max_order=16 bucket_order=0
[0.026984] io scheduler mq-deadline registered
[0.027187] Serial: 8250/16550 driver, 4 ports, IRQ sharing enabled
[0.038726] serial8250: ttyS2 at I/O 0x3e8 (irq = 4, base_baud = 115200) is a 16550A
[0.050594] serial8250: ttyS3 at I/O 0x2e8 (irq = 3, base_baud = 115200) is a 16550A
[0.050696] printk: legacy console [ttynull0] enabled
[0.050935] hv_vmbus: registering driver uio_hv_generic
[0.055952] VFIO - User Level meta-driver version: 0.3
[0.055961] vfio_pci: add [1414:00ba[ffffffff:ffffffff]] class 0x000000/00000000
[0.056366] NET: Registered PF_VSOCK protocol family
[0.056368] hv_vmbus: registering driver hv_sock
[0.056437] IPI shorthand broadcast: enabled
[0.056928] sched_clock: Marking stable (50273920, 5908935)->(63771873, -7589018)
[0.066085] Demotion targets for Node 0: null
[0.066164] clk: Disabling unused clocks
[0.127367] Freeing initrd memory: 8336K
[0.127668] Freeing unused kernel image (initmem) memory: 912K
[0.127673] Write protecting the kernel read-only data: 8192k
[0.128361] Freeing unused kernel image (text/rodata gap) memory: 1756K
[0.128646] Freeing unused kernel image (rodata/data gap) memory: 808K
[0.128753] Freeing unused kernel image (bss_decrypted gap) memory: 344K
[0.175919] Run /underhill-init as init process
[0.175920] with arguments:
[0.175921] /underhill-init
[0.175921] with environment:
[0.175922] HOME=/
[0.175922] TERM=linux
[0.175923] OPENHCL_NVME_VFIO=1
[0.175923] MIMALLOC_ARENA_EAGER_COMMIT=0
[0.175924] acpi=off
[0.175924] OPENHCL_NVME_KEEP_ALIVE=host,privatepool
[0.175924] OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release
[0.175925] UNDERHILL_DIAG=1
[0.175925] HVLITE_LOG=info
[0.175926] RUST_BACKTRACE=full
[0.177075] underhill_init: Initial process: crate_name=underhill_init, crate_revision=54b35025e40bcb9d919f80c88c2d7000f968384d, crate_branch=user/hargar/servicing-branch
[0.177090] underhill_init: Mounting filesystems
[0.177168] underhill_init: Command line args: Options { setup_script: [], underhill_args: [] }
[0.177169] underhill_init: Setting system resource limits and parameters
[0.177586] underhill_init: Using 64 bytes of entropy from the host
[0.177603] underhill_init: registered vfio-pci as driver for nvme
[0.177659] underhill_init: running KERNEL_BOOT_TIME="171126799" "/bin/openvmm_hcl" "--pid" "/run/underhill.pid"
[0.177829] underhill_init: loading kernel module /lib/modules/000/pci-hyperv-intf.ko:
[0.178640] underhill_init: load complete for /lib/modules/000/pci-hyperv-intf.ko
[0.178662] underhill_init: loading kernel module /lib/modules/001/pci-hyperv.ko:
[0.179460] hv_vmbus: registering driver hv_pci
[0.179521] underhill_init: load complete for /lib/modules/001/pci-hyperv.ko
[0.179545] underhill_init: loading kernel module /lib/modules/999/hv_storvsc.ko: storvsc_vcpus_per_sub_channel=2048 storvsc_max_hw_queues=2 storvsc_ringbuffer_size=0x8000
[0.180118] underhill_core: INFO  VMM process crate_name="build_info" crate_revision="54b35025e40bcb9d919f80c88c2d7000f968384d" openhcl_version=""
[0.180157] underhill_core: INFO  boot loader times start=0x7cd14 end=0x8b261 sidecar_start=0x7d0af sidecar_end=0x85358 elapsed=5.8701ms sidecar_elapsed=3.3449ms
[0.180399] hv_vmbus: registering driver hv_storvsc
[0.180426] diag_server: INFO  control starting control_address=VmAddress(Address { cid: ffffffff, port: 1 })
[0.180440] diag_server: INFO  data starting data_address=VmAddress(Address { cid: ffffffff, port: 2 })
[0.180456] underhill_init: load complete for /lib/modules/999/hv_storvsc.ko
[0.180514] mesh_tracing: INFO  updated trace filter filter="info"
[0.182540] mesh_tracing: INFO  updated trace filter filter="info"
[0.183562] guest_emulation_transport::process_loop: INFO  [GET] version negotiated protocol=NICKEL_REV2
[0.183624] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.183998] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  kernel boot time kernel_boot_time_ns=0xa33300f
[0.185561] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] read_from_dt: persisted_state_header present=false, sidecar=Enabled { enable_logging: false, cpu_threshold: Some(100) }"
[0.185582] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] reading topology from host device tree"
[0.185600] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] reclaim device tree memory 409759000-409799000"
[0.185619] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] persisted state region sized to MemoryRange {"
[0.185636] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    start: 0x408000000,"
[0.185654] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    end: 0x408014000,"
[0.185672] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="}, remainder MemoryRange {"
[0.185688] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    start: 0x408014000,"
[0.185705] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    end: 0x408200000,"
[0.185720] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="}"
[0.185726] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] vtl2 gpa pool coming from heuristics table: Release"
[0.185731] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] Extrapolated VTL2 DMA hint: 5120 pages (20 MiB) for 64 VPs and 276 MiB VTL2 memory"
[0.185736] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] allocated VTL2 pool at MemoryRange {"
[0.185740] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    start: 0x418000000,"
[0.185745] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    end: 0x419400000,"
[0.185749] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="}"
[0.185754] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] openhcl_boot: logging enabled"
[0.185758] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] serial configuration: None"
[0.185763] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] sidecar: created node index=0 base_vp=0 vp_count=32 vnode=0"
[0.185768] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] sidecar: created node index=1 base_vp=32 vp_count=32 vnode=0"
[0.185772] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] sidecar starting, 2 nodes, 64 cpus, 0x342000 total bytes"
[0.185777] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] sidecar: boot_cpus parameter: boot_cpus=0,32"
[0.185781] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] uninitializing hypercalls"
[0.185786] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] about to jump to kernel"
[0.186060] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  memory allocation mode memory_allocation_mode=Host
[0.186067] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  vtl2 ram vtl2_ram="0x408000000-0x419400000"
[0.186075] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  overriding dps to enable default_boot_always_attempt
[0.186417] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  vtl0 ram vtl0_ram="0x0-0xf8000000(0), 0x100000000-0x408000000(0)"
[0.186461] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  vtl0 mmio vtl0_mmio="0xf8000000-0x100000000, 0xfe0000000-0x1000000000"
[0.186480] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  shared pool shared_pool=""
[0.186675] vmgs::storage: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:vmgs_open:  vmgs_capacity=0x400000
[0.187337] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  enabling alias map alias_map=0x200000000000
[0.187580] sidecar_client: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  sidecar client initialized with 2 node(s) layout=[0..20, 20..40]
[0.187600] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.187741] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.202062] underhill_mem::init: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  VTL 1 memory map created
[0.202086] openhcl_dma_manager: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  create dma manager shared_ranges=[] private_ranges=[MemoryRange { start: 418000000, end: 419400000 }] vtom=0x0 isolation_type=None
[0.202101] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.202120] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  starting guest memory self test
[0.202145] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  guest memory self test complete
[0.202927] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Reading security profile tee_type=None secure_boot=false tpm_enabled=false tpm_persisted=true hardware_sealing_supported=false hardware_sealing_policy=Hash
[0.202950] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Reading VM ID from VMGS
[0.202972] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  attempt to unlock VMGS file attempt=0x0
[0.202990] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Key-encryption key retrieval not required
[0.203009] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Successfully retrieved key-encryption key
[0.203027] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Reading key protector from VMGS dek_minimal_size=0x100
[0.203048] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Deriving keys key_derivation_policy=None vmgs_encrypted=false op_type=BeginDecryptVmgs
[0.203069] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  encryption policy guest_state_encryption_policy=None strict_encryption_policy=false
[0.203077] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  initial vmgs encryption state is_encrypted=false is_gsp_by_id=false is_gsp=false found_dek=false
[0.203081] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  attempting GSP
[0.203089] guest_emulation_transport::client: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Getting guest state protection data op_type=BeginGspCallback
[0.203484] guest_emulation_transport::client: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Got guest state protection data op_type=GspCallback latency=0
[0.203503] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  GSP response request_data_length_in_vmgs=0x0 no_rpc_server=true requires_rpc_server=false encrypted_gsp_length=0x0
[0.203521] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  attempting GSP By Id
[0.203650] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Encryption sources kek=false gsp_available=false gsp=false gsp_by_id_available=Some(false) gsp_by_id=false hw_sealing=false
[0.203655] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  No VMGS encryption used.
[0.203660] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Unlocking VMGS
[0.203665] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Encryption disabled, skipping unlock vmgs data store
[0.203672] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Unlocked datastore op_type=DecryptVmgs success=true decrypt_gsp_type=None encrypt_gsp_type=None latency=0
[0.203678] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  determine if refreshing tpm seeds is needed state_refresh_request_from_gsp=false vm_id_changed=false
[0.204045] underhill_core::dispatch::vtl2_settings_worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:new_initial_controllers:  Initial VTL2 settings vtl2_settings=None config_timeout_in_seconds=0x5
[0.204075] underhill_core::nvme_manager::manager: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  starting nvme manager vp_count=0x40 save_restore_supported=true saved_state=false
[0.204858] underhill_core::emuplat::local_clock: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  no saved skew found: defaulting to host local time skew=-25200s
[0.209107] firmware_uefi::service::nvram: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:base_chipset_build:add_device{ device="uefi"}:  configuring secure boot enabled=false
[0.212174] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  Underhill MNF enabled? enable_mnf=false
[0.212201] vmbus_server::channels: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  Limiting VmBus connections to version version=MaxVersionInfo { version: 60000, feature_flags: FeatureFlags { guest_specified_signal_parameters: true, channel_interrupt_redirection: true, modify_connection: true, client_id: false, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: true } }
[0.212425] vmbus_client: INFO  VmBus client connected, requesting offers version=VersionInfo { version: Copper, feature_flags: FeatureFlags { guest_specified_signal_parameters: true, channel_interrupt_redirection: true, modify_connection: true, client_id: true, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: false } }
[0.212503] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x1 interface_id=525074dc-8985-46e2-8057-a307dc18a502 instance_id=1eccfd72-4b41-45ef-b73a-4a6e44c12924 subchannel_index=0x0
[0.212515] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x2 interface_id=3375baf4-9e15-4b30-b765-67acb10d607b instance_id=4487b255-b88c-403f-bb51-d1f69cf17f87 subchannel_index=0x0
[0.212526] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x3 interface_id=57164f39-9115-4e78-ab55-382f3bd5422d instance_id=fd149e91-82e0-4a7d-afa6-2a4166cbd7c0 subchannel_index=0x0
[0.212536] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x4 interface_id=a9a0f4e7-5a45-4d96-b827-8a841e8c03e6 instance_id=242ff919-07db-4180-9c2e-b86cb68c8c55 subchannel_index=0x0
[0.212546] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x5 interface_id=0e0b6031-5213-4934-818b-38d90ced39db instance_id=b6650ff7-33bc-4840-8048-e0676786f393 subchannel_index=0x0
[0.212561] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x6 interface_id=9527e630-d0ae-497b-adce-e80ab0175caf instance_id=2dd1ce17-079e-403c-b352-a1921ee207ee subchannel_index=0x0
[0.212571] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x7 interface_id=35fa2e29-ea23-4236-96ae-3a6ebacba440 instance_id=2450ee40-33bf-4fbd-892e-9fb06e9214cf subchannel_index=0x0
[0.212586] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x8 interface_id=276aacf4-ac15-426c-98dd-7521ad3f01fe instance_id=f5bee29c-1741-4aad-a4c2-8fdedb46dcc2 subchannel_index=0x0
[0.212597] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x9 interface_id=ba6163d9-04a1-4d29-b605-72e2ffb1dc7f instance_id=443ce0be-fefb-4d2b-afdb-448da1809a1e subchannel_index=0x0
[0.212610] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0xa interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=61f0a39b-c778-405b-bc5b-ffca0b817e4b subchannel_index=0x0
[0.212618] vmbus_client: INFO  VmBus client connected, offers delivered version=VersionInfo { version: Copper, feature_flags: FeatureFlags { guest_specified_signal_parameters: true, channel_interrupt_redirection: true, modify_connection: true, client_id: true, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: false } }
[0.212714] vmbus_server::channels: INFO  new channel offer_id=OfferId(0) key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.212760] vmbus_server::channels: INFO  new channel offer_id=OfferId(1) key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.212798] vmbus_server::channels: INFO  new channel offer_id=OfferId(2) key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.212824] vmbus_server::channels: INFO  new channel offer_id=OfferId(3) key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.212844] vmbus_server::channels: INFO  new channel offer_id=OfferId(4) key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.212882] vmbus_server::channels: INFO  new channel offer_id=OfferId(5) key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.212921] vmbus_server::channels: INFO  new channel offer_id=OfferId(6) key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.212959] vmbus_server::channels: INFO  new channel offer_id=OfferId(7) key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.213013] vmbus_server::channels: INFO  new channel offer_id=OfferId(8) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.213736] vmbus_server::channels: INFO  new channel offer_id=OfferId(9) key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 confidential_ring_buffer=true confidential_external_memory=false
[0.213791] vmbus_relay_intercept_device: INFO  matching channel offered offer=OfferInfo { offer: OfferChannel { interface_id: 0e0b6031-5213-4934-818b-38d90ced39db, instance_id: b6650ff7-33bc-4840-8048-e0676786f393, rsvd: [0, 0, 0, 0], flags: OfferFlags { enumerate_device_interface: true, confidential_ring_buffer: false, confidential_external_memory: false, require_pinned_external_memory: false, named_pipe_mode: true, tlnpi_provider: false }, mmio_megabytes: 0, user_defined: UserDefinedData([040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000]), subchannel_index: 0, mmio_megabytes_optional: 0, channel_id: ChannelId(5), monitor_id: ff, monitor_allocated: 0, is_dedicated: 1, connection_id: 2005 }, guest_to_host_interrupt: Interrupt { inner: InterruptInner }, request_send: SenderCore(Ma
[0.213793] vmbus_relay_intercept_device: nuallyDrop { value: MaybeDangling(Queue { remote: OnceLock(<uninit>), local: Mutex { data: LocalQueue { messages: ErasedVecDeque { cap: 0, head: 0, len: 0 }, ports: [], waker: Some(Waker { data: 0x240840e0270, vtable: 0x7fc22cc5e698 }), remote: false, receiver_gone: false, remove_closed: false, new_handler: 0x7fc22bd605c0 } }, receiver: UnsafeCell { .. } }) }), revoke_recv: ManuallyDrop { value: MaybeDangling(OneshotReceiverCore(Slot { state: Mutex { data: Waiting(None) }, receiver: UnsafeCell { .. } })) } }
[0.214025] vmbus_client: INFO  opening channel on host channel_id=0x5 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[0.216183] underhill_core::loader: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:load_firmware:  loading UEFI into VTL0
[0.235802] hyperv_ic_guest::shutdown: INFO  version negotiated framework_version=3.0 message_version=3.2
[0.236375] underhill_core: INFO  vm worker started
[0.237627] state_unit: INFO state_change{ operation="start"}:  state change complete duration=1.223509ms
[0.241231] underhill_core::dispatch: INFO  starting VM boot_time_ms=0x133 boot_time="307.1824ms"
[0.241405] firmware_uefi: INFO  set gpa for diagnostics addr=2b data=0x76000
[0.241617] firmware_uefi: INFO  set gpa for diagnostics addr=2b data=0x437d000
[0.247409] vmbus_server::channels: INFO  client released channel offer_id=OfferId(0) key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0
[0.247431] vmbus_server::channels: INFO  client released channel offer_id=OfferId(1) key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0
[0.247449] vmbus_server::channels: INFO  client released channel offer_id=OfferId(2) key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0
[0.247464] vmbus_server::channels: INFO  client released channel offer_id=OfferId(3) key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[0.247482] vmbus_server::channels: INFO  client released channel offer_id=OfferId(4) key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0
[0.247501] vmbus_server::channels: INFO  client released channel offer_id=OfferId(5) key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0
[0.247521] vmbus_server::channels: INFO  client released channel offer_id=OfferId(6) key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0
[0.247557] vmbus_server::channels: INFO  client released channel offer_id=OfferId(7) key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0
[0.247591] vmbus_server::channels: INFO  client released channel offer_id=OfferId(8) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[0.247616] vmbus_server::channels: INFO  client released channel dropped_ratelimited=0x0 offer_id=OfferId(9) key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[0.247622] vmbus_server::channels: INFO  guest requested unsupported feature flags. supported=0x0 requested=0x8
[0.247736] vmbus_server::channels: INFO  guest negotiated version vtl=0x0 version=VersionInfo { version: Copper, feature_flags: FeatureFlags { guest_specified_signal_parameters: false, channel_interrupt_redirection: false, modify_connection: false, client_id: false, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: false } } client_id=18dd3964-3e8a-4e42-86fa-c8e6b191ee0e trusted=false
[0.247792] vmbus_server::channels: INFO  sending offer to guest channel_id=0x1 connection_id=0x2001 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[0.247798] vmbus_server::channels: INFO  sending offer to guest channel_id=0x2 connection_id=0x2002 key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0
[0.247804] vmbus_server::channels: INFO  sending offer to guest channel_id=0x3 connection_id=0x2003 key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0
[0.247809] vmbus_server::channels: INFO  sending offer to guest channel_id=0x4 connection_id=0x2004 key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0
[0.247815] vmbus_server::channels: INFO  sending offer to guest channel_id=0x5 connection_id=0x2005 key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0
[0.247822] vmbus_server::channels: INFO  sending offer to guest channel_id=0x6 connection_id=0x2006 key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0
[0.247828] vmbus_server::channels: INFO  sending offer to guest channel_id=0x7 connection_id=0x2007 key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0
[0.247834] vmbus_server::channels: INFO  sending offer to guest channel_id=0x8 connection_id=0x2008 key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[0.247840] vmbus_server::channels: INFO  sending offer to guest channel_id=0x9 connection_id=0x2009 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0
[0.247858] vmbus_server::channels: INFO  sending offer to guest channel_id=0xa connection_id=0x200a key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[0.261520] vmbus_client: INFO  opening channel on host channel_id=0x9 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0
[0.261965] vmbus_server::channels: INFO  opened channel offer_id=0x7 channel_id=0x9 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0 result=0
[0.294892] vmbus_client: INFO  opening channel on host channel_id=0xa key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[0.295373] vmbus_server::channels: INFO  opened channel offer_id=0x8 channel_id=0xa key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0 result=0
[0.296418] vmbus_client: INFO  closing channel on host channel_id=0xa key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[0.296435] vmbus_server::channels: INFO  closed channel offer_id=0x8 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[0.297699] vmbus_client: INFO  opening channel on host channel_id=0xa key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[0.298116] vmbus_server::channels: INFO  opened channel offer_id=0x8 channel_id=0xa key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0 result=0
[10.650571] firmware_uefi::service::event_log: INFO  uefi boot: success
[10.650622] firmware_uefi::service::nvram: INFO  Revoking guest vsm
[10.650692] vmbus_server::channels: INFO  client released channel offer_id=OfferId(0) key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0
[10.650701] vmbus_server::channels: INFO  client released channel offer_id=OfferId(1) key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0
[10.650707] vmbus_server::channels: INFO  client released channel offer_id=OfferId(2) key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0
[10.650714] vmbus_server::channels: INFO  client released channel offer_id=OfferId(3) key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[10.650721] vmbus_server::channels: INFO  client released channel offer_id=OfferId(4) key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0
[10.650727] vmbus_server::channels: INFO  client released channel offer_id=OfferId(5) key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0
[10.650734] vmbus_server::channels: INFO  client released channel offer_id=OfferId(6) key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0
[10.650740] vmbus_server::channels: INFO  client released channel offer_id=OfferId(7) key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0
[10.650753] vmbus_server::channels: INFO  client released channel offer_id=OfferId(8) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[10.650764] vmbus_server::channels: INFO  client released channel dropped_ratelimited=0x0 offer_id=OfferId(9) key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[10.650809] vmbus_client: INFO  closing channel on host channel_id=0x9 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0
[10.650827] vmbus_client: INFO  closing channel on host channel_id=0xa key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[10.650842] vmbus_server::channels: INFO  closed channel offer_id=0x7 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0
[10.650849] vmbus_server::channels: INFO  closed channel offer_id=0x8 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[10.651538] vmbus_server::channels: INFO  Vmbus disconnected
[10.651696] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x2efef6 phase=PEI_CORE log_message="PeiDelayedDispatchOnEndOfPei Count of dispatch cycles is 0"
[10.651722] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x2fe4cc phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (0E0B6031-5213-4934-818B-38D90CED39DB)."
[10.651729] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x2fe5a9 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (276AACF4-AC15-426C-98DD-7521AD3F01FE)."
[10.651734] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x2fe681 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (3375BAF4-9E15-4B30-B765-67ACB10D607B)."
[10.651740] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x2fe756 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (35FA2E29-EA23-4236-96AE-3A6EBACBA440)."
[10.651745] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x2fe82e phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (525074DC-8985-46E2-8057-A307DC18A502)."
[10.651750] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x2fe906 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (57164F39-9115-4E78-AB55-382F3BD5422D)."
[10.651755] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x2fe9e1 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (9527E630-D0AE-497B-ADCE-E80AB0175CAF)."
[10.651760] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x2feab8 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (A9A0F4E7-5A45-4D96-B827-8A841E8C03E6)."
[10.651768] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x2ffb62 phase=DXE log_message="FPDT: WARNING: SEC Performance Data Hob not found, ResetEnd will be set to 0!"
[10.651775] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x2fff52 phase=DXE log_message="IoMmuLibInit: IoMmuProtocol revision 0x10001 does not support SetAttributeById. IoMmuSetAttributeById will return EFI_UNSUPPORTED."
[10.651781] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x2fff7c phase=DXE log_message="Error: Image at 000F7E15000 start failed: Unsupported"
[10.651790] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x3032e3 phase=DXE log_message="Error: Image at 000F7C60000 start failed: Unsupported"
[10.651798] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x304372 phase=DXE log_message="IoMmuLibInit: IoMmuProtocol revision 0x10001 does not support SetAttributeById. IoMmuSetAttributeById will return EFI_UNSUPPORTED."
[10.651811] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x3075ba phase=DXE log_message="IoMmuLibInit: IoMmuProtocol revision 0x10001 does not support SetAttributeById. IoMmuSetAttributeById will return EFI_UNSUPPORTED."
[10.651816] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x3075e1 phase=DXE log_message="Error: Image at 000F7BA2000 start failed: Unsupported"
[10.651830] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x31d353 phase=DXE log_message="Handle for Hyper-V keyboard device not found"
[10.651834] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x31d361 phase=DXE log_message="Handle for Hyper-V video device not found"
[10.651843] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x3772d0 phase=DXE log_message="MnpStart: MnpStartSnp failed, Already started."
[10.651851] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x37f6d2 phase=DXE log_message="OnReadyToBoot: Transitioning to boot manager; errors past this point and before ExitBootServices suggest a boot manager issue."
[10.651859] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x4389ba phase=DXE log_message="ActivateCompatibilityMode - Activating Memory Protection Compatibility Mode!"
[10.651864] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x52dd33 phase=DXE log_message="MnpSyncSendPacket: No network cable detected."
[10.651869] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0xb27ccb phase=DXE log_message="MnpSyncSendPacket: No network cable detected."
[10.651874] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x62e4bd3 phase=DXE log_message="MnpSyncSendPacket: No network cable detected."
[10.651882] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x6633806 phase=RUNTIME log_message="SecurityLock::LockType: SOFTWARE_LOCK, Module: 42857F0A-13F2-4B21-8A23-53D3F714B840, Function: LockCapsuleInterface, Output: Lock Capsule Interface"
[10.651888] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x6633972 phase=DXE log_message="VmbusRootExitBootServices (1374) GPADL 0x1 not cleaned up."
[10.651892] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x6633983 phase=DXE log_message="VmbusRootExitBootServices (1374) GPADL 0x2 not cleaned up."
[10.651896] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x6633992 phase=DXE log_message="VmbusRootExitBootServices (1374) GPADL 0x3 not cleaned up."
[10.651901] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x66339a0 phase=DXE log_message="VmbusRootExitBootServices (1374) GPADL 0x4 not cleaned up."
[10.651906] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x66339dd phase=DXE log_message="VmbusRootExitBootServices (1383) orphaned 4 GPADLs (IsolationArchitecture=0)"
[10.651912] firmware_uefi::service::diagnostics::processor: WARN  suppressed logs substring="ConvertPages: range" count=0x2
[10.651917] firmware_uefi::service::diagnostics::processor: INFO  processed EFI log entries entries_processed=0x4b1 entries_emitted=0x1e bytes_read=0x1bf30
[11.026940] virt_mshv_vtl::processor: INFO  sidecar exit
[11.026954] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.027482] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1
[11.028157] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1
[11.106168] mshv_vtl_sidecar 4098b4000.sidecar: cpu 1: removing from sidecar
[11.106246] mshv_vtl_sidecar 4098b4000.sidecar: cpu 1: remove complete
[11.106249] smpboot: Booting Node 0 Processor 1 APIC 0x1
[11.108087] virt_mshv_vtl::processor: INFO  sidecar exit
[11.108099] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.108430] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2
[11.109239] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2
[11.196124] mshv_vtl_sidecar 4098b4000.sidecar: cpu 2: removing from sidecar
[11.196202] mshv_vtl_sidecar 4098b4000.sidecar: cpu 2: remove complete
[11.196205] smpboot: Booting Node 0 Processor 2 APIC 0x2
[11.197503] virt_mshv_vtl::processor: INFO  sidecar exit
[11.197513] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.198030] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3
[11.198583] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3
[11.306179] mshv_vtl_sidecar 4098b4000.sidecar: cpu 3: removing from sidecar
[11.306255] mshv_vtl_sidecar 4098b4000.sidecar: cpu 3: remove complete
[11.306260] smpboot: Booting Node 0 Processor 3 APIC 0x3
[11.307554] virt_mshv_vtl::processor: INFO  sidecar exit
[11.307564] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.308036] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x4
[11.308650] underhill_core::vp: INFO  onlining sidecar VP cpu=0x4
[11.466191] mshv_vtl_sidecar 4098b4000.sidecar: cpu 4: removing from sidecar
[11.466270] mshv_vtl_sidecar 4098b4000.sidecar: cpu 4: remove complete
[11.466276] smpboot: Booting Node 0 Processor 4 APIC 0x4
[11.467543] virt_mshv_vtl::processor: INFO  sidecar exit
[11.467552] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.467876] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x5
[11.468396] underhill_core::vp: INFO  onlining sidecar VP cpu=0x5
[11.546189] mshv_vtl_sidecar 4098b4000.sidecar: cpu 5: removing from sidecar
[11.546265] mshv_vtl_sidecar 4098b4000.sidecar: cpu 5: remove complete
[11.546270] smpboot: Booting Node 0 Processor 5 APIC 0x5
[11.547591] virt_mshv_vtl::processor: INFO  sidecar exit
[11.547601] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.547894] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x6
[11.548413] underhill_core::vp: INFO  onlining sidecar VP cpu=0x6
[11.548755] mshv_vtl_sidecar 4098b4000.sidecar: cpu 6: removing from sidecar
[11.548831] mshv_vtl_sidecar 4098b4000.sidecar: cpu 6: remove complete
[11.548835] smpboot: Booting Node 0 Processor 6 APIC 0x6
[11.550019] virt_mshv_vtl::processor: INFO  sidecar exit
[11.550027] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.550276] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x7
[11.550791] underhill_core::vp: INFO  onlining sidecar VP cpu=0x7
[11.551124] mshv_vtl_sidecar 4098b4000.sidecar: cpu 7: removing from sidecar
[11.551200] mshv_vtl_sidecar 4098b4000.sidecar: cpu 7: remove complete
[11.551204] smpboot: Booting Node 0 Processor 7 APIC 0x7
[11.552349] virt_mshv_vtl::processor: INFO  sidecar exit
[11.552356] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.552612] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x8
[11.553122] underhill_core::vp: INFO  onlining sidecar VP cpu=0x8
[11.553469] mshv_vtl_sidecar 4098b4000.sidecar: cpu 8: removing from sidecar
[11.553545] mshv_vtl_sidecar 4098b4000.sidecar: cpu 8: remove complete
[11.553549] smpboot: Booting Node 0 Processor 8 APIC 0x8
[11.554791] virt_mshv_vtl::processor: INFO  sidecar exit
[11.554797] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.555257] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x9
[11.555800] underhill_core::vp: INFO  onlining sidecar VP cpu=0x9
[11.556229] mshv_vtl_sidecar 4098b4000.sidecar: cpu 9: removing from sidecar
[11.556306] mshv_vtl_sidecar 4098b4000.sidecar: cpu 9: remove complete
[11.556315] smpboot: Booting Node 0 Processor 9 APIC 0x9
[11.557496] virt_mshv_vtl::processor: INFO  sidecar exit
[11.557505] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.557755] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xa
[11.558273] underhill_core::vp: INFO  onlining sidecar VP cpu=0xa
[11.558609] mshv_vtl_sidecar 4098b4000.sidecar: cpu 10: removing from sidecar
[11.558685] mshv_vtl_sidecar 4098b4000.sidecar: cpu 10: remove complete
[11.558689] smpboot: Booting Node 0 Processor 10 APIC 0xa
[11.559861] virt_mshv_vtl::processor: INFO  sidecar exit
[11.559868] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.560108] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xb
[11.560620] underhill_core::vp: INFO  onlining sidecar VP cpu=0xb
[11.560952] mshv_vtl_sidecar 4098b4000.sidecar: cpu 11: removing from sidecar
[11.561029] mshv_vtl_sidecar 4098b4000.sidecar: cpu 11: remove complete
[11.561037] smpboot: Booting Node 0 Processor 11 APIC 0xb
[11.562260] virt_mshv_vtl::processor: INFO  sidecar exit
[11.562267] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.562506] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xc
[11.563021] underhill_core::vp: INFO  onlining sidecar VP cpu=0xc
[11.563347] mshv_vtl_sidecar 4098b4000.sidecar: cpu 12: removing from sidecar
[11.563423] mshv_vtl_sidecar 4098b4000.sidecar: cpu 12: remove complete
[11.563427] smpboot: Booting Node 0 Processor 12 APIC 0xc
[11.564634] virt_mshv_vtl::processor: INFO  sidecar exit
[11.564643] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.564912] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xd
[11.565430] underhill_core::vp: INFO  onlining sidecar VP cpu=0xd
[11.565766] mshv_vtl_sidecar 4098b4000.sidecar: cpu 13: removing from sidecar
[11.565842] mshv_vtl_sidecar 4098b4000.sidecar: cpu 13: remove complete
[11.565846] smpboot: Booting Node 0 Processor 13 APIC 0xd
[11.567026] virt_mshv_vtl::processor: INFO  sidecar exit
[11.567034] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.567275] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xe
[11.567787] underhill_core::vp: INFO  onlining sidecar VP cpu=0xe
[11.568193] mshv_vtl_sidecar 4098b4000.sidecar: cpu 14: removing from sidecar
[11.568270] mshv_vtl_sidecar 4098b4000.sidecar: cpu 14: remove complete
[11.568278] smpboot: Booting Node 0 Processor 14 APIC 0xe
[11.569537] virt_mshv_vtl::processor: INFO  sidecar exit
[11.569544] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.569765] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xf
[11.570281] underhill_core::vp: INFO  onlining sidecar VP cpu=0xf
[11.570617] mshv_vtl_sidecar 4098b4000.sidecar: cpu 15: removing from sidecar
[11.570693] mshv_vtl_sidecar 4098b4000.sidecar: cpu 15: remove complete
[11.570697] smpboot: Booting Node 0 Processor 15 APIC 0xf
[11.571859] virt_mshv_vtl::processor: INFO  sidecar exit
[11.571866] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.572118] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x10
[11.572633] underhill_core::vp: INFO  onlining sidecar VP cpu=0x10
[11.573000] mshv_vtl_sidecar 4098b4000.sidecar: cpu 16: removing from sidecar
[11.573076] mshv_vtl_sidecar 4098b4000.sidecar: cpu 16: remove complete
[11.573080] smpboot: Booting Node 0 Processor 16 APIC 0x10
[11.574280] virt_mshv_vtl::processor: INFO  sidecar exit
[11.574287] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.574522] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x11
[11.575036] underhill_core::vp: INFO  onlining sidecar VP cpu=0x11
[11.575364] mshv_vtl_sidecar 4098b4000.sidecar: cpu 17: removing from sidecar
[11.575440] mshv_vtl_sidecar 4098b4000.sidecar: cpu 17: remove complete
[11.575444] smpboot: Booting Node 0 Processor 17 APIC 0x11
[11.576723] virt_mshv_vtl::processor: INFO  sidecar exit
[11.576731] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.576976] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x12
[11.577490] underhill_core::vp: INFO  onlining sidecar VP cpu=0x12
[11.577880] mshv_vtl_sidecar 4098b4000.sidecar: cpu 18: removing from sidecar
[11.577956] mshv_vtl_sidecar 4098b4000.sidecar: cpu 18: remove complete
[11.577959] smpboot: Booting Node 0 Processor 18 APIC 0x12
[11.579165] virt_mshv_vtl::processor: INFO  sidecar exit
[11.579172] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.579402] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x13
[11.579912] underhill_core::vp: INFO  onlining sidecar VP cpu=0x13
[11.580240] mshv_vtl_sidecar 4098b4000.sidecar: cpu 19: removing from sidecar
[11.580315] mshv_vtl_sidecar 4098b4000.sidecar: cpu 19: remove complete
[11.580319] smpboot: Booting Node 0 Processor 19 APIC 0x13
[11.581479] virt_mshv_vtl::processor: INFO  sidecar exit
[11.581486] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.581717] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x14
[11.582232] underhill_core::vp: INFO  onlining sidecar VP cpu=0x14
[11.582561] mshv_vtl_sidecar 4098b4000.sidecar: cpu 20: removing from sidecar
[11.582637] mshv_vtl_sidecar 4098b4000.sidecar: cpu 20: remove complete
[11.582640] smpboot: Booting Node 0 Processor 20 APIC 0x14
[11.583851] virt_mshv_vtl::processor: INFO  sidecar exit
[11.583857] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.584073] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x15
[11.584588] underhill_core::vp: INFO  onlining sidecar VP cpu=0x15
[11.584924] mshv_vtl_sidecar 4098b4000.sidecar: cpu 21: removing from sidecar
[11.584999] mshv_vtl_sidecar 4098b4000.sidecar: cpu 21: remove complete
[11.585003] smpboot: Booting Node 0 Processor 21 APIC 0x15
[11.586287] virt_mshv_vtl::processor: INFO  sidecar exit
[11.586294] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.586549] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x16
[11.587062] underhill_core::vp: INFO  onlining sidecar VP cpu=0x16
[11.587398] mshv_vtl_sidecar 4098b4000.sidecar: cpu 22: removing from sidecar
[11.587473] mshv_vtl_sidecar 4098b4000.sidecar: cpu 22: remove complete
[11.587477] smpboot: Booting Node 0 Processor 22 APIC 0x16
[11.588694] virt_mshv_vtl::processor: INFO  sidecar exit
[11.588700] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.588925] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x17
[11.589438] underhill_core::vp: INFO  onlining sidecar VP cpu=0x17
[11.589741] mshv_vtl_sidecar 4098b4000.sidecar: cpu 23: removing from sidecar
[11.589817] mshv_vtl_sidecar 4098b4000.sidecar: cpu 23: remove complete
[11.589820] smpboot: Booting Node 0 Processor 23 APIC 0x17
[11.591001] virt_mshv_vtl::processor: INFO  sidecar exit
[11.591008] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.591244] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x18
[11.591758] underhill_core::vp: INFO  onlining sidecar VP cpu=0x18
[11.592162] mshv_vtl_sidecar 4098b4000.sidecar: cpu 24: removing from sidecar
[11.592238] mshv_vtl_sidecar 4098b4000.sidecar: cpu 24: remove complete
[11.592246] smpboot: Booting Node 0 Processor 24 APIC 0x18
[11.593537] virt_mshv_vtl::processor: INFO  sidecar exit
[11.593543] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.593781] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x19
[11.594293] underhill_core::vp: INFO  onlining sidecar VP cpu=0x19
[11.594655] mshv_vtl_sidecar 4098b4000.sidecar: cpu 25: removing from sidecar
[11.594730] mshv_vtl_sidecar 4098b4000.sidecar: cpu 25: remove complete
[11.594735] smpboot: Booting Node 0 Processor 25 APIC 0x19
[11.596114] virt_mshv_vtl::processor: INFO  sidecar exit
[11.596124] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.596383] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1a
[11.596900] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1a
[11.597240] mshv_vtl_sidecar 4098b4000.sidecar: cpu 26: removing from sidecar
[11.597315] mshv_vtl_sidecar 4098b4000.sidecar: cpu 26: remove complete
[11.597320] smpboot: Booting Node 0 Processor 26 APIC 0x1a
[11.598870] virt_mshv_vtl::processor: INFO  sidecar exit
[11.598879] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.599133] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1b
[11.599657] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1b
[11.599991] mshv_vtl_sidecar 4098b4000.sidecar: cpu 27: removing from sidecar
[11.600067] mshv_vtl_sidecar 4098b4000.sidecar: cpu 27: remove complete
[11.600071] smpboot: Booting Node 0 Processor 27 APIC 0x1b
[11.601355] virt_mshv_vtl::processor: INFO  sidecar exit
[11.601362] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.601611] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1c
[11.602124] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1c
[11.602490] mshv_vtl_sidecar 4098b4000.sidecar: cpu 28: removing from sidecar
[11.602566] mshv_vtl_sidecar 4098b4000.sidecar: cpu 28: remove complete
[11.602571] smpboot: Booting Node 0 Processor 28 APIC 0x1c
[11.603847] virt_mshv_vtl::processor: INFO  sidecar exit
[11.603853] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.604132] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1d
[11.604920] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1d
[11.605264] mshv_vtl_sidecar 4098b4000.sidecar: cpu 29: removing from sidecar
[11.605339] mshv_vtl_sidecar 4098b4000.sidecar: cpu 29: remove complete
[11.605343] smpboot: Booting Node 0 Processor 29 APIC 0x1d
[11.606603] virt_mshv_vtl::processor: INFO  sidecar exit
[11.606611] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.606918] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1e
[11.607426] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1e
[11.607796] mshv_vtl_sidecar 4098b4000.sidecar: cpu 30: removing from sidecar
[11.615928] mshv_vtl_sidecar 4098b4000.sidecar: cpu 30: remove complete
[11.615935] smpboot: Booting Node 0 Processor 30 APIC 0x1e
[11.617374] virt_mshv_vtl::processor: INFO  sidecar exit
[11.617383] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.617661] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1f
[11.618178] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1f
[11.618519] mshv_vtl_sidecar 4098b4000.sidecar: cpu 31: removing from sidecar
[11.618594] mshv_vtl_sidecar 4098b4000.sidecar: cpu 31: remove complete
[11.618599] smpboot: Booting Node 0 Processor 31 APIC 0x1f
[11.620263] virt_mshv_vtl::processor: INFO  sidecar exit
[11.620273] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.620568] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x21
[11.621091] underhill_core::vp: INFO  onlining sidecar VP cpu=0x21
[11.621430] mshv_vtl_sidecar 409a55000.sidecar: cpu 33: removing from sidecar
[11.621506] mshv_vtl_sidecar 409a55000.sidecar: cpu 33: remove complete
[11.621510] smpboot: Booting Node 0 Processor 33 APIC 0x21
[11.622883] virt_mshv_vtl::processor: INFO  sidecar exit
[11.622892] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.623190] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x22
[11.623712] underhill_core::vp: INFO  onlining sidecar VP cpu=0x22
[11.624080] mshv_vtl_sidecar 409a55000.sidecar: cpu 34: removing from sidecar
[11.624157] mshv_vtl_sidecar 409a55000.sidecar: cpu 34: remove complete
[11.624160] smpboot: Booting Node 0 Processor 34 APIC 0x22
[11.625466] virt_mshv_vtl::processor: INFO  sidecar exit
[11.625473] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.625741] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x23
[11.626254] underhill_core::vp: INFO  onlining sidecar VP cpu=0x23
[11.626621] mshv_vtl_sidecar 409a55000.sidecar: cpu 35: removing from sidecar
[11.626700] mshv_vtl_sidecar 409a55000.sidecar: cpu 35: remove complete
[11.626707] smpboot: Booting Node 0 Processor 35 APIC 0x23
[11.628032] virt_mshv_vtl::processor: INFO  sidecar exit
[11.628055] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.628309] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x24
[11.628821] underhill_core::vp: INFO  onlining sidecar VP cpu=0x24
[11.629181] mshv_vtl_sidecar 409a55000.sidecar: cpu 36: removing from sidecar
[11.629257] mshv_vtl_sidecar 409a55000.sidecar: cpu 36: remove complete
[11.629263] smpboot: Booting Node 0 Processor 36 APIC 0x24
[11.630621] virt_mshv_vtl::processor: INFO  sidecar exit
[11.630628] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.630892] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x25
[11.631405] underhill_core::vp: INFO  onlining sidecar VP cpu=0x25
[11.716217] mshv_vtl_sidecar 409a55000.sidecar: cpu 37: removing from sidecar
[11.716293] mshv_vtl_sidecar 409a55000.sidecar: cpu 37: remove complete
[11.716298] smpboot: Booting Node 0 Processor 37 APIC 0x25
[11.717637] virt_mshv_vtl::processor: INFO  sidecar exit
[11.717648] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.717947] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x26
[11.718452] underhill_core::vp: INFO  onlining sidecar VP cpu=0x26
[11.718881] mshv_vtl_sidecar 409a55000.sidecar: cpu 38: removing from sidecar
[11.718959] mshv_vtl_sidecar 409a55000.sidecar: cpu 38: remove complete
[11.718967] smpboot: Booting Node 0 Processor 38 APIC 0x26
[11.720347] virt_mshv_vtl::processor: INFO  sidecar exit
[11.720358] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.720607] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x27
[11.721121] underhill_core::vp: INFO  onlining sidecar VP cpu=0x27
[11.721522] mshv_vtl_sidecar 409a55000.sidecar: cpu 39: removing from sidecar
[11.721598] mshv_vtl_sidecar 409a55000.sidecar: cpu 39: remove complete
[11.721603] smpboot: Booting Node 0 Processor 39 APIC 0x27
[11.722975] virt_mshv_vtl::processor: INFO  sidecar exit
[11.722983] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.723241] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x28
[11.723750] underhill_core::vp: INFO  onlining sidecar VP cpu=0x28
[11.724090] mshv_vtl_sidecar 409a55000.sidecar: cpu 40: removing from sidecar
[11.724167] mshv_vtl_sidecar 409a55000.sidecar: cpu 40: remove complete
[11.724174] smpboot: Booting Node 0 Processor 40 APIC 0x28
[11.725533] virt_mshv_vtl::processor: INFO  sidecar exit
[11.725541] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.725790] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x29
[11.726303] underhill_core::vp: INFO  onlining sidecar VP cpu=0x29
[11.726623] mshv_vtl_sidecar 409a55000.sidecar: cpu 41: removing from sidecar
[11.726700] mshv_vtl_sidecar 409a55000.sidecar: cpu 41: remove complete
[11.726708] smpboot: Booting Node 0 Processor 41 APIC 0x29
[11.728026] virt_mshv_vtl::processor: INFO  sidecar exit
[11.728034] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.728278] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2a
[11.728788] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2a
[11.729195] mshv_vtl_sidecar 409a55000.sidecar: cpu 42: removing from sidecar
[11.729271] mshv_vtl_sidecar 409a55000.sidecar: cpu 42: remove complete
[11.729276] smpboot: Booting Node 0 Processor 42 APIC 0x2a
[11.730636] virt_mshv_vtl::processor: INFO  sidecar exit
[11.730644] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.730872] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2b
[11.731383] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2b
[11.731755] mshv_vtl_sidecar 409a55000.sidecar: cpu 43: removing from sidecar
[11.731835] mshv_vtl_sidecar 409a55000.sidecar: cpu 43: remove complete
[11.731842] smpboot: Booting Node 0 Processor 43 APIC 0x2b
[11.733256] virt_mshv_vtl::processor: INFO  sidecar exit
[11.733263] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.733543] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2c
[11.734048] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2c
[11.734378] mshv_vtl_sidecar 409a55000.sidecar: cpu 44: removing from sidecar
[11.734456] mshv_vtl_sidecar 409a55000.sidecar: cpu 44: remove complete
[11.734461] smpboot: Booting Node 0 Processor 44 APIC 0x2c
[11.735862] virt_mshv_vtl::processor: INFO  sidecar exit
[11.735869] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.736216] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2d
[11.736723] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2d
[11.737114] mshv_vtl_sidecar 409a55000.sidecar: cpu 45: removing from sidecar
[11.737191] mshv_vtl_sidecar 409a55000.sidecar: cpu 45: remove complete
[11.737197] smpboot: Booting Node 0 Processor 45 APIC 0x2d
[11.738670] virt_mshv_vtl::processor: INFO  sidecar exit
[11.738678] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.738977] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2e
[11.739787] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2e
[11.740154] mshv_vtl_sidecar 409a55000.sidecar: cpu 46: removing from sidecar
[11.740231] mshv_vtl_sidecar 409a55000.sidecar: cpu 46: remove complete
[11.740237] smpboot: Booting Node 0 Processor 46 APIC 0x2e
[11.741687] virt_mshv_vtl::processor: INFO  sidecar exit
[11.741695] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.741977] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2f
[11.742480] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2f
[11.742834] mshv_vtl_sidecar 409a55000.sidecar: cpu 47: removing from sidecar
[11.742909] mshv_vtl_sidecar 409a55000.sidecar: cpu 47: remove complete
[11.742914] smpboot: Booting Node 0 Processor 47 APIC 0x2f
[11.744271] virt_mshv_vtl::processor: INFO  sidecar exit
[11.744279] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.744512] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x30
[11.745028] underhill_core::vp: INFO  onlining sidecar VP cpu=0x30
[11.745418] mshv_vtl_sidecar 409a55000.sidecar: cpu 48: removing from sidecar
[11.745493] mshv_vtl_sidecar 409a55000.sidecar: cpu 48: remove complete
[11.745498] smpboot: Booting Node 0 Processor 48 APIC 0x30
[11.746894] virt_mshv_vtl::processor: INFO  sidecar exit
[11.746902] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.747151] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x31
[11.747665] underhill_core::vp: INFO  onlining sidecar VP cpu=0x31
[11.748092] mshv_vtl_sidecar 409a55000.sidecar: cpu 49: removing from sidecar
[11.748168] mshv_vtl_sidecar 409a55000.sidecar: cpu 49: remove complete
[11.748176] smpboot: Booting Node 0 Processor 49 APIC 0x31
[11.749648] virt_mshv_vtl::processor: INFO  sidecar exit
[11.749656] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.749900] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x32
[11.750410] underhill_core::vp: INFO  onlining sidecar VP cpu=0x32
[11.750740] mshv_vtl_sidecar 409a55000.sidecar: cpu 50: removing from sidecar
[11.750816] mshv_vtl_sidecar 409a55000.sidecar: cpu 50: remove complete
[11.750822] smpboot: Booting Node 0 Processor 50 APIC 0x32
[11.752230] virt_mshv_vtl::processor: INFO  sidecar exit
[11.752238] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.752479] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x33
[11.752988] underhill_core::vp: INFO  onlining sidecar VP cpu=0x33
[11.753326] mshv_vtl_sidecar 409a55000.sidecar: cpu 51: removing from sidecar
[11.753404] mshv_vtl_sidecar 409a55000.sidecar: cpu 51: remove complete
[11.753414] smpboot: Booting Node 0 Processor 51 APIC 0x33
[11.754804] virt_mshv_vtl::processor: INFO  sidecar exit
[11.754811] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.755047] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x34
[11.755564] underhill_core::vp: INFO  onlining sidecar VP cpu=0x34
[11.755940] mshv_vtl_sidecar 409a55000.sidecar: cpu 52: removing from sidecar
[11.756018] mshv_vtl_sidecar 409a55000.sidecar: cpu 52: remove complete
[11.756024] smpboot: Booting Node 0 Processor 52 APIC 0x34
[11.757527] virt_mshv_vtl::processor: INFO  sidecar exit
[11.757536] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.757778] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x35
[11.758292] underhill_core::vp: INFO  onlining sidecar VP cpu=0x35
[11.758605] mshv_vtl_sidecar 409a55000.sidecar: cpu 53: removing from sidecar
[11.758681] mshv_vtl_sidecar 409a55000.sidecar: cpu 53: remove complete
[11.758686] smpboot: Booting Node 0 Processor 53 APIC 0x35
[11.760103] virt_mshv_vtl::processor: INFO  sidecar exit
[11.760110] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.760351] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x36
[11.760863] underhill_core::vp: INFO  onlining sidecar VP cpu=0x36
[11.761179] mshv_vtl_sidecar 409a55000.sidecar: cpu 54: removing from sidecar
[11.761255] mshv_vtl_sidecar 409a55000.sidecar: cpu 54: remove complete
[11.761260] smpboot: Booting Node 0 Processor 54 APIC 0x36
[11.762680] virt_mshv_vtl::processor: INFO  sidecar exit
[11.762688] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.762919] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x37
[11.763431] underhill_core::vp: INFO  onlining sidecar VP cpu=0x37
[11.763772] mshv_vtl_sidecar 409a55000.sidecar: cpu 55: removing from sidecar
[11.763849] mshv_vtl_sidecar 409a55000.sidecar: cpu 55: remove complete
[11.763854] smpboot: Booting Node 0 Processor 55 APIC 0x37
[11.765308] virt_mshv_vtl::processor: INFO  sidecar exit
[11.765315] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.765561] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x38
[11.766074] underhill_core::vp: INFO  onlining sidecar VP cpu=0x38
[11.766414] mshv_vtl_sidecar 409a55000.sidecar: cpu 56: removing from sidecar
[11.766490] mshv_vtl_sidecar 409a55000.sidecar: cpu 56: remove complete
[11.766496] smpboot: Booting Node 0 Processor 56 APIC 0x38
[11.767986] virt_mshv_vtl::processor: INFO  sidecar exit
[11.767994] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.768240] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x39
[11.768751] underhill_core::vp: INFO  onlining sidecar VP cpu=0x39
[11.769098] mshv_vtl_sidecar 409a55000.sidecar: cpu 57: removing from sidecar
[11.769175] mshv_vtl_sidecar 409a55000.sidecar: cpu 57: remove complete
[11.769180] smpboot: Booting Node 0 Processor 57 APIC 0x39
[11.770648] virt_mshv_vtl::processor: INFO  sidecar exit
[11.770656] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.770892] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3a
[11.771410] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3a
[11.771773] mshv_vtl_sidecar 409a55000.sidecar: cpu 58: removing from sidecar
[11.771849] mshv_vtl_sidecar 409a55000.sidecar: cpu 58: remove complete
[11.771855] smpboot: Booting Node 0 Processor 58 APIC 0x3a
[11.773360] virt_mshv_vtl::processor: INFO  sidecar exit
[11.773368] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.773617] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3b
[11.774128] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3b
[11.774442] mshv_vtl_sidecar 409a55000.sidecar: cpu 59: removing from sidecar
[11.774518] mshv_vtl_sidecar 409a55000.sidecar: cpu 59: remove complete
[11.774523] smpboot: Booting Node 0 Processor 59 APIC 0x3b
[11.775958] virt_mshv_vtl::processor: INFO  sidecar exit
[11.775966] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.776221] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3c
[11.776730] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3c
[11.777077] mshv_vtl_sidecar 409a55000.sidecar: cpu 60: removing from sidecar
[11.777153] mshv_vtl_sidecar 409a55000.sidecar: cpu 60: remove complete
[11.777159] smpboot: Booting Node 0 Processor 60 APIC 0x3c
[11.778715] virt_mshv_vtl::processor: INFO  sidecar exit
[11.778723] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.778969] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3d
[11.779477] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3d
[11.779823] mshv_vtl_sidecar 409a55000.sidecar: cpu 61: removing from sidecar
[11.779901] mshv_vtl_sidecar 409a55000.sidecar: cpu 61: remove complete
[11.779909] smpboot: Booting Node 0 Processor 61 APIC 0x3d
[11.781547] virt_mshv_vtl::processor: INFO  sidecar exit
[11.781554] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.781789] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3e
[11.782306] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3e
[11.782616] mshv_vtl_sidecar 409a55000.sidecar: cpu 62: removing from sidecar
[11.782691] mshv_vtl_sidecar 409a55000.sidecar: cpu 62: remove complete
[11.782697] smpboot: Booting Node 0 Processor 62 APIC 0x3e
[11.784407] virt_mshv_vtl::processor: INFO  sidecar exit
[11.784414] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[11.784660] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3f
[11.785171] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3f
[11.785467] mshv_vtl_sidecar 409a55000.sidecar: cpu 63: removing from sidecar
[11.785543] mshv_vtl_sidecar 409a55000.sidecar: cpu 63: remove complete
[11.785548] smpboot: Booting Node 0 Processor 63 APIC 0x3f
[11.850184] vmbus_server::channels: INFO  guest negotiated version vtl=0x0 version=VersionInfo { version: Iron, feature_flags: FeatureFlags { guest_specified_signal_parameters: false, channel_interrupt_redirection: false, modify_connection: false, client_id: false, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: false } } client_id=00000000-0000-0000-0000-000000000000 trusted=false
[11.850682] vmbus_server::channels: INFO  sending offer to guest channel_id=0x1 connection_id=0x2001 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[11.850691] vmbus_server::channels: INFO  sending offer to guest channel_id=0x2 connection_id=0x2002 key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0
[11.850698] vmbus_server::channels: INFO  sending offer to guest channel_id=0x3 connection_id=0x2003 key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0
[11.850705] vmbus_server::channels: INFO  sending offer to guest channel_id=0x4 connection_id=0x2004 key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0
[11.850711] vmbus_server::channels: INFO  sending offer to guest channel_id=0x5 connection_id=0x2005 key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0
[11.850717] vmbus_server::channels: INFO  sending offer to guest channel_id=0x6 connection_id=0x2006 key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0
[11.850724] vmbus_server::channels: INFO  sending offer to guest channel_id=0x7 connection_id=0x2007 key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0
[11.850730] vmbus_server::channels: INFO  sending offer to guest channel_id=0x8 connection_id=0x2008 key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[11.850736] vmbus_server::channels: INFO  sending offer to guest channel_id=0x9 connection_id=0x2009 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0
[11.850745] vmbus_server::channels: INFO  sending offer to guest channel_id=0xa connection_id=0x200a key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[12.013372] vmbus_client: INFO  opening channel on host channel_id=0x9 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0
[12.014004] vmbus_server::channels: INFO  opened channel offer_id=0x7 channel_id=0x9 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{443ce0be-fefb-4d2b-afdb-448da1809a1e}-0 result=0
[12.027250] vmbus_server::channels: INFO  opened channel offer_id=0x9 channel_id=0x1 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 result=0
[12.027306] vmbus_client: INFO  opening channel on host channel_id=0x7 key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0
[12.027325] vmbus_client: INFO  opening channel on host channel_id=0x3 key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0
[12.027341] vmbus_client: INFO  opening channel on host channel_id=0x6 key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0
[12.027354] vmbus_client: INFO  opening channel on host channel_id=0x4 key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[12.027806] vmbus_server::channels: INFO  opened channel offer_id=0x5 channel_id=0x4 key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 result=0
[12.027942] vmbus_server::channels: INFO  opened channel offer_id=0x2 channel_id=0x6 key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 result=0
[12.028211] vmbus_server::channels: INFO  opened channel offer_id=0x4 channel_id=0x7 key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 result=0
[12.028240] vmbus_server::channels: INFO  opened channel offer_id=0x3 channel_id=0x8 key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 result=0
[12.029589] vmbus_client: INFO  opening channel on host channel_id=0x1 key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0
[12.029965] vmbus_server::channels: INFO  opened channel offer_id=0x0 channel_id=0x5 key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 result=0
[12.128295] vmbus_server: INFO  waking guest for outgoing ring channel={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[12.288063] vmbus_client: INFO  opening channel on host channel_id=0xa key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0
[12.288825] vmbus_server::channels: INFO  opened channel offer_id=0x8 channel_id=0xa key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-0 result=0
[12.306327] vmbus_client: INFO  received offer state=Connected channel_id=0xb interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=61f0a39b-c778-405b-bc5b-ffca0b817e4b subchannel_index=0x1
[12.306346] vmbus_client: INFO  received offer state=Connected channel_id=0xc interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=61f0a39b-c778-405b-bc5b-ffca0b817e4b subchannel_index=0x2
[12.306378] vmbus_server::channels: INFO  sending offer to guest channel_id=0xb connection_id=0x200b key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-1
[12.306388] vmbus_server::channels: INFO  new channel offer_id=OfferId(a) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-1 confidential_ring_buffer=false confidential_external_memory=false
[12.306410] vmbus_server::channels: INFO  sending offer to guest channel_id=0xc connection_id=0x200c key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-2
[12.306424] vmbus_server::channels: INFO  new channel offer_id=OfferId(b) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-2 confidential_ring_buffer=false confidential_external_memory=false
[12.306459] vmbus_client: INFO  received offer state=Connected channel_id=0xd interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=61f0a39b-c778-405b-bc5b-ffca0b817e4b subchannel_index=0x3
[12.306471] vmbus_client: INFO  received offer state=Connected channel_id=0xe interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=61f0a39b-c778-405b-bc5b-ffca0b817e4b subchannel_index=0x4
[12.306490] vmbus_client: INFO  received offer state=Connected channel_id=0xf interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=61f0a39b-c778-405b-bc5b-ffca0b817e4b subchannel_index=0x5
[12.306499] vmbus_client: INFO  received offer state=Connected channel_id=0x10 interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=61f0a39b-c778-405b-bc5b-ffca0b817e4b subchannel_index=0x6
[12.306513] vmbus_server::channels: INFO  sending offer to guest channel_id=0xd connection_id=0x200d key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-3
[12.306520] vmbus_server::channels: INFO  new channel offer_id=OfferId(c) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-3 confidential_ring_buffer=false confidential_external_memory=false
[12.306532] vmbus_server::channels: INFO  sending offer to guest channel_id=0xe connection_id=0x200e key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-4
[12.306546] vmbus_server::channels: INFO  new channel offer_id=OfferId(d) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-4 confidential_ring_buffer=false confidential_external_memory=false
[12.306556] vmbus_server::channels: INFO  sending offer to guest channel_id=0xf connection_id=0x200f key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-5
[12.306563] vmbus_server::channels: INFO  new channel offer_id=OfferId(e) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-5 confidential_ring_buffer=false confidential_external_memory=false
[12.306581] vmbus_server::channels: INFO  sending offer to guest channel_id=0x10 connection_id=0x2010 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-6
[12.306588] vmbus_server::channels: INFO  new channel offer_id=OfferId(f) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-6 confidential_ring_buffer=false confidential_external_memory=false
[12.306602] vmbus_client: INFO  received offer state=Connected channel_id=0x11 interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=61f0a39b-c778-405b-bc5b-ffca0b817e4b subchannel_index=0x7
[12.306621] vmbus_server::channels: INFO  sending offer to guest channel_id=0x11 connection_id=0x2011 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-7
[12.306628] vmbus_server::channels: INFO  new channel offer_id=OfferId(10) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-7 confidential_ring_buffer=false confidential_external_memory=false
[12.307211] vmbus_client: INFO  opening channel on host channel_id=0xb key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-1
[12.307709] vmbus_server::channels: INFO  opened channel offer_id=0xa channel_id=0xb key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-1 result=0
[12.308232] vmbus_client: INFO  opening channel on host channel_id=0xc key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-2
[12.308687] vmbus_server::channels: INFO  opened channel dropped_ratelimited=0x0 offer_id=0xb channel_id=0xc key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-2 result=0
[12.309189] vmbus_client: INFO  opening channel on host channel_id=0xd key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-3
[12.310083] vmbus_client: INFO  opening channel on host channel_id=0xe key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-4
[12.311074] vmbus_client: INFO  opening channel on host channel_id=0xf key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-5
[12.311996] vmbus_client: INFO  opening channel on host channel_id=0x10 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-6
[12.312961] vmbus_client: INFO  opening channel on host channel_id=0x11 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{61f0a39b-c778-405b-bc5b-ffca0b817e4b}-7
