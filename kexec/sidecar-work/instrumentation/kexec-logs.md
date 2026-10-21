[0.000000] Linux version 6.18.37-microsoft-hcl+ (hargar@UbuntuVM-22) (gcc (Ubuntu 11.4.0-1ubuntu1~22.04.3) 11.4.0, GNU ld (GNU Binutils for Ubuntu) 2.38) #1 SMP PREEMPT_DYNAMIC Tue Oct  6 00:28:51 UTC 2026
[0.000000] Command line: loglevel=8 log_buf_len=128K printk.time=1 console_msg_format=syslog uio_hv_generic.no_mask=1 coredump_filter=0x33 cpufreq.off=1 cpuidle.off=1 cryptomgr.notests idle=halt initcall_blacklist=init_real_mode,sbf_init lpj=3000000 no_timer_check noxsave oops=panic panic_on_warn=0 panic_print=0 panic=-1 printk.devkmsg=on reboot=t rootfstype=tmpfs sysctl.vm.compaction_proactiveness=0 tsc=reliable unknown_nmi_panic=1 vfio_pci.ids=1414:00ba vfio.enable_unsafe_noiommu_mode=1 rdinit=/underhill-init OPENHCL_NVME_VFIO=1 hv_storvsc.storvsc_vcpus_per_sub_channel=2048 hv_storvsc.storvsc_max_hw_queues=2 hv_storvsc.storvsc_ringbuffer_size=0x8000 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off iommu=off pci=off swiotlb=1,1 console=ttynull OPENHCL_NVME_KEEP_ALIVE=host,privatepool hv_vmbus.message_connection_id=0x800074 OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug RUST_BACKTRACE=full boot_cpus=0,32
[0.000000] KERNEL supported cpus:
[0.000000] Intel GenuineIntel
[0.000000] AMD AuthenticAMD
[0.000000] BIOS-provided physical RAM map:
[0.000000] BIOS-e820: [mem 0x0000000408000000-0x000000040803ffff] reserved
[0.000000] BIOS-e820: [mem 0x0000000408040000-0x0000000408dfffff] usable
[0.000000] BIOS-e820: [mem 0x0000000408e00000-0x0000000408e19fff] reserved
[0.000000] BIOS-e820: [mem 0x0000000408e1a000-0x000000040a323fff] usable
[0.000000] BIOS-e820: [mem 0x000000040a324000-0x000000040a34ffff] reserved
[0.000000] BIOS-e820: [mem 0x000000040a350000-0x000000040a38ffff] usable
[0.000000] BIOS-e820: [mem 0x000000040a390000-0x000000040a493fff] reserved
[0.000000] BIOS-e820: [mem 0x000000040a494000-0x000000040a4aafff] usable
[0.000000] BIOS-e820: [mem 0x000000040a4ab000-0x000000040a7ecfff] reserved
[0.000000] BIOS-e820: [mem 0x000000040a7ed000-0x000000040a7fffff] usable
[0.000000] BIOS-e820: [mem 0x000000040a800000-0x000000040affffff] reserved
[0.000000] BIOS-e820: [mem 0x000000040b000000-0x0000000443bfffff] usable
[0.000000] BIOS-e820: [mem 0x0000000443c00000-0x0000000447ffffff] reserved
[0.000000] random: crng init done
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
[0.000000] tsc: Detected 2000.000 MHz processor
[0.000004] e820: update [mem 0x00000000-0x00000fff] usable ==> reserved
[0.000006] e820: remove [mem 0x000a0000-0x000fffff] usable
[0.000010] last_pfn = 0x443c00 max_arch_pfn = 0x1000000000
[0.000011] x86/PAT: PAT support disabled because CONFIG_X86_PAT is disabled in the kernel.
[0.000013] x86/PAT: Configuration [0-7]: WB  WT  UC- UC  WB  WT  UC- UC
[0.000014] x2apic: enabled by BIOS, switching to x2apic ops
[0.000016] last_pfn = 0x0 max_arch_pfn = 0x1000000000
[0.000019] Using GB pages for direct mapping
[0.000541] printk: log buffer data + meta data: 131072 + 458752 = 589824 bytes
[0.000542] printk: early log buf free: 12752(77%)
[0.000543] RAMDISK: [mem 0x4421cb000-0x4429fffff]
[0.000607] OF: reserved mem: Reserved memory: No reserved-memory node in the DT
[0.000762] NODE_DATA(0) allocated [mem 0x443b4d8c0-0x443b5213f]
[0.000797] Zone ranges:
[0.000798] DMA      empty
[0.000798] DMA32    empty
[0.000799] Normal   [mem 0x0000000408040000-0x0000000443bfffff]
[0.000800] Device   empty
[0.000801] Movable zone start for each node
[0.000801] Early memory node ranges
[0.000802] node   0: [mem 0x0000000408040000-0x0000000408dfffff]
[0.000803] node   0: [mem 0x0000000408e1a000-0x000000040a323fff]
[0.000803] node   0: [mem 0x000000040a350000-0x000000040a38ffff]
[0.000804] node   0: [mem 0x000000040a494000-0x000000040a4aafff]
[0.000804] node   0: [mem 0x000000040a7ed000-0x000000040a7fffff]
[0.000804] node   0: [mem 0x000000040b000000-0x0000000443bfffff]
[0.000805] Initmem setup node 0 [mem 0x0000000408040000-0x0000000443bfffff]
[0.000830] On node 0, zone Normal: 64 pages in unavailable ranges
[0.000863] On node 0, zone Normal: 26 pages in unavailable ranges
[0.000864] On node 0, zone Normal: 44 pages in unavailable ranges
[0.000866] On node 0, zone Normal: 260 pages in unavailable ranges
[0.000873] On node 0, zone Normal: 834 pages in unavailable ranges
[0.002287] On node 0, zone Normal: 2048 pages in unavailable ranges
[0.002405] On node 0, zone Normal: 17408 pages in unavailable ranges
[0.002447] APIC: Switched APIC routing to: physical x2apic
[0.002448] IMCR and PIC compatibility mode.
[0.002470] TSC deadline timer available
[0.002473] CPU topo: Max. logical packages:   1
[0.002473] CPU topo: Max. logical dies:       1
[0.002474] CPU topo: Max. dies per package:   1
[0.002476] CPU topo: Max. threads per core:   2
[0.002477] CPU topo: Num. cores per package:    32
[0.002478] CPU topo: Num. threads per package:  64
[0.002478] CPU topo: Allowing 64 present CPUs plus 0 hotplug CPUs
[0.002483] Cannot find an available gap in the 32-bit address range
[0.002484] PCI devices with unassigned 32-bit BARs may not work!
[0.002484] [mem 0x443d00000-0x4440fffff] available for PCI devices
[0.002485] Booting paravirtualized kernel on Hyper-V
[0.002489] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
[0.002495] setup_percpu: NR_CPUS:2048 nr_cpumask_bits:64 nr_cpu_ids:64 nr_node_ids:1
[0.004850] percpu: Embedded 41 pages/cpu s127680 r8192 d32064 u262144
[0.004858] pcpu-alloc: s127680 r8192 d32064 u262144 alloc=1*2097152
[0.004860] pcpu-alloc: [0] 00 01 02 03 04 05 06 07 [0] 08 09 10 11 12 13 14 15
[0.004866] pcpu-alloc: [0] 16 17 18 19 20 21 22 23 [0] 24 25 26 27 28 29 30 31
[0.004869] pcpu-alloc: [0] 32 33 34 35 36 37 38 39 [0] 40 41 42 43 44 45 46 47
[0.004873] pcpu-alloc: [0] 48 49 50 51 52 53 54 55 [0] 56 57 58 59 60 61 62 63
[0.004893] Hyper-V: PV spinlocks enabled
[0.004895] PV qspinlock hash table entries: 256 (order: 0, 4096 bytes, linear)
[0.004899] Kernel command line: loglevel=8 log_buf_len=128K printk.time=1 console_msg_format=syslog uio_hv_generic.no_mask=1 coredump_filter=0x33 cpufreq.off=1 cpuidle.off=1 cryptomgr.notests idle=halt initcall_blacklist=init_real_mode,sbf_init lpj=3000000 no_timer_check noxsave oops=panic panic_on_warn=0 panic_print=0 panic=-1 printk.devkmsg=on reboot=t rootfstype=tmpfs sysctl.vm.compaction_proactiveness=0 tsc=reliable unknown_nmi_panic=1 vfio_pci.ids=1414:00ba vfio.enable_unsafe_noiommu_mode=1 rdinit=/underhill-init OPENHCL_NVME_VFIO=1 hv_storvsc.storvsc_vcpus_per_sub_channel=2048 hv_storvsc.storvsc_max_hw_queues=2 hv_storvsc.storvsc_ringbuffer_size=0x8000 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off iommu=off pci=off swiotlb=1,1 console=ttynull OPENHCL_NVME_KEEP_ALIVE=host,privatepool hv_vmbus.message_connection_id=0x800074 OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug RUST_BACKTRACE=full boot_cpus=0,32
[0.004945] blacklisting initcall init_real_mode
[0.004947] blacklisting initcall sbf_init
[0.004962] Kernel: The 'panic_print' parameter is now deprecated. Please use 'panic_sys_info' and 'panic_console_replay' instead.
[0.005055] Unknown kernel command line parameters "OPENHCL_NVME_VFIO=1 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off OPENHCL_NVME_KEEP_ALIVE=host,privatepool OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug RUST_BACKTRACE=full OPENHCL_KEXEC_BOOTSTRAP_TIMES=0000000019681bb3,00000000196846ec OPENHCL_KEXEC_SERVICING=1", will be passed to user space.
[0.005168] Dentry cache hash table entries: 131072 (order: 8, 1048576 bytes, linear)
[0.005215] Inode-cache hash table entries: 65536 (order: 7, 524288 bytes, linear)
[0.006034] Fallback order for Node 0: 0
[0.006039] Built 1 zonelists, mobility grouping on.  Total pages: 241460
[0.006040] Policy zone: Normal
[0.006041] mem auto-init: stack:off, heap alloc:off, heap free:off
[0.007601] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=64, Nodes=1
[0.007979] Dynamic Preempt: none
[0.008247] rcu: Preemptible hierarchical RCU implementation.
[0.008247] rcu: <09>RCU restricting CPUs from NR_CPUS=2048 to nr_cpu_ids=64.
[0.008249] rcu: RCU calculated value of scheduler-enlistment delay is 10 jiffies.
[0.008249] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=64
[0.008312] Using NULL legacy PIC
[0.008312] NR_IRQS: 131328, nr_irqs: 1040, preallocated irqs: 0
[0.008376] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[0.008379] Persistent clock returned invalid value
[0.008514] APIC: Switch to symmetric I/O mode setup
[0.008519] Hyper-V: Host Build 10.0.29667.1001-1-0
[0.008520] Hyper-V: enabling crash_kexec_post_notifiers
[0.008656] Hyper-V: Using IPI hypercalls
[0.008659] APIC: send_IPI() replaced with hv_send_ipi()
[0.008664] APIC: send_IPI_mask() replaced with hv_send_ipi_mask()
[0.008666] APIC: send_IPI_mask_allbutself() replaced with hv_send_ipi_mask_allbutself()
[0.008668] APIC: send_IPI_allbutself() replaced with hv_send_ipi_allbutself()
[0.008669] APIC: send_IPI_all() replaced with hv_send_ipi_all()
[0.008671] APIC: send_IPI_self() replaced with hv_send_ipi_self()
[0.008747] APIC: wakeup_secondary_cpu_64() replaced with hv_vtl_wakeup_secondary_cpu()
[0.008768] clocksource: tsc-early: mask: 0xffffffffffffffff max_cycles: 0x39a85c9bff6, max_idle_ns: 881590591483 ns
[0.008773] Calibrating delay loop (skipped) preset value.. 600.00 BogoMIPS (lpj=3000000)
[0.008846] x86/cpu: User Mode Instruction Prevention (UMIP) activated
[0.008852] Last level iTLB entries: 4KB 0, 2MB 0, 4MB 0
[0.008853] Last level dTLB entries: 4KB 0, 2MB 0, 4MB 0, 1GB 0
[0.008861] mitigations: Enabled attack vectors: user_kernel, user_user, SMT mitigations: auto
[0.008864] Speculative Store Bypass: Vulnerable
[0.008865] Spectre V2 : Vulnerable
[0.008866] RETBleed: Vulnerable
[0.008866] Spectre V2 : User space: Vulnerable
[0.008867] Spectre V1 : Vulnerable: __user pointer sanitization and usercopy barriers only; no swapgs barriers
[0.008869] x86/fpu: x87 FPU will use FXSAVE
[0.010256] Freeing SMP alternatives memory: 20K
[0.010259] pid_max: default: 65536 minimum: 512
[0.010385] Mount-cache hash table entries: 2048 (order: 2, 16384 bytes, linear)
[0.010388] Mountpoint-cache hash table entries: 2048 (order: 2, 16384 bytes, linear)
[0.010628] smpboot: CPU0: Intel(R) Xeon(R) Platinum 8460Y+ (family: 0x6, model: 0x8f, stepping: 0x8)
[0.010789] Performance Events: unsupported CPU family 6 model 143 no PMU driver, software events only.
[0.010794] signal: max sigframe size: 1040
[0.010827] rcu: Hierarchical SRCU implementation.
[0.010828] rcu: <09>Max phase no-delay instances is 1000.
[0.010865] Timer migration: 2 hierarchy levels; 8 children per group; 2 crossnode level
[0.011203] smp: Bringing up secondary CPUs ...
[0.011277] smpboot: x86: Booting SMP configuration:
[0.011277] .... node  #0, CPUs:       #32
[0.018771] smp: Brought up 1 node, 2 CPUs
[0.018771] smpboot: Total of 2 processors activated (1200.00 BogoMIPS)
[0.018771] Memory: 911968K/965840K available (4421K kernel code, 480K rwdata, 1272K rodata, 920K init, 584K bss, 47920K reserved, 0K cma-reserved)
[0.018771] devtmpfs: initialized
[0.018771] x86/mm: Memory block size: 128MB
[0.018771] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
[0.018771] posixtimers hash table entries: 32768 (order: 7, 524288 bytes, linear)
[0.018771] futex hash table entries: 16384 (1048576 bytes on 1 NUMA nodes, total 1024 KiB, linear).
[0.018771] NET: Registered PF_NETLINK/PF_ROUTE protocol family
[0.018771] initcall sbf_init blacklisted
[0.018771] PCI: Fatal: No config space access function found
[0.018771] iommu: Default domain type: Translated
[0.018771] iommu: DMA domain TLB invalidation policy: lazy mode
[0.018771] SCSI subsystem initialized
[0.018771] hv_vmbus: VMBus is present in DeviceTree
[0.018771] hv_vmbus: Vmbus version:5.3
[0.018771] PCI: System does not support PCI
[0.018771] clocksource: Switched to clocksource tsc-early
[0.018771] NET: Registered PF_UNIX/PF_LOCAL protocol family
[0.018771] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x39a85c9bff6, max_idle_ns: 881590591483 ns
[0.018771] clocksource: Switched to clocksource tsc
[0.018771] Unpacking initramfs...
[0.029715] workingset: timestamp_bits=56 max_order=18 bucket_order=0
[0.029810] io scheduler mq-deadline registered
[0.030135] Serial: 8250/16550 driver, 4 ports, IRQ sharing enabled
[0.031214] printk: legacy console [ttynull0] enabled
[0.031586] hv_vmbus: registering driver uio_hv_generic
[0.038796] VFIO - User Level meta-driver version: 0.3
[0.038818] vfio_pci: add [1414:00ba[ffffffff:ffffffff]] class 0x000000/00000000
[0.039119] NET: Registered PF_VSOCK protocol family
[0.039121] hv_vmbus: registering driver hv_sock
[0.039200] IPI shorthand broadcast: enabled
[0.039676] sched_clock: Marking stable (30154228, 8771652)->(46178875, -7252995)
[0.048908] Demotion targets for Node 0: null
[0.049190] clk: Disabling unused clocks
[0.130718] Freeing initrd memory: 8404K
[0.131024] Freeing unused kernel image (initmem) memory: 920K
[0.131028] Write protecting the kernel read-only data: 8192k
[0.131655] Freeing unused kernel image (text/rodata gap) memory: 1720K
[0.131921] Freeing unused kernel image (rodata/data gap) memory: 776K
[0.132032] Freeing unused kernel image (bss_decrypted gap) memory: 332K
[0.168781] Run /underhill-init as init process
[0.168782] with arguments:
[0.168783] /underhill-init
[0.168783] with environment:
[0.168784] HOME=/
[0.168784] TERM=linux
[0.168785] OPENHCL_NVME_VFIO=1
[0.168785] MIMALLOC_ARENA_EAGER_COMMIT=0
[0.168785] acpi=off
[0.168786] OPENHCL_NVME_KEEP_ALIVE=host,privatepool
[0.168786] OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release
[0.168787] UNDERHILL_DIAG=1
[0.168787] HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug
[0.168788] RUST_BACKTRACE=full
[0.168788] OPENHCL_KEXEC_BOOTSTRAP_TIMES=0000000019681bb3,00000000196846ec
[0.168788] OPENHCL_KEXEC_SERVICING=1
[0.170008] underhill_init: Initial process: crate_name=underhill_init, crate_revision=20bcbc74bb2e4d64d64b77436fd40f438c19d8c8, crate_branch=user/hargar/kexec-2026-6.18-igvm-clean
[0.170023] underhill_init: Mounting filesystems
[0.170107] underhill_init: Command line args: Options { setup_script: [], underhill_args: [] }
[0.170108] underhill_init: Setting system resource limits and parameters
[0.170393] underhill_init: Using 64 bytes of entropy from the host
[0.170409] underhill_init: registered vfio-pci as driver for nvme
[0.170472] underhill_init: running KERNEL_BOOT_TIME="161201044" "/bin/openvmm_hcl" "--pid" "/run/underhill.pid"
[0.170642] underhill_init: loading kernel module /lib/modules/000/pci-hyperv-intf.ko:
[0.171448] underhill_init: load complete for /lib/modules/000/pci-hyperv-intf.ko
[0.171472] underhill_init: loading kernel module /lib/modules/001/pci-hyperv.ko:
[0.172300] hv_vmbus: registering driver hv_pci
[0.172360] underhill_init: load complete for /lib/modules/001/pci-hyperv.ko
[0.172379] underhill_init: loading kernel module /lib/modules/999/hv_storvsc.ko: storvsc_vcpus_per_sub_channel=2048 storvsc_max_hw_queues=2 storvsc_ringbuffer_size=0x8000
[0.172888] underhill_core: INFO  VMM process crate_name="build_info" crate_revision="20bcbc74bb2e4d64d64b77436fd40f438c19d8c8" openhcl_version=""
[0.172926] underhill_core: INFO  boot loader times start=0x885f0 end=0x96149 sidecar_start=0x88af1 sidecar_end=0x9012b elapsed=5.6153ms sidecar_elapsed=3.0266ms
[0.173132] hv_vmbus: registering driver hv_storvsc
[0.173183] underhill_init: load complete for /lib/modules/999/hv_storvsc.ko
[0.173203] diag_server: INFO  control starting control_address=VmAddress(Address { cid: ffffffff, port: 1 })
[0.173216] diag_server: INFO  data starting data_address=VmAddress(Address { cid: ffffffff, port: 2 })
[0.173292] mesh_tracing: INFO  updated trace filter filter="info,underhill_core::dispatch=debug,underhill_core::worker=debug"
[0.175441] mesh_tracing: INFO  updated trace filter filter="info,underhill_core::dispatch=debug,underhill_core::worker=debug"
[0.176374] guest_emulation_transport::process_loop: INFO  [GET] version negotiated protocol=NICKEL_REV2
[0.176423] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.176793] underhill_core::worker: DEBUG worker_new{ name="UnderhillWorker" action="new"}:init:init/dps:  device platform settings DevicePlatformSettings { smbios: Smbios { serial_number: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], base_board_serial_number: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], chassis_serial_number: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], chassis_asset_tag: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], system_manufacturer: [], system_product_name: [], system_version: [], system_sku_number: [], system_family: [], bios_lock_string: [], memory_device_serial_number: [], processor_manufacturer: [73, 110, 116, 1
[0.176795] underhill_core::worker: 01, 108, 0], processor_version: [73, 110, 116, 101, 108, 40, 82, 41, 32, 88, 101, 111, 110, 40, 82, 41, 32, 80, 108, 97, 116, 105, 110, 117, 109, 32, 56, 52, 54, 48, 89, 43, 0], processor_id: 0, external_clock: 16000, max_speed: 4000, current_speed: 2000, processor_characteristics: 252, processor_family2: 179, processor_type: 3, voltage: 144, status: 65, processor_upgrade: 6 }, general: General { secure_boot_enabled: false, secure_boot_template: MicrosoftWindows, bios_guid: ee949614-8319-40ba-acdd-481cb8109d7c, console_mode: Default, battery_enabled: false, processor_idle_enabled: false, tpm_enabled: false, com1_enabled: false, com1_debugger_mode: false, com1_vmbus_redirector: false, com2_enabled: false, com2_debugger_mode: false, com2_vmbus_redirector: false, firmware_debugging_enabled: false, hibernation_enabled: false, suppress_attestation: Some(false), generation_id: Some([70, 87, 40, 60, 253, 29, 184, 235, 60, 147, 147, 206, 109,
[0.176797] underhill_core::worker: 88, 191, 103]), legacy_memory_map: false, pause_after_boot_failure: false, pxe_ip_v6: false, measure_additional_pcrs: true, disable_frontpage: false, disable_sha384_pcr: false, media_present_enabled_by_default: false, vpci_boot_enabled: false, memory_protection_mode: Disabled, default_boot_always_attempt: false, num_lock_enabled: false, pcat_boot_device_order: [Optical, HardDrive, Network, Floppy], vpci_instance_filter: None, nvdimm_count: 0, psp_enabled: false, vmbus_redirection_enabled: true, always_relay_host_mmio: false, vtl2_settings: None, is_servicing_scenario: false, watchdog_enabled: false, firmware_mode_is_pcat: false, imc_enabled: false, cxl_memory_enabled: false, efi_diagnostics_log_level: DEFAULT, guest_state_lifetime: Default, guest_state_encryption_policy: None, management_vtl_features: ManagementVtlFeatures { strict_encryption_policy: false, load_firmware_supported: false, control_ak_cert_provisioning: false, attempt_
[0.176800] underhill_core::worker: ak_cert_callback: false, tx_only_serial_port: false }, force_dma_bounce_enabled: false, hardware_sealing_policy: None }, acpi_tables: [] }
[0.178980] underhill_core::worker: DEBUG worker_new{ name="UnderhillWorker" action="new"}:init:  restored servicing state from persisted memory saved_state_len=0x1e0bc
[0.181116] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  kernel boot time kernel_boot_time_ns=0x99bbb94
[0.183034] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] read_from_dt: persisted_state_header present=false, sidecar=Enabled { enable_logging: false, cpu_threshold: Some(100) }"
[0.183041] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] reading topology from host device tree"
[0.183047] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] reclaim device tree memory 40a350000-40a390000"
[0.183052] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] persisted state region sized to MemoryRange {"
[0.183057] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="    start: 0x408000000,"
[0.183063] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="    end: 0x408040000,"
[0.183068] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="}, remainder MemoryRange {"
[0.183074] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="    start: 0x408040000,"
[0.183079] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="    end: 0x408200000,"
[0.183084] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="}"
[0.183089] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] vtl2 gpa pool coming from heuristics table: Release"
[0.183095] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] Extrapolated VTL2 DMA hint: 17408 pages (68 MiB) for 64 VPs and 1024 MiB VTL2 memory"
[0.183100] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] allocated VTL2 pool at MemoryRange {"
[0.183105] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="    start: 0x443c00000,"
[0.183110] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="    end: 0x448000000,"
[0.183115] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="}"
[0.183120] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] openhcl_boot: logging enabled"
[0.183125] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] serial configuration: None"
[0.183130] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] sidecar: created node index=0 base_vp=0 vp_count=32 vnode=0"
[0.183135] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] sidecar: created node index=1 base_vp=32 vp_count=32 vnode=0"
[0.183141] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] sidecar: reserved kexec regions MemoryRange {"
[0.183146] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="    start: 0x40a800000,"
[0.183151] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="    end: 0x40ac00000,"
[0.183157] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="} and MemoryRange {"
[0.183162] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="    start: 0x40ac00000,"
[0.183167] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="    end: 0x40b000000,"
[0.183172] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="}"
[0.183176] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] sidecar starting, 2 nodes, 64 cpus, 0x342000 total bytes"
[0.183181] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] sidecar: boot_cpus parameter: boot_cpus=0,32"
[0.183186] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] uninitializing hypercalls"
[0.183191] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  openhcl_boot log line="[INFO] about to jump to kernel"
[0.183444] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  memory allocation mode memory_allocation_mode=Host
[0.183452] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  vtl2 ram vtl2_ram="0x408000000-0x448000000"
[0.183460] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  overriding dps to enable default_boot_always_attempt
[0.183466] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  flush logs result duration_us=0xb8
[0.183788] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  vtl0 ram vtl0_ram="0x0-0xf8000000(0), 0x100000000-0x408000000(0)"
[0.183796] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  vtl0 mmio vtl0_mmio="0xf8000000-0x100000000, 0xfe0000000-0x1000000000"
[0.183802] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  shared pool shared_pool=""
[0.183832] vmgs::storage: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  vmgs_capacity=0x4000000
[0.183843] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  enabling alias map alias_map=0x200000000000
[0.184061] sidecar_client: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  sidecar client initialized with 2 node(s) layout=[0..20, 20..40]
[0.184079] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.184151] underhill_mem::init: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  Skipping VTL 1 memory map creation
[0.184161] openhcl_dma_manager: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  create dma manager shared_ranges=[] private_ranges=[MemoryRange { start: 443c00000, end: 448000000 }] vtom=0x0 isolation_type=None
[0.184176] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.184188] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  starting guest memory self test
[0.184207] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  guest memory self test complete
[0.185141] underhill_core::dispatch::vtl2_settings_worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:new_initial_controllers:  Initial VTL2 settings vtl2_settings=None config_timeout_in_seconds=0x5
[0.185188] underhill_core::nvme_manager::manager: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  starting nvme manager vp_count=0x40 save_restore_supported=true saved_state=true
[0.185261] underhill_core::nvme_manager::manager: INFO nvme_manager_restore:nvme_manager_worker_restore:  nvme manager worker restored 0 devices
[0.185288] underhill_core::worker: DEBUG worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  NVMe VFIO manager initialized, setting up resolver nvme_vfio=true save_restore_supported=true
[0.185602] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  Underhill MNF enabled? enable_mnf=false
[0.185635] vmbus_server::channels: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:  Limiting VmBus connections to version version=MaxVersionInfo { version: 60000, feature_flags: FeatureFlags { guest_specified_signal_parameters: true, channel_interrupt_redirection: true, modify_connection: true, client_id: false, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: true } }
[0.185690] vmbus_client::saved_state: INFO  channel restored key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 state=Opened
[0.185702] vmbus_client::saved_state: INFO  channel restored key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0 state=Offered
[0.185708] vmbus_client::saved_state: INFO  channel restored key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0 state=Opened
[0.185719] vmbus_client::saved_state: INFO  channel restored key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0 state=Opened
[0.185725] vmbus_client::saved_state: INFO  channel restored key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0 state=Opened
[0.185734] vmbus_client::saved_state: INFO  channel restored key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 state=Offered
[0.185740] vmbus_client::saved_state: INFO  channel restored key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 state=Opened
[0.185753] vmbus_client::saved_state: INFO  channel restored key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 state=Opened
[0.185759] vmbus_client::saved_state: INFO  channel restored key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 state=Offered
[0.185770] vmbus_client::saved_state: INFO  channel restored key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 state=Opened
[0.185777] vmbus_client::saved_state: INFO  channel restored key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 state=Offered
[0.185783] vmbus_client::saved_state: INFO  channel restored key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0 state=Opened
[0.185789] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0 state=Opened
[0.185794] vmbus_client::saved_state: INFO  channel restored key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 state=Opened
[0.185810] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-1 state=Opened
[0.185816] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-2 state=Opened
[0.185821] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-3 state=Opened
[0.185832] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-4 state=Opened
[0.185838] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-5 state=Opened
[0.185843] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-6 state=Opened
[0.185849] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-7 state=Opened
[0.185923] vmbus_server::channels: INFO  new channel offer_id=OfferId(0) key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.185955] vmbus_server::channels: INFO  new channel offer_id=OfferId(1) key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.185982] vmbus_server::channels: INFO  new channel offer_id=OfferId(2) key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186002] vmbus_server::channels: INFO  new channel offer_id=OfferId(3) key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186020] vmbus_server::channels: INFO  new channel offer_id=OfferId(4) key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186033] vmbus_server::channels: INFO  new channel offer_id=OfferId(5) key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186045] vmbus_server::channels: INFO  new channel offer_id=OfferId(6) key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186058] vmbus_server::channels: INFO  new channel offer_id=OfferId(7) key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186076] vmbus_server::channels: INFO  new channel offer_id=OfferId(8) key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186086] vmbus_server::channels: INFO  new channel offer_id=OfferId(9) key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186098] vmbus_server::channels: INFO  new channel offer_id=OfferId(a) key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186111] vmbus_server::channels: INFO  new channel offer_id=OfferId(b) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186121] vmbus_server::channels: INFO  new channel offer_id=OfferId(c) key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186133] vmbus_server::channels: INFO  new channel offer_id=OfferId(d) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-1 confidential_ring_buffer=false confidential_external_memory=false
[0.186145] vmbus_server::channels: INFO  new channel offer_id=OfferId(e) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-2 confidential_ring_buffer=false confidential_external_memory=false
[0.186164] vmbus_server::channels: INFO  new channel offer_id=OfferId(f) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-3 confidential_ring_buffer=false confidential_external_memory=false
[0.186179] vmbus_server::channels: INFO  new channel offer_id=OfferId(10) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-4 confidential_ring_buffer=false confidential_external_memory=false
[0.186192] vmbus_server::channels: INFO  new channel offer_id=OfferId(11) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-5 confidential_ring_buffer=false confidential_external_memory=false
[0.186203] vmbus_server::channels: INFO  new channel offer_id=OfferId(12) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-6 confidential_ring_buffer=false confidential_external_memory=false
[0.186216] vmbus_server::channels: INFO  new channel offer_id=OfferId(13) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-7 confidential_ring_buffer=false confidential_external_memory=false
[0.186854] vmbus_server::channels: INFO  new channel offer_id=OfferId(14) key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 confidential_ring_buffer=true confidential_external_memory=false
[0.186911] vmbus_relay_intercept_device: INFO  matching channel offered offer=OfferInfo { offer: OfferChannel { interface_id: 0e0b6031-5213-4934-818b-38d90ced39db, instance_id: b6650ff7-33bc-4840-8048-e0676786f393, rsvd: [0, 0, 0, 0], flags: OfferFlags { enumerate_device_interface: true, confidential_ring_buffer: false, confidential_external_memory: false, require_pinned_external_memory: false, named_pipe_mode: true, tlnpi_provider: false }, mmio_megabytes: 0, user_defined: UserDefinedData([040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000]), subchannel_index: 0, mmio_megabytes_optional: 0, channel_id: ChannelId(9), monitor_id: ff, monitor_allocated: 0, is_dedicated: 1, connection_id: 2009 }, guest_to_host_interrupt: Interrupt { inner: InterruptInner }, request_send: SenderCore(Ma
[0.186913] vmbus_relay_intercept_device: nuallyDrop { value: MaybeDangling(Queue { remote: OnceLock(<uninit>), local: Mutex { data: LocalQueue { messages: ErasedVecDeque { cap: 0, head: 0, len: 0 }, ports: [], waker: Some(Waker { data: 0x4f6800e03b0, vtable: 0x7f1d39423828 }), remote: false, receiver_gone: false, remove_closed: false, new_handler: 0x7f1d38502b70 } }, receiver: UnsafeCell { .. } }) }), revoke_recv: ManuallyDrop { value: MaybeDangling(OneshotReceiverCore(Slot { state: Mutex { data: Waiting(None) }, receiver: UnsafeCell { .. } })) } }
[0.187152] vmbus_client: INFO  opening channel on host channel_id=0x9 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[0.193185] vmbus_server::channels::saved_state: INFO  channel restored key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 state=Open
[0.193193] vmbus_server::channels::saved_state: INFO  channel restored key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0 state=Closed
[0.193199] vmbus_server::channels::saved_state: INFO  channel restored key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0 state=Open
[0.193207] vmbus_server::channels::saved_state: INFO  channel restored key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0 state=Open
[0.193213] vmbus_server::channels::saved_state: INFO  channel restored key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0 state=Open
[0.193219] vmbus_server::channels::saved_state: INFO  channel restored key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 state=Closed
[0.193224] vmbus_server::channels::saved_state: INFO  channel restored key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 state=Open
[0.193230] vmbus_server::channels::saved_state: INFO  channel restored key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 state=Open
[0.193235] vmbus_server::channels::saved_state: INFO  channel restored key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 state=Open
[0.193241] vmbus_server::channels::saved_state: INFO  channel restored key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 state=Closed
[0.193246] vmbus_server::channels::saved_state: INFO  channel restored key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0 state=Open
[0.193251] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0 state=Open
[0.193256] vmbus_server::channels::saved_state: INFO  channel restored key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 state=Open
[0.193265] vmbus_server::channels::saved_state: INFO  channel restored key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 state=Open
[0.193270] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-1 state=Open
[0.193276] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-2 state=Open
[0.193285] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-3 state=Open
[0.193291] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-4 state=Open
[0.193296] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-5 state=Open
[0.193301] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-6 state=Open
[0.193306] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-7 state=Open
[0.193381] hyperv_ic_guest::shutdown: INFO  version negotiated framework_version=3.0 message_version=3.2
[0.216699] state_unit: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/restore{ correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f}:restore_units:state_change{ operation="restore"}:  state change complete duration=25.752656ms
[0.216917] underhill_core: INFO  vm worker started
[0.217306] state_unit: INFO state_change{ operation="start"}:  state change complete duration=469.338<c2><b5>s
[0.217328] underhill_core::dispatch: INFO  resuming VM correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f blackout_time_ms=0x155 blackout_time="341.1049ms"
[0.217348] underhill_core::dispatch: INFO  servicing blackout phases correlation_id=f1dadae9-b2aa-4837-95a8-3a961c5de36f method="kexec" total=Some(341.1049ms) phases=Some([("stop", 1.7849ms), ("save", 1.9378ms), ("shutdown", 257.9<c2><b5>s), ("flush", 194<c2><b5>s), ("kexec_prep", 190.9<c2><b5>s), ("kexec_jump", 109.6705ms), ("bootstrap", 1.1065ms), ("kernel_init", 185.0741ms), ("settings", 423.3<c2><b5>s), ("state_read", 2.1371ms), ("state_fixup", 2.0876ms), ("vm_build", 9.8039ms), ("restore", 25.8443ms), ("restore_notify", 2.8<c2><b5>s), ("resume_wait", 74<c2><b5>s), ("start", 515.3<c2><b5>s)])
[8.735882] virt_mshv_vtl::processor: INFO  sidecar exit
[8.735896] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 28, write: true })
[8.736414] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x19
[8.737196] underhill_core::vp: INFO  onlining sidecar VP cpu=0x19
[8.819015] mshv_vtl_sidecar 40a880000.sidecar: cpu 25: removing from sidecar
[8.828788] mshv_vtl_sidecar 40a880000.sidecar: cpu 25: remove complete
[8.828792] smpboot: Booting Node 0 Processor 25 APIC 0x19
