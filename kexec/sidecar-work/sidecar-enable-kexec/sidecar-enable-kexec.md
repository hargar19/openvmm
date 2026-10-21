[0.000000] Linux version 6.18.37-microsoft-hcl+ (hargar@UbuntuVM-22) (gcc (Ubuntu 11.4.0-1ubuntu1~22.04.3) 11.4.0, GNU ld (GNU Binutils for Ubuntu) 2.38) #1 SMP PREEMPT_DYNAMIC Mon Oct  5 21:26:43 UTC 2026
[0.000000] Command line: loglevel=8 log_buf_len=128K printk.time=1 console_msg_format=syslog uio_hv_generic.no_mask=1 coredump_filter=0x33 cpufreq.off=1 cpuidle.off=1 cryptomgr.notests idle=halt initcall_blacklist=init_real_mode,sbf_init lpj=3000000 no_timer_check noxsave oops=panic panic_on_warn=0 panic_print=0 panic=-1 printk.devkmsg=on reboot=t rootfstype=tmpfs sysctl.vm.compaction_proactiveness=0 tsc=reliable unknown_nmi_panic=1 vfio_pci.ids=1414:00ba vfio.enable_unsafe_noiommu_mode=1 rdinit=/underhill-init OPENHCL_NVME_VFIO=1 hv_storvsc.storvsc_vcpus_per_sub_channel=2048 hv_storvsc.storvsc_max_hw_queues=2 hv_storvsc.storvsc_ringbuffer_size=0x8000 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off iommu=off pci=off swiotlb=1,1 console=ttynull OPENHCL_NVME_KEEP_ALIVE=host,privatepool hv_vmbus.message_connection_id=0x800074 OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug RUST_BACKTRACE=full boot_cpus=0,32
[0.000000] KERNEL supported cpus:
[0.000000] Intel GenuineIntel
[0.000000] AMD AuthenticAMD
[0.000000] BIOS-provided physical RAM map:
[0.000000] BIOS-e820: [mem 0x0000000408000000-0x000000040803ffff] reserved
[0.000000] BIOS-e820: [mem 0x0000000408040000-0x0000000408dfffff] usable
[0.000000] BIOS-e820: [mem 0x0000000408e00000-0x0000000408e19fff] reserved
[0.000000] BIOS-e820: [mem 0x0000000408e1a000-0x000000040a322fff] usable
[0.000000] BIOS-e820: [mem 0x000000040a323000-0x000000040a34efff] reserved
[0.000000] BIOS-e820: [mem 0x000000040a34f000-0x000000040a38efff] usable
[0.000000] BIOS-e820: [mem 0x000000040a38f000-0x000000040a492fff] reserved
[0.000000] BIOS-e820: [mem 0x000000040a493000-0x000000040a4a9fff] usable
[0.000000] BIOS-e820: [mem 0x000000040a4aa000-0x000000040a7ebfff] reserved
[0.000000] BIOS-e820: [mem 0x000000040a7ec000-0x000000040a7fffff] usable
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
[0.000003] e820: update [mem 0x00000000-0x00000fff] usable ==> reserved
[0.000005] e820: remove [mem 0x000a0000-0x000fffff] usable
[0.000010] last_pfn = 0x443c00 max_arch_pfn = 0x1000000000
[0.000010] x86/PAT: PAT support disabled because CONFIG_X86_PAT is disabled in the kernel.
[0.000012] x86/PAT: Configuration [0-7]: WB  WT  UC- UC  WB  WT  UC- UC
[0.000014] x2apic: enabled by BIOS, switching to x2apic ops
[0.000015] last_pfn = 0x0 max_arch_pfn = 0x1000000000
[0.000019] Using GB pages for direct mapping
[0.000540] printk: log buffer data + meta data: 131072 + 458752 = 589824 bytes
[0.000541] printk: early log buf free: 12752(77%)
[0.000542] RAMDISK: [mem 0x4421cc000-0x4429fffff]
[0.000604] OF: reserved mem: Reserved memory: No reserved-memory node in the DT
[0.000756] NODE_DATA(0) allocated [mem 0x443b4d880-0x443b520ff]
[0.000792] Zone ranges:
[0.000792] DMA      empty
[0.000793] DMA32    empty
[0.000794] Normal   [mem 0x0000000408040000-0x0000000443bfffff]
[0.000795] Device   empty
[0.000795] Movable zone start for each node
[0.000796] Early memory node ranges
[0.000796] node   0: [mem 0x0000000408040000-0x0000000408dfffff]
[0.000797] node   0: [mem 0x0000000408e1a000-0x000000040a322fff]
[0.000798] node   0: [mem 0x000000040a34f000-0x000000040a38efff]
[0.000798] node   0: [mem 0x000000040a493000-0x000000040a4a9fff]
[0.000799] node   0: [mem 0x000000040a7ec000-0x000000040a7fffff]
[0.000799] node   0: [mem 0x000000040b000000-0x0000000443bfffff]
[0.000800] Initmem setup node 0 [mem 0x0000000408040000-0x0000000443bfffff]
[0.000824] On node 0, zone Normal: 64 pages in unavailable ranges
[0.000858] On node 0, zone Normal: 26 pages in unavailable ranges
[0.000859] On node 0, zone Normal: 44 pages in unavailable ranges
[0.000861] On node 0, zone Normal: 260 pages in unavailable ranges
[0.000867] On node 0, zone Normal: 834 pages in unavailable ranges
[0.002268] On node 0, zone Normal: 2048 pages in unavailable ranges
[0.002385] On node 0, zone Normal: 17408 pages in unavailable ranges
[0.002427] APIC: Switched APIC routing to: physical x2apic
[0.002428] IMCR and PIC compatibility mode.
[0.002450] TSC deadline timer available
[0.002453] CPU topo: Max. logical packages:   1
[0.002453] CPU topo: Max. logical dies:       1
[0.002454] CPU topo: Max. dies per package:   1
[0.002456] CPU topo: Max. threads per core:   2
[0.002457] CPU topo: Num. cores per package:    32
[0.002458] CPU topo: Num. threads per package:  64
[0.002458] CPU topo: Allowing 64 present CPUs plus 0 hotplug CPUs
[0.002463] Cannot find an available gap in the 32-bit address range
[0.002463] PCI devices with unassigned 32-bit BARs may not work!
[0.002464] [mem 0x443d00000-0x4440fffff] available for PCI devices
[0.002464] Booting paravirtualized kernel on Hyper-V
[0.002469] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
[0.002474] setup_percpu: NR_CPUS:2048 nr_cpumask_bits:64 nr_cpu_ids:64 nr_node_ids:1
[0.004916] percpu: Embedded 41 pages/cpu s127680 r8192 d32064 u262144
[0.004920] pcpu-alloc: s127680 r8192 d32064 u262144 alloc=1*2097152
[0.004922] pcpu-alloc: [0] 00 01 02 03 04 05 06 07 [0] 08 09 10 11 12 13 14 15
[0.004927] pcpu-alloc: [0] 16 17 18 19 20 21 22 23 [0] 24 25 26 27 28 29 30 31
[0.004930] pcpu-alloc: [0] 32 33 34 35 36 37 38 39 [0] 40 41 42 43 44 45 46 47
[0.004934] pcpu-alloc: [0] 48 49 50 51 52 53 54 55 [0] 56 57 58 59 60 61 62 63
[0.004950] Hyper-V: PV spinlocks enabled
[0.004952] PV qspinlock hash table entries: 256 (order: 0, 4096 bytes, linear)
[0.004956] Kernel command line: loglevel=8 log_buf_len=128K printk.time=1 console_msg_format=syslog uio_hv_generic.no_mask=1 coredump_filter=0x33 cpufreq.off=1 cpuidle.off=1 cryptomgr.notests idle=halt initcall_blacklist=init_real_mode,sbf_init lpj=3000000 no_timer_check noxsave oops=panic panic_on_warn=0 panic_print=0 panic=-1 printk.devkmsg=on reboot=t rootfstype=tmpfs sysctl.vm.compaction_proactiveness=0 tsc=reliable unknown_nmi_panic=1 vfio_pci.ids=1414:00ba vfio.enable_unsafe_noiommu_mode=1 rdinit=/underhill-init OPENHCL_NVME_VFIO=1 hv_storvsc.storvsc_vcpus_per_sub_channel=2048 hv_storvsc.storvsc_max_hw_queues=2 hv_storvsc.storvsc_ringbuffer_size=0x8000 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off iommu=off pci=off swiotlb=1,1 console=ttynull OPENHCL_NVME_KEEP_ALIVE=host,privatepool hv_vmbus.message_connection_id=0x800074 OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug RUST_BACKTRACE=full boot_cpus=0,32
[0.005004] blacklisting initcall init_real_mode
[0.005006] blacklisting initcall sbf_init
[0.005021] Kernel: The 'panic_print' parameter is now deprecated. Please use 'panic_sys_info' and 'panic_console_replay' instead.
[0.005111] Unknown kernel command line parameters "OPENHCL_NVME_VFIO=1 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off OPENHCL_NVME_KEEP_ALIVE=host,privatepool OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug RUST_BACKTRACE=full OPENHCL_KEXEC_SERVICING=1", will be passed to user space.
[0.005225] Dentry cache hash table entries: 131072 (order: 8, 1048576 bytes, linear)
[0.005272] Inode-cache hash table entries: 65536 (order: 7, 524288 bytes, linear)
[0.006109] Fallback order for Node 0: 0
[0.006114] Built 1 zonelists, mobility grouping on.  Total pages: 241460
[0.006115] Policy zone: Normal
[0.006116] mem auto-init: stack:off, heap alloc:off, heap free:off
[0.007736] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=64, Nodes=1
[0.008087] Dynamic Preempt: none
[0.008333] rcu: Preemptible hierarchical RCU implementation.
[0.008333] rcu: <09>RCU restricting CPUs from NR_CPUS=2048 to nr_cpu_ids=64.
[0.008334] rcu: RCU calculated value of scheduler-enlistment delay is 10 jiffies.
[0.008335] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=64
[0.008396] Using NULL legacy PIC
[0.008397] NR_IRQS: 131328, nr_irqs: 1040, preallocated irqs: 0
[0.008455] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[0.008458] Persistent clock returned invalid value
[0.008593] APIC: Switch to symmetric I/O mode setup
[0.008598] Hyper-V: Host Build 10.0.29667.1001-1-0
[0.008600] Hyper-V: enabling crash_kexec_post_notifiers
[0.008745] Hyper-V: Using IPI hypercalls
[0.008750] APIC: send_IPI() replaced with hv_send_ipi()
[0.008755] APIC: send_IPI_mask() replaced with hv_send_ipi_mask()
[0.008757] APIC: send_IPI_mask_allbutself() replaced with hv_send_ipi_mask_allbutself()
[0.008759] APIC: send_IPI_allbutself() replaced with hv_send_ipi_allbutself()
[0.008761] APIC: send_IPI_all() replaced with hv_send_ipi_all()
[0.008762] APIC: send_IPI_self() replaced with hv_send_ipi_self()
[0.008834] APIC: wakeup_secondary_cpu_64() replaced with hv_vtl_wakeup_secondary_cpu()
[0.008853] clocksource: tsc-early: mask: 0xffffffffffffffff max_cycles: 0x39a85c9bff6, max_idle_ns: 881590591483 ns
[0.008858] Calibrating delay loop (skipped) preset value.. 600.00 BogoMIPS (lpj=3000000)
[0.008931] x86/cpu: User Mode Instruction Prevention (UMIP) activated
[0.008937] Last level iTLB entries: 4KB 0, 2MB 0, 4MB 0
[0.008938] Last level dTLB entries: 4KB 0, 2MB 0, 4MB 0, 1GB 0
[0.008946] mitigations: Enabled attack vectors: user_kernel, user_user, SMT mitigations: auto
[0.008948] Speculative Store Bypass: Vulnerable
[0.008949] Spectre V2 : Vulnerable
[0.008950] RETBleed: Vulnerable
[0.008950] Spectre V2 : User space: Vulnerable
[0.008950] Spectre V1 : Vulnerable: __user pointer sanitization and usercopy barriers only; no swapgs barriers
[0.008953] x86/fpu: x87 FPU will use FXSAVE
[0.010336] Freeing SMP alternatives memory: 20K
[0.010339] pid_max: default: 65536 minimum: 512
[0.010478] Mount-cache hash table entries: 2048 (order: 2, 16384 bytes, linear)
[0.010481] Mountpoint-cache hash table entries: 2048 (order: 2, 16384 bytes, linear)
[0.010764] smpboot: CPU0: Intel(R) Xeon(R) Platinum 8460Y+ (family: 0x6, model: 0x8f, stepping: 0x8)
[0.010931] Performance Events: unsupported CPU family 6 model 143 no PMU driver, software events only.
[0.010937] signal: max sigframe size: 1040
[0.010972] rcu: Hierarchical SRCU implementation.
[0.010973] rcu: <09>Max phase no-delay instances is 1000.
[0.011011] Timer migration: 2 hierarchy levels; 8 children per group; 2 crossnode level
[0.011355] smp: Bringing up secondary CPUs ...
[0.011429] smpboot: x86: Booting SMP configuration:
[0.011429] .... node  #0, CPUs:       #32
[0.018856] smp: Brought up 1 node, 2 CPUs
[0.018856] smpboot: Total of 2 processors activated (1200.00 BogoMIPS)
[0.018856] Memory: 911976K/965840K available (4421K kernel code, 480K rwdata, 1272K rodata, 920K init, 584K bss, 47912K reserved, 0K cma-reserved)
[0.018856] devtmpfs: initialized
[0.018856] x86/mm: Memory block size: 128MB
[0.018856] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
[0.018856] posixtimers hash table entries: 32768 (order: 7, 524288 bytes, linear)
[0.018856] futex hash table entries: 16384 (1048576 bytes on 1 NUMA nodes, total 1024 KiB, linear).
[0.018856] NET: Registered PF_NETLINK/PF_ROUTE protocol family
[0.018856] initcall sbf_init blacklisted
[0.018856] PCI: Fatal: No config space access function found
[0.018856] iommu: Default domain type: Translated
[0.018856] iommu: DMA domain TLB invalidation policy: lazy mode
[0.018856] SCSI subsystem initialized
[0.018856] hv_vmbus: VMBus is present in DeviceTree
[0.018856] hv_vmbus: Vmbus version:5.3
[0.018856] PCI: System does not support PCI
[0.018856] clocksource: Switched to clocksource tsc-early
[0.018856] NET: Registered PF_UNIX/PF_LOCAL protocol family
[0.018856] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x39a85c9bff6, max_idle_ns: 881590591483 ns
[0.018856] clocksource: Switched to clocksource tsc
[0.018856] Unpacking initramfs...
[0.029887] workingset: timestamp_bits=56 max_order=18 bucket_order=0
[0.029978] io scheduler mq-deadline registered
[0.030508] Serial: 8250/16550 driver, 4 ports, IRQ sharing enabled
[0.031412] printk: legacy console [ttynull0] enabled
[0.032023] hv_vmbus: registering driver uio_hv_generic
[0.038888] VFIO - User Level meta-driver version: 0.3
[0.038910] vfio_pci: add [1414:00ba[ffffffff:ffffffff]] class 0x000000/00000000
[0.039330] NET: Registered PF_VSOCK protocol family
[0.039332] hv_vmbus: registering driver hv_sock
[0.039410] IPI shorthand broadcast: enabled
[0.039888] sched_clock: Marking stable (30286502, 8856367)->(46533762, -7390893)
[0.049087] Demotion targets for Node 0: null
[0.049330] clk: Disabling unused clocks
[0.130707] Freeing initrd memory: 8400K
[0.131006] Freeing unused kernel image (initmem) memory: 920K
[0.131010] Write protecting the kernel read-only data: 8192k
[0.131644] Freeing unused kernel image (text/rodata gap) memory: 1720K
[0.131914] Freeing unused kernel image (rodata/data gap) memory: 776K
[0.132023] Freeing unused kernel image (bss_decrypted gap) memory: 332K
[0.168870] Run /underhill-init as init process
[0.168871] with arguments:
[0.168872] /underhill-init
[0.168872] with environment:
[0.168873] HOME=/
[0.168873] TERM=linux
[0.168874] OPENHCL_NVME_VFIO=1
[0.168874] MIMALLOC_ARENA_EAGER_COMMIT=0
[0.168874] acpi=off
[0.168875] OPENHCL_NVME_KEEP_ALIVE=host,privatepool
[0.168875] OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release
[0.168876] UNDERHILL_DIAG=1
[0.168876] HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug
[0.168877] RUST_BACKTRACE=full
[0.168877] OPENHCL_KEXEC_SERVICING=1
[0.170022] underhill_init: Initial process: crate_name=underhill_init, crate_revision=20bcbc74bb2e4d64d64b77436fd40f438c19d8c8, crate_branch=user/hargar/kexec-2026-6.18-igvm-clean
[0.170035] underhill_init: Mounting filesystems
[0.170116] underhill_init: Command line args: Options { setup_script: [], underhill_args: [] }
[0.170117] underhill_init: Setting system resource limits and parameters
[0.170401] underhill_init: Using 64 bytes of entropy from the host
[0.170416] underhill_init: registered vfio-pci as driver for nvme
[0.170478] underhill_init: running KERNEL_BOOT_TIME="161127533" "/bin/openvmm_hcl" "--pid" "/run/underhill.pid"
[0.170654] underhill_init: loading kernel module /lib/modules/000/pci-hyperv-intf.ko:
[0.171419] underhill_init: load complete for /lib/modules/000/pci-hyperv-intf.ko
[0.171442] underhill_init: loading kernel module /lib/modules/001/pci-hyperv.ko:
[0.172316] hv_vmbus: registering driver hv_pci
[0.172390] underhill_init: load complete for /lib/modules/001/pci-hyperv.ko
[0.172438] underhill_init: loading kernel module /lib/modules/999/hv_storvsc.ko: storvsc_vcpus_per_sub_channel=2048 storvsc_max_hw_queues=2 storvsc_ringbuffer_size=0x8000
[0.172873] underhill_core: INFO  VMM process crate_name="build_info" crate_revision="20bcbc74bb2e4d64d64b77436fd40f438c19d8c8" openhcl_version=""
[0.172912] underhill_core: INFO  boot loader times start=0x88647 end=0x9667a sidecar_start=0x88a59 sidecar_end=0x90658 elapsed=5.7395ms sidecar_elapsed=3.1743ms
[0.173181] diag_server: INFO  control starting control_address=VmAddress(Address { cid: ffffffff, port: 1 })
[0.173186] hv_vmbus: registering driver hv_storvsc
[0.173193] diag_server: INFO  data starting data_address=VmAddress(Address { cid: ffffffff, port: 2 })
[0.173235] underhill_init: load complete for /lib/modules/999/hv_storvsc.ko
[0.173277] mesh_tracing: INFO  updated trace filter filter="info,underhill_core::dispatch=debug,underhill_core::worker=debug"
[0.175333] mesh_tracing: INFO  updated trace filter filter="info,underhill_core::dispatch=debug,underhill_core::worker=debug"
[0.176134] guest_emulation_transport::process_loop: INFO  [GET] version negotiated protocol=NICKEL_REV2
[0.176206] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.176580] underhill_core::worker: DEBUG worker_new{ name="UnderhillWorker" action="new"}:init:init/dps:  device platform settings DevicePlatformSettings { smbios: Smbios { serial_number: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], base_board_serial_number: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], chassis_serial_number: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], chassis_asset_tag: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], system_manufacturer: [], system_product_name: [], system_version: [], system_sku_number: [], system_family: [], bios_lock_string: [], memory_device_serial_number: [], processor_manufacturer: [73, 110, 116, 1
[0.176583] underhill_core::worker: 01, 108, 0], processor_version: [73, 110, 116, 101, 108, 40, 82, 41, 32, 88, 101, 111, 110, 40, 82, 41, 32, 80, 108, 97, 116, 105, 110, 117, 109, 32, 56, 52, 54, 48, 89, 43, 0], processor_id: 0, external_clock: 16000, max_speed: 4000, current_speed: 2000, processor_characteristics: 252, processor_family2: 179, processor_type: 3, voltage: 144, status: 65, processor_upgrade: 6 }, general: General { secure_boot_enabled: false, secure_boot_template: MicrosoftWindows, bios_guid: ee949614-8319-40ba-acdd-481cb8109d7c, console_mode: Default, battery_enabled: false, processor_idle_enabled: false, tpm_enabled: false, com1_enabled: false, com1_debugger_mode: false, com1_vmbus_redirector: false, com2_enabled: false, com2_debugger_mode: false, com2_vmbus_redirector: false, firmware_debugging_enabled: false, hibernation_enabled: false, suppress_attestation: Some(false), generation_id: Some([70, 87, 40, 60, 253, 29, 184, 235, 60, 147, 147, 206, 109,
[0.176585] underhill_core::worker: 88, 191, 103]), legacy_memory_map: false, pause_after_boot_failure: false, pxe_ip_v6: false, measure_additional_pcrs: true, disable_frontpage: false, disable_sha384_pcr: false, media_present_enabled_by_default: false, vpci_boot_enabled: false, memory_protection_mode: Disabled, default_boot_always_attempt: false, num_lock_enabled: false, pcat_boot_device_order: [Optical, HardDrive, Network, Floppy], vpci_instance_filter: None, nvdimm_count: 0, psp_enabled: false, vmbus_redirection_enabled: true, always_relay_host_mmio: false, vtl2_settings: None, is_servicing_scenario: false, watchdog_enabled: false, firmware_mode_is_pcat: false, imc_enabled: false, cxl_memory_enabled: false, efi_diagnostics_log_level: DEFAULT, guest_state_lifetime: Default, guest_state_encryption_policy: None, management_vtl_features: ManagementVtlFeatures { strict_encryption_policy: false, load_firmware_supported: false, control_ak_cert_provisioning: false, attempt_
[0.176587] underhill_core::worker: ak_cert_callback: false, tx_only_serial_port: false }, force_dma_bounce_enabled: false, hardware_sealing_policy: None }, acpi_tables: [] }
[0.178798] underhill_core::worker: DEBUG worker_new{ name="UnderhillWorker" action="new"}:init:  restored servicing state from persisted memory saved_state_len=0x1e0c4
[0.181023] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  kernel boot time kernel_boot_time_ns=0x99a9c6d
[0.183061] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] read_from_dt: persisted_state_header present=false, sidecar=Enabled { enable_logging: false, cpu_threshold: Some(100) }"
[0.183068] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] reading topology from host device tree"
[0.183073] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] reclaim device tree memory 40a34f000-40a38f000"
[0.183079] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] persisted state region sized to MemoryRange {"
[0.183085] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="    start: 0x408000000,"
[0.183094] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="    end: 0x408040000,"
[0.183100] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="}, remainder MemoryRange {"
[0.183105] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="    start: 0x408040000,"
[0.183112] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="    end: 0x408200000,"
[0.183116] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="}"
[0.183122] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] vtl2 gpa pool coming from heuristics table: Release"
[0.183128] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] Extrapolated VTL2 DMA hint: 17408 pages (68 MiB) for 64 VPs and 1024 MiB VTL2 memory"
[0.183133] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] allocated VTL2 pool at MemoryRange {"
[0.183138] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="    start: 0x443c00000,"
[0.183143] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="    end: 0x448000000,"
[0.183148] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="}"
[0.183153] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] openhcl_boot: logging enabled"
[0.183158] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] serial configuration: None"
[0.183164] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] sidecar: created node index=0 base_vp=0 vp_count=32 vnode=0"
[0.183169] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] sidecar: created node index=1 base_vp=32 vp_count=32 vnode=0"
[0.183174] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] sidecar: reserved kexec regions MemoryRange {"
[0.183178] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="    start: 0x40a800000,"
[0.183183] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="    end: 0x40ac00000,"
[0.183188] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="} and MemoryRange {"
[0.183193] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="    start: 0x40ac00000,"
[0.183199] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="    end: 0x40b000000,"
[0.183204] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="}"
[0.183209] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] sidecar starting, 2 nodes, 64 cpus, 0x342000 total bytes"
[0.183213] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] sidecar: boot_cpus parameter: boot_cpus=0,32"
[0.183219] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] uninitializing hypercalls"
[0.183224] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  openhcl_boot log line="[INFO] about to jump to kernel"
[0.183477] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  memory allocation mode memory_allocation_mode=Host
[0.183485] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  vtl2 ram vtl2_ram="0x408000000-0x448000000"
[0.183493] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  overriding dps to enable default_boot_always_attempt
[0.183499] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  flush logs result duration_us=0xc9
[0.183856] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  vtl0 ram vtl0_ram="0x0-0xf8000000(0), 0x100000000-0x408000000(0)"
[0.183863] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  vtl0 mmio vtl0_mmio="0xf8000000-0x100000000, 0xfe0000000-0x1000000000"
[0.183869] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  shared pool shared_pool=""
[0.183900] vmgs::storage: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  vmgs_capacity=0x4000000
[0.183912] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  enabling alias map alias_map=0x200000000000
[0.184152] sidecar_client: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  sidecar client initialized with 2 node(s) layout=[0..20, 20..40]
[0.184172] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.184244] underhill_mem::init: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  Skipping VTL 1 memory map creation
[0.184256] openhcl_dma_manager: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  create dma manager shared_ranges=[] private_ranges=[MemoryRange { start: 443c00000, end: 448000000 }] vtom=0x0 isolation_type=None
[0.184272] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.184283] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  starting guest memory self test
[0.184302] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  guest memory self test complete
[0.185276] underhill_core::dispatch::vtl2_settings_worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:new_initial_controllers:  Initial VTL2 settings vtl2_settings=None config_timeout_in_seconds=0x5
[0.185338] underhill_core::nvme_manager::manager: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  starting nvme manager vp_count=0x40 save_restore_supported=true saved_state=true
[0.185420] underhill_core::nvme_manager::manager: INFO nvme_manager_restore:nvme_manager_worker_restore:  nvme manager worker restored 0 devices
[0.185453] underhill_core::worker: DEBUG worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  NVMe VFIO manager initialized, setting up resolver nvme_vfio=true save_restore_supported=true
[0.185931] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  Underhill MNF enabled? enable_mnf=false
[0.185964] vmbus_server::channels: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:  Limiting VmBus connections to version version=MaxVersionInfo { version: 60000, feature_flags: FeatureFlags { guest_specified_signal_parameters: true, channel_interrupt_redirection: true, modify_connection: true, client_id: false, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: true } }
[0.186029] vmbus_client::saved_state: INFO  channel restored key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 state=Opened
[0.186043] vmbus_client::saved_state: INFO  channel restored key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0 state=Offered
[0.186050] vmbus_client::saved_state: INFO  channel restored key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0 state=Opened
[0.186060] vmbus_client::saved_state: INFO  channel restored key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0 state=Opened
[0.186067] vmbus_client::saved_state: INFO  channel restored key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0 state=Opened
[0.186077] vmbus_client::saved_state: INFO  channel restored key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 state=Offered
[0.186083] vmbus_client::saved_state: INFO  channel restored key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 state=Opened
[0.186101] vmbus_client::saved_state: INFO  channel restored key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 state=Opened
[0.186108] vmbus_client::saved_state: INFO  channel restored key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 state=Offered
[0.186120] vmbus_client::saved_state: INFO  channel restored key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 state=Opened
[0.186126] vmbus_client::saved_state: INFO  channel restored key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 state=Offered
[0.186133] vmbus_client::saved_state: INFO  channel restored key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0 state=Opened
[0.186138] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0 state=Opened
[0.186144] vmbus_client::saved_state: INFO  channel restored key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 state=Opened
[0.186161] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-1 state=Opened
[0.186170] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-2 state=Opened
[0.186176] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-3 state=Opened
[0.186190] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-4 state=Opened
[0.186196] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-5 state=Opened
[0.186202] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-6 state=Opened
[0.186208] vmbus_client::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-7 state=Opened
[0.186278] vmbus_server::channels: INFO  new channel offer_id=OfferId(0) key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186307] vmbus_server::channels: INFO  new channel offer_id=OfferId(1) key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186331] vmbus_server::channels: INFO  new channel offer_id=OfferId(2) key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186358] vmbus_server::channels: INFO  new channel offer_id=OfferId(3) key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186388] vmbus_server::channels: INFO  new channel offer_id=OfferId(4) key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186409] vmbus_server::channels: INFO  new channel offer_id=OfferId(5) key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186432] vmbus_server::channels: INFO  new channel offer_id=OfferId(6) key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186456] vmbus_server::channels: INFO  new channel offer_id=OfferId(7) key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186486] vmbus_server::channels: INFO  new channel offer_id=OfferId(8) key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186505] vmbus_server::channels: INFO  new channel offer_id=OfferId(9) key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186525] vmbus_server::channels: INFO  new channel offer_id=OfferId(a) key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186551] vmbus_server::channels: INFO  new channel offer_id=OfferId(b) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186574] vmbus_server::channels: INFO  new channel offer_id=OfferId(c) key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.186594] vmbus_server::channels: INFO  new channel offer_id=OfferId(d) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-1 confidential_ring_buffer=false confidential_external_memory=false
[0.186615] vmbus_server::channels: INFO  new channel offer_id=OfferId(e) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-2 confidential_ring_buffer=false confidential_external_memory=false
[0.186648] vmbus_server::channels: INFO  new channel offer_id=OfferId(f) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-3 confidential_ring_buffer=false confidential_external_memory=false
[0.186674] vmbus_server::channels: INFO  new channel offer_id=OfferId(10) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-4 confidential_ring_buffer=false confidential_external_memory=false
[0.186702] vmbus_server::channels: INFO  new channel offer_id=OfferId(11) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-5 confidential_ring_buffer=false confidential_external_memory=false
[0.186722] vmbus_server::channels: INFO  new channel offer_id=OfferId(12) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-6 confidential_ring_buffer=false confidential_external_memory=false
[0.186746] vmbus_server::channels: INFO  new channel offer_id=OfferId(13) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-7 confidential_ring_buffer=false confidential_external_memory=false
[0.187453] vmbus_server::channels: INFO  new channel offer_id=OfferId(14) key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 confidential_ring_buffer=true confidential_external_memory=false
[0.187497] vmbus_relay_intercept_device: INFO  matching channel offered offer=OfferInfo { offer: OfferChannel { interface_id: 0e0b6031-5213-4934-818b-38d90ced39db, instance_id: b6650ff7-33bc-4840-8048-e0676786f393, rsvd: [0, 0, 0, 0], flags: OfferFlags { enumerate_device_interface: true, confidential_ring_buffer: false, confidential_external_memory: false, require_pinned_external_memory: false, named_pipe_mode: true, tlnpi_provider: false }, mmio_megabytes: 0, user_defined: UserDefinedData([040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000]), subchannel_index: 0, mmio_megabytes_optional: 0, channel_id: ChannelId(9), monitor_id: ff, monitor_allocated: 0, is_dedicated: 1, connection_id: 2009 }, guest_to_host_interrupt: Interrupt { inner: InterruptInner }, request_send: SenderCore(Ma
[0.187499] vmbus_relay_intercept_device: nuallyDrop { value: MaybeDangling(Queue { remote: OnceLock(<uninit>), local: Mutex { data: LocalQueue { messages: ErasedVecDeque { cap: 0, head: 0, len: 0 }, ports: [], waker: Some(Waker { data: 0x23a960e0390, vtable: 0x7f849519a8e8 }), remote: false, receiver_gone: false, remove_closed: false, new_handler: 0x7f849427ab70 } }, receiver: UnsafeCell { .. } }) }), revoke_recv: ManuallyDrop { value: MaybeDangling(OneshotReceiverCore(Slot { state: Mutex { data: Waiting(None) }, receiver: UnsafeCell { .. } })) } }
[0.187791] vmbus_client: INFO  opening channel on host channel_id=0x9 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[0.194882] hyperv_ic_guest::shutdown: INFO  version negotiated framework_version=3.0 message_version=3.2
[0.195259] vmbus_server::channels::saved_state: INFO  channel restored key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 state=Open
[0.195268] vmbus_server::channels::saved_state: INFO  channel restored key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0 state=Closed
[0.195274] vmbus_server::channels::saved_state: INFO  channel restored key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0 state=Open
[0.195279] vmbus_server::channels::saved_state: INFO  channel restored key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0 state=Open
[0.195285] vmbus_server::channels::saved_state: INFO  channel restored key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0 state=Open
[0.195291] vmbus_server::channels::saved_state: INFO  channel restored key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 state=Closed
[0.195296] vmbus_server::channels::saved_state: INFO  channel restored key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 state=Open
[0.195301] vmbus_server::channels::saved_state: INFO  channel restored key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 state=Open
[0.195307] vmbus_server::channels::saved_state: INFO  channel restored key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 state=Open
[0.195312] vmbus_server::channels::saved_state: INFO  channel restored key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 state=Closed
[0.195317] vmbus_server::channels::saved_state: INFO  channel restored key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0 state=Open
[0.195323] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0 state=Open
[0.195328] vmbus_server::channels::saved_state: INFO  channel restored key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 state=Open
[0.195334] vmbus_server::channels::saved_state: INFO  channel restored key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 state=Open
[0.195339] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-1 state=Open
[0.195345] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-2 state=Open
[0.195350] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-3 state=Open
[0.195355] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-4 state=Open
[0.195361] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-5 state=Open
[0.195366] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-6 state=Open
[0.195372] vmbus_server::channels::saved_state: INFO  channel restored key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-7 state=Open
[0.217683] state_unit: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/restore{ correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8}:restore_units:state_change{ operation="restore"}:  state change complete duration=24.195401ms
[0.217898] underhill_core: INFO  vm worker started
[0.221252] state_unit: INFO state_change{ operation="start"}:  state change complete duration=3.383517ms
[0.221274] underhill_core::dispatch: INFO  resuming VM correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8 blackout_time_ms=0x158 blackout_time="344.605ms"
[0.221292] underhill_core::dispatch: INFO  servicing blackout phases correlation_id=ef8dc73b-7fd8-4105-af53-e7e6c96708d8 method="kexec" total=Some(344.605ms) phases=Some([("stop", 1.5913ms), ("save", 2.4851ms), ("shutdown", 293.3<c2><b5>s), ("flush", 214.9<c2><b5>s), ("handoff_kernel_init", 294.9701ms), ("settings", 428.1<c2><b5>s), ("state_read", 2.1681ms), ("state_fixup", 2.1623ms), ("vm_build", 12.4446ms), ("restore", 24.2952ms), ("restore_notify", 2.5<c2><b5>s), ("resume_wait", 79.6<c2><b5>s), ("start", 3.4699ms)])
[249.071896] virt_mshv_vtl::processor: INFO  sidecar exit
[249.071910] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Io { port: 28, write: true })
[249.072510] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x29
[249.073291] underhill_core::vp: INFO  onlining sidecar VP cpu=0x29
[249.149146] mshv_vtl_sidecar 40aa21000.sidecar: cpu 41: removing from sidecar
[249.158872] mshv_vtl_sidecar 40aa21000.sidecar: cpu 41: remove complete
[249.158880] smpboot: Booting Node 0 Processor 41 APIC 0x29
