[0.000000] Linux version 6.18.37-microsoft-hcl+ (hargar@UbuntuVM-22) (gcc (Ubuntu 11.4.0-1ubuntu1~22.04.3) 11.4.0, GNU ld (GNU Binutils for Ubuntu) 2.38) #1 SMP PREEMPT_DYNAMIC Mon Oct  5 21:26:43 UTC 2026
[0.000000] Command line: loglevel=8 log_buf_len=128K printk.time=1 console_msg_format=syslog uio_hv_generic.no_mask=1 coredump_filter=0x33 cpufreq.off=1 cpuidle.off=1 cryptomgr.notests idle=halt initcall_blacklist=init_real_mode,sbf_init lpj=3000000 no_timer_check noxsave oops=panic panic_on_warn=0 panic_print=0 panic=-1 printk.devkmsg=on reboot=t rootfstype=tmpfs sysctl.vm.compaction_proactiveness=0 tsc=reliable unknown_nmi_panic=1 vfio_pci.ids=1414:00ba vfio.enable_unsafe_noiommu_mode=1 rdinit=/underhill-init OPENHCL_NVME_VFIO=1 hv_storvsc.storvsc_vcpus_per_sub_channel=2048 hv_storvsc.storvsc_max_hw_queues=2 hv_storvsc.storvsc_ringbuffer_size=0x8000 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off iommu=off pci=off swiotlb=1,1 console=ttynull OPENHCL_NVME_KEEP_ALIVE=host,privatepool boot_cpus=0,32 hv_vmbus.message_connection_id=0x800074 OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug RUST_BACKTRACE=full
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
[0.000015] last_pfn = 0x0 max_arch_pfn = 0x1000000000
[0.000019] Using GB pages for direct mapping
[0.000509] printk: log buffer data + meta data: 131072 + 458752 = 589824 bytes
[0.000510] printk: early log buf free: 12880(78%)
[0.000510] RAMDISK: [mem 0x408f09000-0x40973cfff]
[0.000575] OF: reserved mem: Reserved memory: No reserved-memory node in the DT
[0.000768] NODE_DATA(0) allocated [mem 0x443b51380-0x443b55bff]
[0.000805] Zone ranges:
[0.000806] DMA      empty
[0.000807] DMA32    empty
[0.000808] Normal   [mem 0x0000000408040000-0x0000000443bfffff]
[0.000810] Device   empty
[0.000810] Movable zone start for each node
[0.000811] Early memory node ranges
[0.000811] node   0: [mem 0x0000000408040000-0x0000000408dfffff]
[0.000812] node   0: [mem 0x0000000408e1a000-0x000000040a322fff]
[0.000813] node   0: [mem 0x000000040a34f000-0x000000040a38efff]
[0.000813] node   0: [mem 0x000000040a493000-0x000000040a4a9fff]
[0.000814] node   0: [mem 0x000000040a7ec000-0x000000040a7fffff]
[0.000814] node   0: [mem 0x000000040b000000-0x0000000443bfffff]
[0.000815] Initmem setup node 0 [mem 0x0000000408040000-0x0000000443bfffff]
[0.000841] On node 0, zone Normal: 64 pages in unavailable ranges
[0.000875] On node 0, zone Normal: 26 pages in unavailable ranges
[0.000876] On node 0, zone Normal: 44 pages in unavailable ranges
[0.000878] On node 0, zone Normal: 260 pages in unavailable ranges
[0.000885] On node 0, zone Normal: 834 pages in unavailable ranges
[0.002283] On node 0, zone Normal: 2048 pages in unavailable ranges
[0.002403] On node 0, zone Normal: 17408 pages in unavailable ranges
[0.002432] IMCR and PIC compatibility mode.
[0.002454] TSC deadline timer available
[0.002458] CPU topo: Max. logical packages:   1
[0.002458] CPU topo: Max. logical dies:       1
[0.002459] CPU topo: Max. dies per package:   1
[0.002461] CPU topo: Max. threads per core:   2
[0.002462] CPU topo: Num. cores per package:    32
[0.002463] CPU topo: Num. threads per package:  64
[0.002463] CPU topo: Allowing 64 present CPUs plus 0 hotplug CPUs
[0.002469] Cannot find an available gap in the 32-bit address range
[0.002469] PCI devices with unassigned 32-bit BARs may not work!
[0.002469] [mem 0x443d00000-0x4440fffff] available for PCI devices
[0.002470] Booting paravirtualized kernel on Hyper-V
[0.002475] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
[0.002482] setup_percpu: NR_CPUS:2048 nr_cpumask_bits:64 nr_cpu_ids:64 nr_node_ids:1
[0.004734] percpu: Embedded 41 pages/cpu s127680 r8192 d32064 u262144
[0.004738] pcpu-alloc: s127680 r8192 d32064 u262144 alloc=1*2097152
[0.004740] pcpu-alloc: [0] 00 01 02 03 04 05 06 07 [0] 08 09 10 11 12 13 14 15
[0.004746] pcpu-alloc: [0] 16 17 18 19 20 21 22 23 [0] 24 25 26 27 28 29 30 31
[0.004750] pcpu-alloc: [0] 32 33 34 35 36 37 38 39 [0] 40 41 42 43 44 45 46 47
[0.004753] pcpu-alloc: [0] 48 49 50 51 52 53 54 55 [0] 56 57 58 59 60 61 62 63
[0.004771] Hyper-V: PV spinlocks enabled
[0.004773] PV qspinlock hash table entries: 256 (order: 0, 4096 bytes, linear)
[0.004778] Kernel command line: loglevel=8 log_buf_len=128K printk.time=1 console_msg_format=syslog uio_hv_generic.no_mask=1 coredump_filter=0x33 cpufreq.off=1 cpuidle.off=1 cryptomgr.notests idle=halt initcall_blacklist=init_real_mode,sbf_init lpj=3000000 no_timer_check noxsave oops=panic panic_on_warn=0 panic_print=0 panic=-1 printk.devkmsg=on reboot=t rootfstype=tmpfs sysctl.vm.compaction_proactiveness=0 tsc=reliable unknown_nmi_panic=1 vfio_pci.ids=1414:00ba vfio.enable_unsafe_noiommu_mode=1 rdinit=/underhill-init OPENHCL_NVME_VFIO=1 hv_storvsc.storvsc_vcpus_per_sub_channel=2048 hv_storvsc.storvsc_max_hw_queues=2 hv_storvsc.storvsc_ringbuffer_size=0x8000 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off iommu=off pci=off swiotlb=1,1 console=ttynull OPENHCL_NVME_KEEP_ALIVE=host,privatepool boot_cpus=0,32 hv_vmbus.message_connection_id=0x800074 OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug RUST_BACKTRACE=full
[0.004826] blacklisting initcall init_real_mode
[0.004828] blacklisting initcall sbf_init
[0.004843] Kernel: The 'panic_print' parameter is now deprecated. Please use 'panic_sys_info' and 'panic_console_replay' instead.
[0.004928] Unknown kernel command line parameters "OPENHCL_NVME_VFIO=1 MIMALLOC_ARENA_EAGER_COMMIT=0 acpi=off OPENHCL_NVME_KEEP_ALIVE=host,privatepool OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release UNDERHILL_DIAG=1 HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug RUST_BACKTRACE=full", will be passed to user space.
[0.004940] random: crng init done
[0.005040] Dentry cache hash table entries: 131072 (order: 8, 1048576 bytes, linear)
[0.005090] Inode-cache hash table entries: 65536 (order: 7, 524288 bytes, linear)
[0.005907] Fallback order for Node 0: 0
[0.005911] Built 1 zonelists, mobility grouping on.  Total pages: 241460
[0.005912] Policy zone: Normal
[0.005913] mem auto-init: stack:off, heap alloc:off, heap free:off
[0.007499] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=64, Nodes=1
[0.007897] Dynamic Preempt: none
[0.008153] rcu: Preemptible hierarchical RCU implementation.
[0.008153] rcu: <09>RCU restricting CPUs from NR_CPUS=2048 to nr_cpu_ids=64.
[0.008155] rcu: RCU calculated value of scheduler-enlistment delay is 10 jiffies.
[0.008156] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=64
[0.008217] Using NULL legacy PIC
[0.008218] NR_IRQS: 131328, nr_irqs: 1040, preallocated irqs: 0
[0.008277] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[0.008280] Persistent clock returned invalid value
[0.008411] APIC: Switch to symmetric I/O mode setup
[0.008421] x2apic enabled
[0.008453] APIC: Switched APIC routing to: physical x2apic
[0.008457] Hyper-V: Host Build 10.0.29667.1001-1-0
[0.008458] Hyper-V: enabling crash_kexec_post_notifiers
[0.008585] Hyper-V: Using IPI hypercalls
[0.008587] APIC: send_IPI() replaced with hv_send_ipi()
[0.008594] APIC: send_IPI_mask() replaced with hv_send_ipi_mask()
[0.008595] APIC: send_IPI_mask_allbutself() replaced with hv_send_ipi_mask_allbutself()
[0.008597] APIC: send_IPI_allbutself() replaced with hv_send_ipi_allbutself()
[0.008599] APIC: send_IPI_all() replaced with hv_send_ipi_all()
[0.008600] APIC: send_IPI_self() replaced with hv_send_ipi_self()
[0.008660] APIC: wakeup_secondary_cpu_64() replaced with hv_vtl_wakeup_secondary_cpu()
[0.008680] clocksource: tsc-early: mask: 0xffffffffffffffff max_cycles: 0x39a85c9bff6, max_idle_ns: 881590591483 ns
[0.008685] Calibrating delay loop (skipped) preset value.. 600.00 BogoMIPS (lpj=3000000)
[0.008757] x86/cpu: User Mode Instruction Prevention (UMIP) activated
[0.008762] Last level iTLB entries: 4KB 0, 2MB 0, 4MB 0
[0.008763] Last level dTLB entries: 4KB 0, 2MB 0, 4MB 0, 1GB 0
[0.008771] mitigations: Enabled attack vectors: user_kernel, user_user, SMT mitigations: auto
[0.008774] Speculative Store Bypass: Vulnerable
[0.008775] Spectre V2 : Vulnerable
[0.008775] RETBleed: Vulnerable
[0.008776] Spectre V2 : User space: Vulnerable
[0.008776] Spectre V1 : Vulnerable: __user pointer sanitization and usercopy barriers only; no swapgs barriers
[0.008779] x86/fpu: x87 FPU will use FXSAVE
[0.010201] Freeing SMP alternatives memory: 20K
[0.010204] pid_max: default: 65536 minimum: 512
[0.010328] Mount-cache hash table entries: 2048 (order: 2, 16384 bytes, linear)
[0.010332] Mountpoint-cache hash table entries: 2048 (order: 2, 16384 bytes, linear)
[0.010585] smpboot: CPU0: Intel(R) Xeon(R) Platinum 8460Y+ (family: 0x6, model: 0x8f, stepping: 0x8)
[0.010747] Performance Events: unsupported CPU family 6 model 143 no PMU driver, software events only.
[0.010754] signal: max sigframe size: 1040
[0.010788] rcu: Hierarchical SRCU implementation.
[0.010789] rcu: <09>Max phase no-delay instances is 1000.
[0.010828] Timer migration: 2 hierarchy levels; 8 children per group; 2 crossnode level
[0.011166] smp: Bringing up secondary CPUs ...
[0.011241] smpboot: x86: Booting SMP configuration:
[0.011242] .... node  #0, CPUs:       #32
[0.018683] smp: Brought up 1 node, 2 CPUs
[0.018683] smpboot: Total of 2 processors activated (1200.00 BogoMIPS)
[0.018683] Memory: 911732K/965840K available (4421K kernel code, 480K rwdata, 1272K rodata, 920K init, 584K bss, 48156K reserved, 0K cma-reserved)
[0.018683] devtmpfs: initialized
[0.018683] x86/mm: Memory block size: 128MB
[0.018683] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
[0.018683] posixtimers hash table entries: 32768 (order: 7, 524288 bytes, linear)
[0.018683] futex hash table entries: 16384 (1048576 bytes on 1 NUMA nodes, total 1024 KiB, linear).
[0.018683] NET: Registered PF_NETLINK/PF_ROUTE protocol family
[0.018683] initcall sbf_init blacklisted
[0.018683] PCI: Fatal: No config space access function found
[0.018683] iommu: Default domain type: Translated
[0.018683] iommu: DMA domain TLB invalidation policy: lazy mode
[0.018683] SCSI subsystem initialized
[0.018683] hv_vmbus: VMBus is present in DeviceTree
[0.018683] hv_vmbus: Vmbus version:5.3
[0.018683] PCI: System does not support PCI
[0.018683] clocksource: Switched to clocksource tsc-early
[0.018683] NET: Registered PF_UNIX/PF_LOCAL protocol family
[0.018683] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x39a85c9bff6, max_idle_ns: 881590591483 ns
[0.018683] clocksource: Switched to clocksource tsc
[0.018683] Unpacking initramfs...
[0.029820] workingset: timestamp_bits=56 max_order=18 bucket_order=0
[0.029920] io scheduler mq-deadline registered
[0.030241] Serial: 8250/16550 driver, 4 ports, IRQ sharing enabled
[0.031362] printk: legacy console [ttynull0] enabled
[0.031972] hv_vmbus: registering driver uio_hv_generic
[0.038707] VFIO - User Level meta-driver version: 0.3
[0.038729] vfio_pci: add [1414:00ba[ffffffff:ffffffff]] class 0x000000/00000000
[0.039007] NET: Registered PF_VSOCK protocol family
[0.039009] hv_vmbus: registering driver hv_sock
[0.039085] IPI shorthand broadcast: enabled
[0.039560] sched_clock: Marking stable (30135488, 8683947)->(46379372, -7559937)
[0.040324] Demotion targets for Node 0: null
[0.040642] clk: Disabling unused clocks
[0.130748] Freeing initrd memory: 8400K
[0.131052] Freeing unused kernel image (initmem) memory: 920K
[0.131055] Write protecting the kernel read-only data: 8192k
[0.131687] Freeing unused kernel image (text/rodata gap) memory: 1720K
[0.131968] Freeing unused kernel image (rodata/data gap) memory: 776K
[0.132075] Freeing unused kernel image (bss_decrypted gap) memory: 332K
[0.168694] Run /underhill-init as init process
[0.168695] with arguments:
[0.168695] /underhill-init
[0.168696] with environment:
[0.168696] HOME=/
[0.168697] TERM=linux
[0.168697] OPENHCL_NVME_VFIO=1
[0.168697] MIMALLOC_ARENA_EAGER_COMMIT=0
[0.168698] acpi=off
[0.168698] OPENHCL_NVME_KEEP_ALIVE=host,privatepool
[0.168699] OPENHCL_IGVM_VTL2_GPA_POOL_CONFIG=release
[0.168699] UNDERHILL_DIAG=1
[0.168700] HVLITE_LOG=info,underhill_core::dispatch=debug,underhill_core::worker=debug
[0.168700] RUST_BACKTRACE=full
[0.169887] underhill_init: Initial process: crate_name=underhill_init, crate_revision=20bcbc74bb2e4d64d64b77436fd40f438c19d8c8, crate_branch=user/hargar/kexec-2026-6.18-igvm-clean
[0.169899] underhill_init: Mounting filesystems
[0.169984] underhill_init: Command line args: Options { setup_script: [], underhill_args: [] }
[0.169985] underhill_init: Setting system resource limits and parameters
[0.170278] underhill_init: Using 64 bytes of entropy from the host
[0.170296] underhill_init: registered vfio-pci as driver for nvme
[0.170355] underhill_init: running KERNEL_BOOT_TIME="161167723" "/bin/openvmm_hcl" "--pid" "/run/underhill.pid"
[0.170525] underhill_init: loading kernel module /lib/modules/000/pci-hyperv-intf.ko:
[0.171340] underhill_init: load complete for /lib/modules/000/pci-hyperv-intf.ko
[0.171362] underhill_init: loading kernel module /lib/modules/001/pci-hyperv.ko:
[0.172215] hv_vmbus: registering driver hv_pci
[0.172275] underhill_init: load complete for /lib/modules/001/pci-hyperv.ko
[0.172297] underhill_init: loading kernel module /lib/modules/999/hv_storvsc.ko: storvsc_vcpus_per_sub_channel=2048 storvsc_max_hw_queues=2 storvsc_ringbuffer_size=0x8000
[0.173063] hv_vmbus: registering driver hv_storvsc
[0.173075] underhill_core: INFO  VMM process crate_name="build_info" crate_revision="20bcbc74bb2e4d64d64b77436fd40f438c19d8c8" openhcl_version=""
[0.173109] underhill_init: load complete for /lib/modules/999/hv_storvsc.ko
[0.173121] underhill_core: INFO  boot loader times start=0x88647 end=0x9667a sidecar_start=0x88a59 sidecar_end=0x90658 elapsed=5.7395ms sidecar_elapsed=3.1743ms
[0.173349] diag_server: INFO  control starting control_address=VmAddress(Address { cid: ffffffff, port: 1 })
[0.173362] diag_server: INFO  data starting data_address=VmAddress(Address { cid: ffffffff, port: 2 })
[0.173437] mesh_tracing: INFO  updated trace filter filter="info,underhill_core::dispatch=debug,underhill_core::worker=debug"
[0.175529] mesh_tracing: INFO  updated trace filter filter="info,underhill_core::dispatch=debug,underhill_core::worker=debug"
[0.176569] guest_emulation_transport::process_loop: INFO  [GET] version negotiated protocol=NICKEL_REV2
[0.176630] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.177187] underhill_core::worker: DEBUG worker_new{ name="UnderhillWorker" action="new"}:init:init/dps:  device platform settings DevicePlatformSettings { smbios: Smbios { serial_number: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], base_board_serial_number: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], chassis_serial_number: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], chassis_asset_tag: [49, 49, 56, 54, 45, 49, 55, 56, 51, 45, 54, 48, 54, 54, 45, 48, 55, 54, 56, 45, 48, 49, 53, 51, 45, 49, 57, 52, 52, 45, 53, 57], system_manufacturer: [], system_product_name: [], system_version: [], system_sku_number: [], system_family: [], bios_lock_string: [], memory_device_serial_number: [], processor_manufacturer: [73, 110, 116, 1
[0.177190] underhill_core::worker: 01, 108, 0], processor_version: [73, 110, 116, 101, 108, 40, 82, 41, 32, 88, 101, 111, 110, 40, 82, 41, 32, 80, 108, 97, 116, 105, 110, 117, 109, 32, 56, 52, 54, 48, 89, 43, 0], processor_id: 0, external_clock: 16000, max_speed: 4000, current_speed: 2000, processor_characteristics: 252, processor_family2: 179, processor_type: 3, voltage: 144, status: 65, processor_upgrade: 6 }, general: General { secure_boot_enabled: false, secure_boot_template: MicrosoftWindows, bios_guid: ee949614-8319-40ba-acdd-481cb8109d7c, console_mode: Default, battery_enabled: false, processor_idle_enabled: false, tpm_enabled: false, com1_enabled: false, com1_debugger_mode: false, com1_vmbus_redirector: false, com2_enabled: false, com2_debugger_mode: false, com2_vmbus_redirector: false, firmware_debugging_enabled: false, hibernation_enabled: false, suppress_attestation: Some(false), generation_id: Some([70, 87, 40, 60, 253, 29, 184, 235, 60, 147, 147, 206, 109,
[0.177192] underhill_core::worker: 88, 191, 103]), legacy_memory_map: false, pause_after_boot_failure: false, pxe_ip_v6: false, measure_additional_pcrs: true, disable_frontpage: false, disable_sha384_pcr: false, media_present_enabled_by_default: false, vpci_boot_enabled: false, memory_protection_mode: Disabled, default_boot_always_attempt: false, num_lock_enabled: false, pcat_boot_device_order: [Optical, HardDrive, Network, Floppy], vpci_instance_filter: None, nvdimm_count: 0, psp_enabled: false, vmbus_redirection_enabled: true, always_relay_host_mmio: false, vtl2_settings: None, is_servicing_scenario: false, watchdog_enabled: false, firmware_mode_is_pcat: false, imc_enabled: false, cxl_memory_enabled: false, efi_diagnostics_log_level: DEFAULT, guest_state_lifetime: Default, guest_state_encryption_policy: None, management_vtl_features: ManagementVtlFeatures { strict_encryption_policy: false, load_firmware_supported: false, control_ak_cert_provisioning: false, attempt_
[0.177194] underhill_core::worker: ak_cert_callback: false, tx_only_serial_port: false }, force_dma_bounce_enabled: false, hardware_sealing_policy: None }, acpi_tables: [] }
[0.177310] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  kernel boot time kernel_boot_time_ns=0x99b396b
[0.180270] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] read_from_dt: persisted_state_header present=false, sidecar=Enabled { enable_logging: false, cpu_threshold: Some(100) }"
[0.180280] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] reading topology from host device tree"
[0.180286] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] reclaim device tree memory 40a34f000-40a38f000"
[0.180291] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] persisted state region sized to MemoryRange {"
[0.180296] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    start: 0x408000000,"
[0.180301] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    end: 0x408040000,"
[0.180306] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="}, remainder MemoryRange {"
[0.180311] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    start: 0x408040000,"
[0.180315] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    end: 0x408200000,"
[0.180320] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="}"
[0.180325] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] vtl2 gpa pool coming from heuristics table: Release"
[0.180330] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] Extrapolated VTL2 DMA hint: 17408 pages (68 MiB) for 64 VPs and 1024 MiB VTL2 memory"
[0.180334] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] allocated VTL2 pool at MemoryRange {"
[0.180339] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    start: 0x443c00000,"
[0.180344] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    end: 0x448000000,"
[0.180348] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="}"
[0.180353] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] openhcl_boot: logging enabled"
[0.180357] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] serial configuration: None"
[0.180362] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] sidecar: created node index=0 base_vp=0 vp_count=32 vnode=0"
[0.180367] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] sidecar: created node index=1 base_vp=32 vp_count=32 vnode=0"
[0.180372] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] sidecar: reserved kexec regions MemoryRange {"
[0.180377] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    start: 0x40a800000,"
[0.180382] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    end: 0x40ac00000,"
[0.180386] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="} and MemoryRange {"
[0.180391] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    start: 0x40ac00000,"
[0.180395] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="    end: 0x40b000000,"
[0.180400] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="}"
[0.180405] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] sidecar starting, 2 nodes, 64 cpus, 0x342000 total bytes"
[0.180411] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] sidecar: boot_cpus parameter: boot_cpus=0,32"
[0.180415] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] uninitializing hypercalls"
[0.180419] underhill_core::loader::vtl2_config: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  openhcl_boot log line="[INFO] about to jump to kernel"
[0.180664] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  memory allocation mode memory_allocation_mode=Host
[0.180670] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  vtl2 ram vtl2_ram="0x408000000-0x448000000"
[0.180679] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  overriding dps to enable default_boot_always_attempt
[0.181055] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  vtl0 ram vtl0_ram="0x0-0xf8000000(0), 0x100000000-0x408000000(0)"
[0.181103] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  vtl0 mmio vtl0_mmio="0xf8000000-0x100000000, 0xfe0000000-0x1000000000"
[0.181124] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  shared pool shared_pool=""
[0.181361] vmgs::storage: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:vmgs_open:  vmgs_capacity=0x4000000
[0.181975] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  enabling alias map alias_map=0x200000000000
[0.182231] sidecar_client: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  sidecar client initialized with 2 node(s) layout=[0..20, 20..40]
[0.182250] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.182390] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.234608] underhill_mem::init: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  VTL 1 memory map created
[0.234622] openhcl_dma_manager: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  create dma manager shared_ranges=[] private_ranges=[MemoryRange { start: 443c00000, end: 448000000 }] vtom=0x0 isolation_type=None
[0.234637] misc mshv_hvcall: Hypercall allow map has been set, pid 54
[0.234644] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  starting guest memory self test
[0.234657] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  guest memory self test complete
[0.235293] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Reading security profile tee_type=None secure_boot=false tpm_enabled=false tpm_persisted=true hardware_sealing_supported=false hardware_sealing_policy=Hash
[0.235300] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Reading VM ID from VMGS
[0.235306] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  attempt to unlock VMGS file attempt=0x0
[0.235313] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Key-encryption key retrieval not required
[0.235318] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Successfully retrieved key-encryption key
[0.235323] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Reading key protector from VMGS dek_minimal_size=0x100
[0.235331] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Deriving keys key_derivation_policy=None vmgs_encrypted=false op_type=BeginDecryptVmgs
[0.235338] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  encryption policy guest_state_encryption_policy=None strict_encryption_policy=false
[0.235344] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  initial vmgs encryption state is_encrypted=false is_gsp_by_id=false is_gsp=false found_dek=false
[0.235349] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  attempting GSP
[0.235356] guest_emulation_transport::client: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Getting guest state protection data op_type=BeginGspCallback
[0.235931] guest_emulation_transport::client: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Got guest state protection data op_type=GspCallback latency=0
[0.235939] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  GSP response request_data_length_in_vmgs=0x0 no_rpc_server=true requires_rpc_server=false encrypted_gsp_length=0x0
[0.235944] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  attempting GSP By Id
[0.236076] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Encryption sources kek=false gsp_available=false gsp=false gsp_by_id_available=Some(false) gsp_by_id=false hw_sealing=false
[0.236081] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  No VMGS encryption used.
[0.236086] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Unlocking VMGS
[0.236091] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Encryption disabled, skipping unlock vmgs data store
[0.236097] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  Unlocked datastore op_type=DecryptVmgs success=true decrypt_gsp_type=None encrypt_gsp_type=None latency=0
[0.236103] underhill_attestation: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:initialize_platform_security:  determine if refreshing tpm seeds is needed state_refresh_request_from_gsp=false vm_id_changed=false
[0.236468] underhill_core::dispatch::vtl2_settings_worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:new_initial_controllers:  Initial VTL2 settings vtl2_settings=None config_timeout_in_seconds=0x5
[0.236500] underhill_core::nvme_manager::manager: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  starting nvme manager vp_count=0x40 save_restore_supported=true saved_state=false
[0.236510] underhill_core::worker: DEBUG worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  NVMe VFIO manager initialized, setting up resolver nvme_vfio=true save_restore_supported=true
[0.237720] underhill_core::emuplat::local_clock: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  no saved skew found: defaulting to host local time skew=-25200s
[0.242047] firmware_uefi::service::nvram: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:base_chipset_build:add_device{ device="uefi"}:  configuring secure boot enabled=false
[0.245060] underhill_core::worker: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  Underhill MNF enabled? enable_mnf=false
[0.245092] vmbus_server::channels: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:  Limiting VmBus connections to version version=MaxVersionInfo { version: 60000, feature_flags: FeatureFlags { guest_specified_signal_parameters: true, channel_interrupt_redirection: true, modify_connection: true, client_id: false, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: true } }
[0.245400] vmbus_client: INFO  VmBus client connected, requesting offers version=VersionInfo { version: Copper, feature_flags: FeatureFlags { guest_specified_signal_parameters: true, channel_interrupt_redirection: true, modify_connection: true, client_id: true, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: false } }
[0.245474] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x1 interface_id=525074dc-8985-46e2-8057-a307dc18a502 instance_id=1eccfd72-4b41-45ef-b73a-4a6e44c12924 subchannel_index=0x0
[0.245496] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x2 interface_id=f8e65716-3cb3-4a06-9a60-1889c5cccab5 instance_id=99221fa0-24ad-11e2-be98-001aa01bbf6e subchannel_index=0x0
[0.245516] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x3 interface_id=cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a instance_id=58f75a6d-d949-4320-99e1-a2a2576d581c subchannel_index=0x0
[0.245537] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x4 interface_id=f912ad6d-2b17-48ea-bd65-f927a61c7684 instance_id=d34b2567-b9b6-42b9-8778-0a4ec0b955bf subchannel_index=0x0
[0.245552] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x5 interface_id=da0a7802-e377-4aac-8e77-0558eb1073f8 instance_id=5620e0c7-8062-4dce-aeb7-520c7ef76171 subchannel_index=0x0
[0.245572] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x6 interface_id=3375baf4-9e15-4b30-b765-67acb10d607b instance_id=4487b255-b88c-403f-bb51-d1f69cf17f87 subchannel_index=0x0
[0.245587] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x7 interface_id=57164f39-9115-4e78-ab55-382f3bd5422d instance_id=fd149e91-82e0-4a7d-afa6-2a4166cbd7c0 subchannel_index=0x0
[0.245608] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x8 interface_id=a9a0f4e7-5a45-4d96-b827-8a841e8c03e6 instance_id=242ff919-07db-4180-9c2e-b86cb68c8c55 subchannel_index=0x0
[0.245622] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0x9 interface_id=0e0b6031-5213-4934-818b-38d90ced39db instance_id=b6650ff7-33bc-4840-8048-e0676786f393 subchannel_index=0x0
[0.245641] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0xa interface_id=35fa2e29-ea23-4236-96ae-3a6ebacba440 instance_id=2450ee40-33bf-4fbd-892e-9fb06e9214cf subchannel_index=0x0
[0.245655] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0xb interface_id=276aacf4-ac15-426c-98dd-7521ad3f01fe instance_id=f5bee29c-1741-4aad-a4c2-8fdedb46dcc2 subchannel_index=0x0
[0.245670] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0xc interface_id=ba6163d9-04a1-4d29-b605-72e2ffb1dc7f instance_id=31e26bcb-1173-4589-9021-6e886ddc520d subchannel_index=0x0
[0.245685] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0xd interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=50f05d00-56d3-42c7-bf23-407a92a441cf subchannel_index=0x0
[0.245700] vmbus_client: INFO  received offer state=RequestingOffers channel_id=0xe interface_id=9527e630-d0ae-497b-adce-e80ab0175caf instance_id=2dd1ce17-079e-403c-b352-a1921ee207ee subchannel_index=0x0
[0.245709] vmbus_client: INFO  VmBus client connected, offers delivered version=VersionInfo { version: Copper, feature_flags: FeatureFlags { guest_specified_signal_parameters: true, channel_interrupt_redirection: true, modify_connection: true, client_id: true, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: false } }
[0.245810] vmbus_server::channels: INFO  new channel offer_id=OfferId(0) key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.245902] vmbus_server::channels: INFO  new channel offer_id=OfferId(1) key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.245945] vmbus_server::channels: INFO  new channel offer_id=OfferId(2) key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.245981] vmbus_server::channels: INFO  new channel offer_id=OfferId(3) key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.246049] vmbus_server::channels: INFO  new channel offer_id=OfferId(4) key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.246067] vmbus_server::channels: INFO  new channel offer_id=OfferId(5) key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.246085] vmbus_server::channels: INFO  new channel offer_id=OfferId(6) key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.246120] vmbus_server::channels: INFO  new channel offer_id=OfferId(7) key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.246232] vmbus_server::channels: INFO  new channel offer_id=OfferId(8) key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.246251] vmbus_server::channels: INFO  new channel offer_id=OfferId(9) key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.246270] vmbus_server::channels: INFO  new channel offer_id=OfferId(a) key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.246350] vmbus_server::channels: INFO  new channel offer_id=OfferId(b) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.246389] vmbus_server::channels: INFO  new channel offer_id=OfferId(c) key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 confidential_ring_buffer=false confidential_external_memory=false
[0.247086] vmbus_server::channels: INFO  new channel offer_id=OfferId(d) key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 confidential_ring_buffer=true confidential_external_memory=false
[0.247141] vmbus_relay_intercept_device: INFO  matching channel offered offer=OfferInfo { offer: OfferChannel { interface_id: 0e0b6031-5213-4934-818b-38d90ced39db, instance_id: b6650ff7-33bc-4840-8048-e0676786f393, rsvd: [0, 0, 0, 0], flags: OfferFlags { enumerate_device_interface: true, confidential_ring_buffer: false, confidential_external_memory: false, require_pinned_external_memory: false, named_pipe_mode: true, tlnpi_provider: false }, mmio_megabytes: 0, user_defined: UserDefinedData([040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000]), subchannel_index: 0, mmio_megabytes_optional: 0, channel_id: ChannelId(9), monitor_id: ff, monitor_allocated: 0, is_dedicated: 1, connection_id: 2009 }, guest_to_host_interrupt: Interrupt { inner: InterruptInner }, request_send: SenderCore(Ma
[0.247144] vmbus_relay_intercept_device: nuallyDrop { value: MaybeDangling(Queue { remote: OnceLock(<uninit>), local: Mutex { data: LocalQueue { messages: ErasedVecDeque { cap: 0, head: 0, len: 0 }, ports: [], waker: Some(Waker { data: 0x5316e0e03a0, vtable: 0x7f0e8b6cd8e8 }), remote: false, receiver_gone: false, remove_closed: false, new_handler: 0x7f0e8a7adb70 } }, receiver: UnsafeCell { .. } }) }), revoke_recv: ManuallyDrop { value: MaybeDangling(OneshotReceiverCore(Slot { state: Mutex { data: Waiting(None) }, receiver: UnsafeCell { .. } })) } }
[0.247408] vmbus_client: INFO  opening channel on host channel_id=0x9 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[0.249882] underhill_core::loader: INFO worker_new{ name="UnderhillWorker" action="new"}:init:init/new_underhill_vm:load_firmware:  loading UEFI into VTL0
[0.254580] hyperv_ic_guest::shutdown: INFO  version negotiated framework_version=3.0 message_version=3.2
[0.269650] underhill_core: INFO  vm worker started
[0.274174] firmware_uefi: INFO  set gpa for diagnostics addr=2b data=0x76000
[0.274394] firmware_uefi: INFO  set gpa for diagnostics addr=2b data=0x437d000
[0.274426] state_unit: INFO state_change{ operation="start"}:  state change complete duration=4.704092ms
[0.274448] underhill_core::dispatch: INFO  starting VM boot_time_ms=0x159 boot_time="345.0778ms"
[0.280693] vmbus_server::channels: INFO  client released channel offer_id=OfferId(0) key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0
[0.280710] vmbus_server::channels: INFO  client released channel offer_id=OfferId(1) key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0
[0.280734] vmbus_server::channels: INFO  client released channel offer_id=OfferId(2) key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0
[0.280745] vmbus_server::channels: INFO  client released channel offer_id=OfferId(3) key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0
[0.280753] vmbus_server::channels: INFO  client released channel offer_id=OfferId(4) key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0
[0.280759] vmbus_server::channels: INFO  client released channel offer_id=OfferId(5) key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0
[0.280765] vmbus_server::channels: INFO  client released channel offer_id=OfferId(6) key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0
[0.280772] vmbus_server::channels: INFO  client released channel offer_id=OfferId(7) key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[0.280778] vmbus_server::channels: INFO  client released channel offer_id=OfferId(8) key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0
[0.280785] vmbus_server::channels: INFO  client released channel dropped_ratelimited=0x0 offer_id=OfferId(9) key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0
[0.280791] vmbus_server::channels: INFO  guest requested unsupported feature flags. supported=0x0 requested=0x8
[0.280906] vmbus_server::channels: INFO  guest negotiated version vtl=0x0 version=VersionInfo { version: Copper, feature_flags: FeatureFlags { guest_specified_signal_parameters: false, channel_interrupt_redirection: false, modify_connection: false, client_id: false, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: false } } client_id=18dd3964-3e8a-4e42-86fa-c8e6b191ee0e trusted=false
[0.280976] vmbus_server::channels: INFO  sending offer to guest channel_id=0x1 connection_id=0x2001 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[0.280983] vmbus_server::channels: INFO  sending offer to guest channel_id=0x2 connection_id=0x2002 key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0
[0.280988] vmbus_server::channels: INFO  sending offer to guest channel_id=0x3 connection_id=0x2003 key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0
[0.280994] vmbus_server::channels: INFO  sending offer to guest channel_id=0x4 connection_id=0x2004 key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0
[0.281001] vmbus_server::channels: INFO  sending offer to guest channel_id=0x5 connection_id=0x2005 key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0
[0.281011] vmbus_server::channels: INFO  sending offer to guest channel_id=0x6 connection_id=0x2006 key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0
[0.281016] vmbus_server::channels: INFO  sending offer to guest channel_id=0x7 connection_id=0x2007 key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0
[0.281022] vmbus_server::channels: INFO  sending offer to guest channel_id=0x8 connection_id=0x2008 key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[0.281029] vmbus_server::channels: INFO  sending offer to guest channel_id=0x9 connection_id=0x2009 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0
[0.281035] vmbus_server::channels: INFO  sending offer to guest channel_id=0xa connection_id=0x200a key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0
[0.281040] vmbus_server::channels: INFO  sending offer to guest channel_id=0xb connection_id=0x200b key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0
[0.281045] vmbus_server::channels: INFO  sending offer to guest channel_id=0xc connection_id=0x200c key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0
[0.281052] vmbus_server::channels: INFO  sending offer to guest channel_id=0xd connection_id=0x200d key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0
[0.281057] vmbus_server::channels: INFO  sending offer to guest channel_id=0xe connection_id=0x200e key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0
[0.296660] vmbus_client: INFO  opening channel on host channel_id=0x5 key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0
[0.297275] vmbus_server::channels: INFO  opened channel offer_id=0x4 channel_id=0xb key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0 result=0
[0.298269] vmbus_client: INFO  opening channel on host channel_id=0x4 key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0
[0.298860] vmbus_server::channels: INFO  opened channel offer_id=0x3 channel_id=0xe key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0 result=0
[0.300191] vmbus_client: INFO  opening channel on host channel_id=0xc key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0
[0.300663] vmbus_server::channels: INFO  opened channel offer_id=0xa channel_id=0x9 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0 result=0
[0.307147] vmbus_client: INFO  opening channel on host channel_id=0xd key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0
[0.307748] vmbus_server::channels: INFO  opened channel offer_id=0xb channel_id=0xc key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0 result=0
[0.308748] vmbus_client: INFO  closing channel on host channel_id=0xd key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0
[0.308774] vmbus_server::channels: INFO  closed channel offer_id=0xb key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0
[0.309924] vmbus_client: INFO  opening channel on host channel_id=0xd key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0
[0.310271] vmbus_server::channels: INFO  opened channel offer_id=0xb channel_id=0xc key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0 result=0
[6.009866] firmware_uefi::service::event_log: INFO  uefi boot: success
[6.009968] firmware_uefi::service::nvram: INFO  Revoking guest vsm
[6.010053] vmbus_server::channels: INFO  client released channel dropped_ratelimited=0x4 offer_id=OfferId(0) key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0
[6.010069] vmbus_server::channels: INFO  client released channel offer_id=OfferId(1) key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0
[6.010084] vmbus_server::channels: INFO  client released channel offer_id=OfferId(2) key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0
[6.010097] vmbus_server::channels: INFO  client released channel offer_id=OfferId(3) key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0
[6.010119] vmbus_server::channels: INFO  client released channel offer_id=OfferId(4) key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0
[6.010134] vmbus_server::channels: INFO  client released channel offer_id=OfferId(5) key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0
[6.010147] vmbus_server::channels: INFO  client released channel offer_id=OfferId(6) key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0
[6.010155] vmbus_server::channels: INFO  client released channel offer_id=OfferId(7) key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[6.010166] vmbus_server::channels: INFO  client released channel offer_id=OfferId(8) key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0
[6.010179] vmbus_server::channels: INFO  client released channel dropped_ratelimited=0x0 offer_id=OfferId(9) key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0
[6.010256] vmbus_client: INFO  closing channel on host channel_id=0x4 key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0
[6.010272] vmbus_client: INFO  closing channel on host channel_id=0x5 key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0
[6.010287] vmbus_client: INFO  closing channel on host channel_id=0xc key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0
[6.010365] vmbus_client: INFO  closing channel on host channel_id=0xd key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0
[6.010433] vmbus_server::channels: INFO  closed channel offer_id=0x3 key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0
[6.010452] vmbus_server::channels: INFO  closed channel offer_id=0x4 key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0
[6.010470] vmbus_server::channels: INFO  closed channel offer_id=0xa key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0
[6.010492] vmbus_server::channels: INFO  closed channel offer_id=0xb key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0
[6.010995] vmbus_server::channels: INFO  Vmbus disconnected
[6.011156] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x34ba54 phase=PEI_CORE log_message="PeiDelayedDispatchOnEndOfPei Count of dispatch cycles is 0"
[6.011192] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35ac49 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (0E0B6031-5213-4934-818B-38D90CED39DB)."
[6.011202] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35ad26 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (276AACF4-AC15-426C-98DD-7521AD3F01FE)."
[6.011219] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35adfc phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (3375BAF4-9E15-4B30-B765-67ACB10D607B)."
[6.011225] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35aed3 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (35FA2E29-EA23-4236-96AE-3A6EBACBA440)."
[6.011236] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35afac phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (525074DC-8985-46E2-8057-A307DC18A502)."
[6.011242] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35b082 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (57164F39-9115-4E78-AB55-382F3BD5422D)."
[6.011253] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35b159 phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (9527E630-D0AE-497B-ADCE-E80AB0175CAF)."
[6.011267] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35b22e phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (A9A0F4E7-5A45-4D96-B827-8A841E8C03E6)."
[6.011272] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35b43f phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (CFA8B69E-5B4A-4CC0-B98B-8BA1A1F3F95A)."
[6.011284] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35b77a phase=DXE log_message="VmbusRootIsChannelAllowed: Channel not allowed during boot (F8E65716-3CB3-4A06-9A60-1889C5CCCAB5)."
[6.011298] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35c948 phase=DXE log_message="FPDT: WARNING: SEC Performance Data Hob not found, ResetEnd will be set to 0!"
[6.011305] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x35cd04 phase=DXE log_message="IoMmuLibInit: IoMmuProtocol revision 0x10001 does not support SetAttributeById. IoMmuSetAttributeById will return EFI_UNSUPPORTED."
[6.011316] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x35cd2c phase=DXE log_message="Error: Image at 000F7E15000 start failed: Unsupported"
[6.011332] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x36042f phase=DXE log_message="Error: Image at 000F7C60000 start failed: Unsupported"
[6.011340] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x361433 phase=DXE log_message="IoMmuLibInit: IoMmuProtocol revision 0x10001 does not support SetAttributeById. IoMmuSetAttributeById will return EFI_UNSUPPORTED."
[6.011353] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x364489 phase=DXE log_message="IoMmuLibInit: IoMmuProtocol revision 0x10001 does not support SetAttributeById. IoMmuSetAttributeById will return EFI_UNSUPPORTED."
[6.011358] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x3644af phase=DXE log_message="Error: Image at 000F7BA2000 start failed: Unsupported"
[6.011395] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x3a0750 phase=DXE log_message="MnpStart: MnpStartSnp failed, Already started."
[6.011404] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x3a7606 phase=DXE log_message="WARN [DE]: Failed to locate on-screen keyboard protocol (Not Found)."
[6.011421] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x3a9fea phase=DXE log_message="OnReadyToBoot: Transitioning to boot manager; errors past this point and before ExitBootServices suggest a boot manager issue."
[6.011433] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x3b978f phase=DXE log_message="ActivateCompatibilityMode - Activating Memory Protection Compatibility Mode!"
[6.011444] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0xb52b26 phase=DXE log_message="MnpSyncSendPacket: No network cable detected."
[6.011449] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0xc46bf8 phase=DXE log_message="MnpSyncSendPacket: No network cable detected."
[6.011467] firmware_uefi::service::diagnostics: ERROR  EFI log entry debug_level=ERROR ticks=0x39fd2e4 phase=RUNTIME log_message="SecurityLock::LockType: SOFTWARE_LOCK, Module: 42857F0A-13F2-4B21-8A23-53D3F714B840, Function: LockCapsuleInterface, Output: Lock Capsule Interface"
[6.011475] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x39fd4a4 phase=DXE log_message="VmbusRootExitBootServices (1374) GPADL 0x1 not cleaned up."
[6.011480] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x39fd4b5 phase=DXE log_message="VmbusRootExitBootServices (1374) GPADL 0x2 not cleaned up."
[6.011490] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x39fd4c4 phase=DXE log_message="VmbusRootExitBootServices (1374) GPADL 0x3 not cleaned up."
[6.011501] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x39fd4d1 phase=DXE log_message="VmbusRootExitBootServices (1374) GPADL 0x4 not cleaned up."
[6.011508] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x39fd4de phase=DXE log_message="VmbusRootExitBootServices (1374) GPADL 0x5 not cleaned up."
[6.011513] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x39fd4ea phase=DXE log_message="VmbusRootExitBootServices (1374) GPADL 0x6 not cleaned up."
[6.011523] firmware_uefi::service::diagnostics: WARN  EFI log entry debug_level=WARNING ticks=0x39fd52d phase=DXE log_message="VmbusRootExitBootServices (1383) orphaned 6 GPADLs (IsolationArchitecture=0)"
[6.011535] firmware_uefi::service::diagnostics::processor: WARN  suppressed logs substring="ConvertPages: range" count=0x2
[6.011542] firmware_uefi::service::diagnostics::processor: INFO  processed EFI log entries entries_processed=0x510 entries_emitted=0x20 bytes_read=0x1de20
[6.363532] virt_mshv_vtl::processor: INFO  sidecar exit
[6.363560] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.364076] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1
[6.364863] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1
[6.438928] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 1: removing from sidecar
[6.448699] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 1: remove complete
[6.448704] smpboot: Booting Node 0 Processor 1 APIC 0x1
[6.450531] virt_mshv_vtl::processor: INFO  sidecar exit
[6.450546] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.451637] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2
[6.452298] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2
[6.558923] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 2: removing from sidecar
[6.559164] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 2: remove complete
[6.559168] smpboot: Booting Node 0 Processor 2 APIC 0x2
[6.560488] virt_mshv_vtl::processor: INFO  sidecar exit
[6.560503] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.560945] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3
[6.561579] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3
[6.668928] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 3: removing from sidecar
[6.669166] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 3: remove complete
[6.669169] smpboot: Booting Node 0 Processor 3 APIC 0x3
[6.670447] virt_mshv_vtl::processor: INFO  sidecar exit
[6.670465] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.671538] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x4
[6.672193] underhill_core::vp: INFO  onlining sidecar VP cpu=0x4
[6.708931] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 4: removing from sidecar
[6.709008] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 4: remove complete
[6.709011] smpboot: Booting Node 0 Processor 4 APIC 0x4
[6.710312] virt_mshv_vtl::processor: INFO  sidecar exit
[6.710327] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.710675] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x5
[6.711327] underhill_core::vp: INFO  onlining sidecar VP cpu=0x5
[6.711727] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 5: removing from sidecar
[6.711804] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 5: remove complete
[6.711808] smpboot: Booting Node 0 Processor 5 APIC 0x5
[6.713075] virt_mshv_vtl::processor: INFO  sidecar exit
[6.713086] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.713362] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x6
[6.713876] underhill_core::vp: INFO  onlining sidecar VP cpu=0x6
[6.714208] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 6: removing from sidecar
[6.714284] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 6: remove complete
[6.714289] smpboot: Booting Node 0 Processor 6 APIC 0x6
[6.715479] virt_mshv_vtl::processor: INFO  sidecar exit
[6.715488] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.715758] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x7
[6.716423] underhill_core::vp: INFO  onlining sidecar VP cpu=0x7
[6.716754] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 7: removing from sidecar
[6.716831] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 7: remove complete
[6.716835] smpboot: Booting Node 0 Processor 7 APIC 0x7
[6.717976] virt_mshv_vtl::processor: INFO  sidecar exit
[6.717985] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.718246] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x8
[6.718926] underhill_core::vp: INFO  onlining sidecar VP cpu=0x8
[6.719338] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 8: removing from sidecar
[6.719416] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 8: remove complete
[6.719422] smpboot: Booting Node 0 Processor 8 APIC 0x8
[6.720740] virt_mshv_vtl::processor: INFO  sidecar exit
[6.720749] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.721093] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x9
[6.722172] underhill_core::vp: INFO  onlining sidecar VP cpu=0x9
[6.722538] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 9: removing from sidecar
[6.722614] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 9: remove complete
[6.722618] smpboot: Booting Node 0 Processor 9 APIC 0x9
[6.723756] virt_mshv_vtl::processor: INFO  sidecar exit
[6.723765] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.724026] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xa
[6.724690] underhill_core::vp: INFO  onlining sidecar VP cpu=0xa
[6.725028] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 10: removing from sidecar
[6.725104] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 10: remove complete
[6.725108] smpboot: Booting Node 0 Processor 10 APIC 0xa
[6.726269] virt_mshv_vtl::processor: INFO  sidecar exit
[6.726276] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.726528] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xb
[6.727193] underhill_core::vp: INFO  onlining sidecar VP cpu=0xb
[6.727518] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 11: removing from sidecar
[6.727594] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 11: remove complete
[6.727598] smpboot: Booting Node 0 Processor 11 APIC 0xb
[6.728980] virt_mshv_vtl::processor: INFO  sidecar exit
[6.728990] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.729289] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xc
[6.730230] underhill_core::vp: INFO  onlining sidecar VP cpu=0xc
[6.730608] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 12: removing from sidecar
[6.730685] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 12: remove complete
[6.730690] smpboot: Booting Node 0 Processor 12 APIC 0xc
[6.731911] virt_mshv_vtl::processor: INFO  sidecar exit
[6.731919] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.732169] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xd
[6.732832] underhill_core::vp: INFO  onlining sidecar VP cpu=0xd
[6.733225] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 13: removing from sidecar
[6.733301] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 13: remove complete
[6.733305] smpboot: Booting Node 0 Processor 13 APIC 0xd
[6.734481] virt_mshv_vtl::processor: INFO  sidecar exit
[6.734489] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.734733] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xe
[6.735398] underhill_core::vp: INFO  onlining sidecar VP cpu=0xe
[6.735739] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 14: removing from sidecar
[6.735815] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 14: remove complete
[6.735819] smpboot: Booting Node 0 Processor 14 APIC 0xe
[6.737081] virt_mshv_vtl::processor: INFO  sidecar exit
[6.737094] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.737394] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0xf
[6.738062] underhill_core::vp: INFO  onlining sidecar VP cpu=0xf
[6.738391] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 15: removing from sidecar
[6.738466] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 15: remove complete
[6.738470] smpboot: Booting Node 0 Processor 15 APIC 0xf
[6.739682] virt_mshv_vtl::processor: INFO  sidecar exit
[6.739692] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.739955] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x10
[6.740617] underhill_core::vp: INFO  onlining sidecar VP cpu=0x10
[6.740999] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 16: removing from sidecar
[6.741075] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 16: remove complete
[6.741078] smpboot: Booting Node 0 Processor 16 APIC 0x10
[6.742281] virt_mshv_vtl::processor: INFO  sidecar exit
[6.742289] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.742533] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x11
[6.743337] underhill_core::vp: INFO  onlining sidecar VP cpu=0x11
[6.743678] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 17: removing from sidecar
[6.743754] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 17: remove complete
[6.743757] smpboot: Booting Node 0 Processor 17 APIC 0x11
[6.744979] virt_mshv_vtl::processor: INFO  sidecar exit
[6.744987] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.745240] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x12
[6.745904] underhill_core::vp: INFO  onlining sidecar VP cpu=0x12
[6.746232] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 18: removing from sidecar
[6.746307] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 18: remove complete
[6.746311] smpboot: Booting Node 0 Processor 18 APIC 0x12
[6.747523] virt_mshv_vtl::processor: INFO  sidecar exit
[6.747530] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.747778] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x13
[6.748448] underhill_core::vp: INFO  onlining sidecar VP cpu=0x13
[6.748792] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 19: removing from sidecar
[6.748868] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 19: remove complete
[6.748872] smpboot: Booting Node 0 Processor 19 APIC 0x13
[6.750060] virt_mshv_vtl::processor: INFO  sidecar exit
[6.750067] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.750339] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x14
[6.751155] underhill_core::vp: INFO  onlining sidecar VP cpu=0x14
[6.751470] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 20: removing from sidecar
[6.751548] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 20: remove complete
[6.751552] smpboot: Booting Node 0 Processor 20 APIC 0x14
[6.752831] virt_mshv_vtl::processor: INFO  sidecar exit
[6.752845] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.753136] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x15
[6.753800] underhill_core::vp: INFO  onlining sidecar VP cpu=0x15
[6.754100] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 21: removing from sidecar
[6.754176] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 21: remove complete
[6.754181] smpboot: Booting Node 0 Processor 21 APIC 0x15
[6.755459] virt_mshv_vtl::processor: INFO  sidecar exit
[6.755468] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.755730] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x16
[6.756252] underhill_core::vp: INFO  onlining sidecar VP cpu=0x16
[6.756559] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 22: removing from sidecar
[6.756635] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 22: remove complete
[6.756639] smpboot: Booting Node 0 Processor 22 APIC 0x16
[6.757853] virt_mshv_vtl::processor: INFO  sidecar exit
[6.757861] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.758114] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x17
[6.758632] underhill_core::vp: INFO  onlining sidecar VP cpu=0x17
[6.758941] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 23: removing from sidecar
[6.759017] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 23: remove complete
[6.759021] smpboot: Booting Node 0 Processor 23 APIC 0x17
[6.760237] virt_mshv_vtl::processor: INFO  sidecar exit
[6.760246] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.760521] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x18
[6.761036] underhill_core::vp: INFO  onlining sidecar VP cpu=0x18
[6.761341] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 24: removing from sidecar
[6.761416] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 24: remove complete
[6.761420] smpboot: Booting Node 0 Processor 24 APIC 0x18
[6.762681] virt_mshv_vtl::processor: INFO  sidecar exit
[6.762688] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.762940] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x19
[6.763757] underhill_core::vp: INFO  onlining sidecar VP cpu=0x19
[6.764045] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 25: removing from sidecar
[6.764120] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 25: remove complete
[6.764124] smpboot: Booting Node 0 Processor 25 APIC 0x19
[6.765310] virt_mshv_vtl::processor: INFO  sidecar exit
[6.765318] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.765568] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1a
[6.766083] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1a
[6.766378] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 26: removing from sidecar
[6.766454] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 26: remove complete
[6.766458] smpboot: Booting Node 0 Processor 26 APIC 0x1a
[6.767731] virt_mshv_vtl::processor: INFO  sidecar exit
[6.767742] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.768007] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1b
[6.768531] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1b
[6.768866] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 27: removing from sidecar
[6.768942] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 27: remove complete
[6.768946] smpboot: Booting Node 0 Processor 27 APIC 0x1b
[6.770237] virt_mshv_vtl::processor: INFO  sidecar exit
[6.770246] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.770522] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1c
[6.771042] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1c
[6.771331] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 28: removing from sidecar
[6.771407] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 28: remove complete
[6.771412] smpboot: Booting Node 0 Processor 28 APIC 0x1c
[6.772683] virt_mshv_vtl::processor: INFO  sidecar exit
[6.772691] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.772937] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1d
[6.773456] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1d
[6.773740] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 29: removing from sidecar
[6.773816] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 29: remove complete
[6.773820] smpboot: Booting Node 0 Processor 29 APIC 0x1d
[6.775054] virt_mshv_vtl::processor: INFO  sidecar exit
[6.775061] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.775306] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1e
[6.775825] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1e
[6.776113] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 30: removing from sidecar
[6.776189] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 30: remove complete
[6.776193] smpboot: Booting Node 0 Processor 30 APIC 0x1e
[6.777511] virt_mshv_vtl::processor: INFO  sidecar exit
[6.777518] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.777761] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x1f
[6.778271] underhill_core::vp: INFO  onlining sidecar VP cpu=0x1f
[6.778564] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 31: removing from sidecar
[6.778639] mshv_vtl_sidecar 40a4aa000.sidecar: cpu 31: remove complete
[6.778643] smpboot: Booting Node 0 Processor 31 APIC 0x1f
[6.780310] virt_mshv_vtl::processor: INFO  sidecar exit
[6.780320] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.780608] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x21
[6.781130] underhill_core::vp: INFO  onlining sidecar VP cpu=0x21
[6.781432] mshv_vtl_sidecar 40a64b000.sidecar: cpu 33: removing from sidecar
[6.781508] mshv_vtl_sidecar 40a64b000.sidecar: cpu 33: remove complete
[6.781513] smpboot: Booting Node 0 Processor 33 APIC 0x21
[6.782869] virt_mshv_vtl::processor: INFO  sidecar exit
[6.782878] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.783165] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x22
[6.783677] underhill_core::vp: INFO  onlining sidecar VP cpu=0x22
[6.783970] mshv_vtl_sidecar 40a64b000.sidecar: cpu 34: removing from sidecar
[6.784046] mshv_vtl_sidecar 40a64b000.sidecar: cpu 34: remove complete
[6.784051] smpboot: Booting Node 0 Processor 34 APIC 0x22
[6.785362] virt_mshv_vtl::processor: INFO  sidecar exit
[6.785369] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.785675] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x23
[6.786177] underhill_core::vp: INFO  onlining sidecar VP cpu=0x23
[6.786479] mshv_vtl_sidecar 40a64b000.sidecar: cpu 35: removing from sidecar
[6.786555] mshv_vtl_sidecar 40a64b000.sidecar: cpu 35: remove complete
[6.786560] smpboot: Booting Node 0 Processor 35 APIC 0x23
[6.787818] virt_mshv_vtl::processor: INFO  sidecar exit
[6.787825] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.788080] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x24
[6.788598] underhill_core::vp: INFO  onlining sidecar VP cpu=0x24
[6.788938] mshv_vtl_sidecar 40a64b000.sidecar: cpu 36: removing from sidecar
[6.789015] mshv_vtl_sidecar 40a64b000.sidecar: cpu 36: remove complete
[6.789020] smpboot: Booting Node 0 Processor 36 APIC 0x24
[6.790354] virt_mshv_vtl::processor: INFO  sidecar exit
[6.790361] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.790621] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x25
[6.791137] underhill_core::vp: INFO  onlining sidecar VP cpu=0x25
[6.791489] mshv_vtl_sidecar 40a64b000.sidecar: cpu 37: removing from sidecar
[6.791566] mshv_vtl_sidecar 40a64b000.sidecar: cpu 37: remove complete
[6.791573] smpboot: Booting Node 0 Processor 37 APIC 0x25
[6.792844] virt_mshv_vtl::processor: INFO  sidecar exit
[6.792854] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[6.793156] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x26
[6.793681] underhill_core::vp: INFO  onlining sidecar VP cpu=0x26
[7.089011] mshv_vtl_sidecar 40a64b000.sidecar: cpu 38: removing from sidecar
[7.089089] mshv_vtl_sidecar 40a64b000.sidecar: cpu 38: remove complete
[7.089096] smpboot: Booting Node 0 Processor 38 APIC 0x26
[7.090509] virt_mshv_vtl::processor: INFO  sidecar exit
[7.090521] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.090863] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x27
[7.091521] underhill_core::vp: INFO  onlining sidecar VP cpu=0x27
[7.279006] mshv_vtl_sidecar 40a64b000.sidecar: cpu 39: removing from sidecar
[7.279085] mshv_vtl_sidecar 40a64b000.sidecar: cpu 39: remove complete
[7.279094] smpboot: Booting Node 0 Processor 39 APIC 0x27
[7.280682] virt_mshv_vtl::processor: INFO  sidecar exit
[7.280694] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.281028] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x28
[7.281825] underhill_core::vp: INFO  onlining sidecar VP cpu=0x28
[7.359001] mshv_vtl_sidecar 40a64b000.sidecar: cpu 40: removing from sidecar
[7.359079] mshv_vtl_sidecar 40a64b000.sidecar: cpu 40: remove complete
[7.359085] smpboot: Booting Node 0 Processor 40 APIC 0x28
[7.360517] virt_mshv_vtl::processor: INFO  sidecar exit
[7.360526] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.360835] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x29
[7.361629] underhill_core::vp: INFO  onlining sidecar VP cpu=0x29
[7.362079] mshv_vtl_sidecar 40a64b000.sidecar: cpu 41: removing from sidecar
[7.362156] mshv_vtl_sidecar 40a64b000.sidecar: cpu 41: remove complete
[7.362163] smpboot: Booting Node 0 Processor 41 APIC 0x29
[7.363523] virt_mshv_vtl::processor: INFO  sidecar exit
[7.363533] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.363825] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2a
[7.364618] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2a
[7.365031] mshv_vtl_sidecar 40a64b000.sidecar: cpu 42: removing from sidecar
[7.365108] mshv_vtl_sidecar 40a64b000.sidecar: cpu 42: remove complete
[7.365114] smpboot: Booting Node 0 Processor 42 APIC 0x2a
[7.366438] virt_mshv_vtl::processor: INFO  sidecar exit
[7.366446] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.366726] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2b
[7.367385] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2b
[7.367768] mshv_vtl_sidecar 40a64b000.sidecar: cpu 43: removing from sidecar
[7.367844] mshv_vtl_sidecar 40a64b000.sidecar: cpu 43: remove complete
[7.367848] smpboot: Booting Node 0 Processor 43 APIC 0x2b
[7.369274] virt_mshv_vtl::processor: INFO  sidecar exit
[7.369282] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.369587] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2c
[7.370526] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2c
[7.370974] mshv_vtl_sidecar 40a64b000.sidecar: cpu 44: removing from sidecar
[7.371052] mshv_vtl_sidecar 40a64b000.sidecar: cpu 44: remove complete
[7.371059] smpboot: Booting Node 0 Processor 44 APIC 0x2c
[7.372447] virt_mshv_vtl::processor: INFO  sidecar exit
[7.372456] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.372759] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2d
[7.373408] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2d
[7.373822] mshv_vtl_sidecar 40a64b000.sidecar: cpu 45: removing from sidecar
[7.373899] mshv_vtl_sidecar 40a64b000.sidecar: cpu 45: remove complete
[7.373906] smpboot: Booting Node 0 Processor 45 APIC 0x2d
[7.375336] virt_mshv_vtl::processor: INFO  sidecar exit
[7.375344] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.375588] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2e
[7.376398] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2e
[7.376809] mshv_vtl_sidecar 40a64b000.sidecar: cpu 46: removing from sidecar
[7.376886] mshv_vtl_sidecar 40a64b000.sidecar: cpu 46: remove complete
[7.376893] smpboot: Booting Node 0 Processor 46 APIC 0x2e
[7.378371] virt_mshv_vtl::processor: INFO  sidecar exit
[7.378381] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.378653] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x2f
[7.379319] underhill_core::vp: INFO  onlining sidecar VP cpu=0x2f
[7.379755] mshv_vtl_sidecar 40a64b000.sidecar: cpu 47: removing from sidecar
[7.379833] mshv_vtl_sidecar 40a64b000.sidecar: cpu 47: remove complete
[7.379840] smpboot: Booting Node 0 Processor 47 APIC 0x2f
[7.381226] virt_mshv_vtl::processor: INFO  sidecar exit
[7.381236] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.381493] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x30
[7.382298] underhill_core::vp: INFO  onlining sidecar VP cpu=0x30
[7.382697] mshv_vtl_sidecar 40a64b000.sidecar: cpu 48: removing from sidecar
[7.382774] mshv_vtl_sidecar 40a64b000.sidecar: cpu 48: remove complete
[7.382779] smpboot: Booting Node 0 Processor 48 APIC 0x30
[7.384206] virt_mshv_vtl::processor: INFO  sidecar exit
[7.384215] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.384455] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x31
[7.385262] underhill_core::vp: INFO  onlining sidecar VP cpu=0x31
[7.385620] mshv_vtl_sidecar 40a64b000.sidecar: cpu 49: removing from sidecar
[7.385697] mshv_vtl_sidecar 40a64b000.sidecar: cpu 49: remove complete
[7.385702] smpboot: Booting Node 0 Processor 49 APIC 0x31
[7.387145] virt_mshv_vtl::processor: INFO  sidecar exit
[7.387153] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.387385] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x32
[7.388190] underhill_core::vp: INFO  onlining sidecar VP cpu=0x32
[7.388536] mshv_vtl_sidecar 40a64b000.sidecar: cpu 50: removing from sidecar
[7.388612] mshv_vtl_sidecar 40a64b000.sidecar: cpu 50: remove complete
[7.388616] smpboot: Booting Node 0 Processor 50 APIC 0x32
[7.390030] virt_mshv_vtl::processor: INFO  sidecar exit
[7.390041] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.390332] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x33
[7.390985] underhill_core::vp: INFO  onlining sidecar VP cpu=0x33
[7.391444] mshv_vtl_sidecar 40a64b000.sidecar: cpu 51: removing from sidecar
[7.391522] mshv_vtl_sidecar 40a64b000.sidecar: cpu 51: remove complete
[7.391530] smpboot: Booting Node 0 Processor 51 APIC 0x33
[7.392939] virt_mshv_vtl::processor: INFO  sidecar exit
[7.392948] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.393193] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x34
[7.394000] underhill_core::vp: INFO  onlining sidecar VP cpu=0x34
[7.394341] mshv_vtl_sidecar 40a64b000.sidecar: cpu 52: removing from sidecar
[7.394419] mshv_vtl_sidecar 40a64b000.sidecar: cpu 52: remove complete
[7.394425] smpboot: Booting Node 0 Processor 52 APIC 0x34
[7.395934] virt_mshv_vtl::processor: INFO  sidecar exit
[7.395942] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.396187] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x35
[7.396983] underhill_core::vp: INFO  onlining sidecar VP cpu=0x35
[7.397296] mshv_vtl_sidecar 40a64b000.sidecar: cpu 53: removing from sidecar
[7.397373] mshv_vtl_sidecar 40a64b000.sidecar: cpu 53: remove complete
[7.397379] smpboot: Booting Node 0 Processor 53 APIC 0x35
[7.398763] virt_mshv_vtl::processor: INFO  sidecar exit
[7.398771] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.399071] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x36
[7.400424] underhill_core::vp: INFO  onlining sidecar VP cpu=0x36
[7.400856] mshv_vtl_sidecar 40a64b000.sidecar: cpu 54: removing from sidecar
[7.400934] mshv_vtl_sidecar 40a64b000.sidecar: cpu 54: remove complete
[7.400942] smpboot: Booting Node 0 Processor 54 APIC 0x36
[7.402441] virt_mshv_vtl::processor: INFO  sidecar exit
[7.402449] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.402732] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x37
[7.403858] underhill_core::vp: INFO  onlining sidecar VP cpu=0x37
[7.404316] mshv_vtl_sidecar 40a64b000.sidecar: cpu 55: removing from sidecar
[7.404395] mshv_vtl_sidecar 40a64b000.sidecar: cpu 55: remove complete
[7.404403] smpboot: Booting Node 0 Processor 55 APIC 0x37
[7.405935] virt_mshv_vtl::processor: INFO  sidecar exit
[7.405944] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.406191] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x38
[7.406992] underhill_core::vp: INFO  onlining sidecar VP cpu=0x38
[7.407342] mshv_vtl_sidecar 40a64b000.sidecar: cpu 56: removing from sidecar
[7.407419] mshv_vtl_sidecar 40a64b000.sidecar: cpu 56: remove complete
[7.407426] smpboot: Booting Node 0 Processor 56 APIC 0x38
[7.408964] virt_mshv_vtl::processor: INFO  sidecar exit
[7.408972] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.409209] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x39
[7.410032] underhill_core::vp: INFO  onlining sidecar VP cpu=0x39
[7.410494] mshv_vtl_sidecar 40a64b000.sidecar: cpu 57: removing from sidecar
[7.410576] mshv_vtl_sidecar 40a64b000.sidecar: cpu 57: remove complete
[7.410584] smpboot: Booting Node 0 Processor 57 APIC 0x39
[7.412082] virt_mshv_vtl::processor: INFO  sidecar exit
[7.412089] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.412350] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3a
[7.413153] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3a
[7.413532] mshv_vtl_sidecar 40a64b000.sidecar: cpu 58: removing from sidecar
[7.413609] mshv_vtl_sidecar 40a64b000.sidecar: cpu 58: remove complete
[7.413617] smpboot: Booting Node 0 Processor 58 APIC 0x3a
[7.415195] virt_mshv_vtl::processor: INFO  sidecar exit
[7.415203] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.415430] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3b
[7.416378] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3b
[7.416692] mshv_vtl_sidecar 40a64b000.sidecar: cpu 59: removing from sidecar
[7.416768] mshv_vtl_sidecar 40a64b000.sidecar: cpu 59: remove complete
[7.416774] smpboot: Booting Node 0 Processor 59 APIC 0x3b
[7.418254] virt_mshv_vtl::processor: INFO  sidecar exit
[7.418261] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.418493] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3c
[7.419311] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3c
[7.419761] mshv_vtl_sidecar 40a64b000.sidecar: cpu 60: removing from sidecar
[7.419838] mshv_vtl_sidecar 40a64b000.sidecar: cpu 60: remove complete
[7.419850] smpboot: Booting Node 0 Processor 60 APIC 0x3c
[7.421349] virt_mshv_vtl::processor: INFO  sidecar exit
[7.421357] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.421606] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3d
[7.422405] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3d
[7.422721] mshv_vtl_sidecar 40a64b000.sidecar: cpu 61: removing from sidecar
[7.422797] mshv_vtl_sidecar 40a64b000.sidecar: cpu 61: remove complete
[7.422803] smpboot: Booting Node 0 Processor 61 APIC 0x3d
[7.424358] virt_mshv_vtl::processor: INFO  sidecar exit
[7.424366] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.424602] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3e
[7.425553] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3e
[7.425859] mshv_vtl_sidecar 40a64b000.sidecar: cpu 62: removing from sidecar
[7.425935] mshv_vtl_sidecar 40a64b000.sidecar: cpu 62: remove complete
[7.425940] smpboot: Booting Node 0 Processor 62 APIC 0x3e
[7.427694] virt_mshv_vtl::processor: INFO  sidecar exit
[7.427705] virt_mshv_vtl::processor: INFO  sidecar exit reason=Exit(Msr { msr: 140, value: None })
[7.427942] underhill_core::vp: INFO  thread spawned for sidecar VP, waiting to online cpu=0x3f
[7.429210] underhill_core::vp: INFO  onlining sidecar VP cpu=0x3f
[7.429598] mshv_vtl_sidecar 40a64b000.sidecar: cpu 63: removing from sidecar
[7.429676] mshv_vtl_sidecar 40a64b000.sidecar: cpu 63: remove complete
[7.429681] smpboot: Booting Node 0 Processor 63 APIC 0x3f
[7.489375] vmbus_server::channels: INFO  guest negotiated version vtl=0x0 version=VersionInfo { version: Iron, feature_flags: FeatureFlags { guest_specified_signal_parameters: false, channel_interrupt_redirection: false, modify_connection: false, client_id: false, confidential_channels: false, pause_resume: false, server_specified_monitor_pages: false, gpa_pinning: false } } client_id=00000000-0000-0000-0000-000000000000 trusted=false
[7.489826] vmbus_server::channels: INFO  sending offer to guest channel_id=0x1 connection_id=0x2001 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0
[7.489836] vmbus_server::channels: INFO  sending offer to guest channel_id=0x2 connection_id=0x2002 key={276aacf4-ac15-426c-98dd-7521ad3f01fe}-{f5bee29c-1741-4aad-a4c2-8fdedb46dcc2}-0
[7.489843] vmbus_server::channels: INFO  sending offer to guest channel_id=0x3 connection_id=0x2003 key={3375baf4-9e15-4b30-b765-67acb10d607b}-{4487b255-b88c-403f-bb51-d1f69cf17f87}-0
[7.489855] vmbus_server::channels: INFO  sending offer to guest channel_id=0x4 connection_id=0x2004 key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0
[7.489867] vmbus_server::channels: INFO  sending offer to guest channel_id=0x5 connection_id=0x2005 key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0
[7.489874] vmbus_server::channels: INFO  sending offer to guest channel_id=0x6 connection_id=0x2006 key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0
[7.489881] vmbus_server::channels: INFO  sending offer to guest channel_id=0x7 connection_id=0x2007 key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0
[7.489888] vmbus_server::channels: INFO  sending offer to guest channel_id=0x8 connection_id=0x2008 key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[7.489894] vmbus_server::channels: INFO  sending offer to guest channel_id=0x9 connection_id=0x2009 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0
[7.489903] vmbus_server::channels: INFO  sending offer to guest channel_id=0xa connection_id=0x200a key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0
[7.489909] vmbus_server::channels: INFO  sending offer to guest channel_id=0xb connection_id=0x200b key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0
[7.489915] vmbus_server::channels: INFO  sending offer to guest channel_id=0xc connection_id=0x200c key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0
[7.489921] vmbus_server::channels: INFO  sending offer to guest channel_id=0xd connection_id=0x200d key={f8e65716-3cb3-4a06-9a60-1889c5cccab5}-{99221fa0-24ad-11e2-be98-001aa01bbf6e}-0
[7.489926] vmbus_server::channels: INFO  sending offer to guest channel_id=0xe connection_id=0x200e key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0
[7.555794] vmbus_client: INFO  opening channel on host channel_id=0xc key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0
[7.556362] vmbus_server::channels: INFO  opened channel offer_id=0xa channel_id=0x9 key={ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}-{31e26bcb-1173-4589-9021-6e886ddc520d}-0 result=0
[7.567044] vmbus_server::channels: INFO  opened channel offer_id=0xd channel_id=0x1 key={0e0b6031-5213-4934-818b-38d90ced39db}-{b6650ff7-33bc-4840-8048-e0676786f393}-0 result=0
[7.567103] vmbus_client: INFO  opening channel on host channel_id=0x7 key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0
[7.567121] vmbus_client: INFO  opening channel on host channel_id=0xa key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0
[7.567140] vmbus_client: INFO  opening channel on host channel_id=0x8 key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[7.567151] vmbus_client: INFO  opening channel on host channel_id=0xe key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0
[7.567780] vmbus_server::channels: INFO  opened channel offer_id=0x8 channel_id=0x4 key={35fa2e29-ea23-4236-96ae-3a6ebacba440}-{2450ee40-33bf-4fbd-892e-9fb06e9214cf}-0 result=0
[7.567799] vmbus_server::channels: INFO  opened channel offer_id=0x6 channel_id=0x6 key={57164f39-9115-4e78-ab55-382f3bd5422d}-{fd149e91-82e0-4a7d-afa6-2a4166cbd7c0}-0 result=0
[7.568021] vmbus_server::channels: INFO  opened channel offer_id=0x7 channel_id=0x8 key={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0 result=0
[7.568206] vmbus_server::channels: INFO  opened channel offer_id=0xc channel_id=0x7 key={9527e630-d0ae-497b-adce-e80ab0175caf}-{2dd1ce17-079e-403c-b352-a1921ee207ee}-0 result=0
[7.569115] vmbus_client: INFO  opening channel on host channel_id=0x1 key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0
[7.569716] vmbus_server::channels: INFO  opened channel offer_id=0x0 channel_id=0x5 key={525074dc-8985-46e2-8057-a307dc18a502}-{1eccfd72-4b41-45ef-b73a-4a6e44c12924}-0 result=0
[7.668080] vmbus_server: INFO  waking guest for outgoing ring channel={a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}-{242ff919-07db-4180-9c2e-b86cb68c8c55}-0
[7.810341] vmbus_client: INFO  opening channel on host channel_id=0x4 key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0
[7.810986] vmbus_client: INFO  opening channel on host channel_id=0xd key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0
[7.811255] vmbus_server::channels: INFO  opened channel offer_id=0x3 channel_id=0xe key={f912ad6d-2b17-48ea-bd65-f927a61c7684}-{d34b2567-b9b6-42b9-8778-0a4ec0b955bf}-0 result=0
[7.811734] vmbus_server::channels: INFO  opened channel offer_id=0xb channel_id=0xc key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-0 result=0
[7.825630] vmbus_client: INFO  opening channel on host channel_id=0x3 key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0
[7.826415] vmbus_server::channels: INFO  opened channel dropped_ratelimited=0x0 offer_id=0x2 channel_id=0xa key={cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}-{58f75a6d-d949-4320-99e1-a2a2576d581c}-0 result=0
[7.829391] vmbus_client: INFO  received offer state=Connected channel_id=0xf interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=50f05d00-56d3-42c7-bf23-407a92a441cf subchannel_index=0x1
[7.829422] vmbus_client: INFO  received offer state=Connected channel_id=0x10 interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=50f05d00-56d3-42c7-bf23-407a92a441cf subchannel_index=0x2
[7.829439] vmbus_client: INFO  received offer state=Connected channel_id=0x11 interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=50f05d00-56d3-42c7-bf23-407a92a441cf subchannel_index=0x3
[7.829456] vmbus_client: INFO  received offer state=Connected channel_id=0x12 interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=50f05d00-56d3-42c7-bf23-407a92a441cf subchannel_index=0x4
[7.829469] vmbus_client: INFO  received offer state=Connected channel_id=0x13 interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=50f05d00-56d3-42c7-bf23-407a92a441cf subchannel_index=0x5
[7.829493] vmbus_client: INFO  received offer state=Connected channel_id=0x14 interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=50f05d00-56d3-42c7-bf23-407a92a441cf subchannel_index=0x6
[7.829542] vmbus_server::channels: INFO  sending offer to guest channel_id=0xf connection_id=0x200f key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-1
[7.829563] vmbus_server::channels: INFO  new channel offer_id=OfferId(e) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-1 confidential_ring_buffer=false confidential_external_memory=false
[7.829597] vmbus_server::channels: INFO  sending offer to guest channel_id=0x10 connection_id=0x2010 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-2
[7.829609] vmbus_server::channels: INFO  new channel offer_id=OfferId(f) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-2 confidential_ring_buffer=false confidential_external_memory=false
[7.829637] vmbus_server::channels: INFO  sending offer to guest channel_id=0x11 connection_id=0x2011 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-3
[7.829649] vmbus_server::channels: INFO  new channel offer_id=OfferId(10) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-3 confidential_ring_buffer=false confidential_external_memory=false
[7.829669] vmbus_server::channels: INFO  sending offer to guest channel_id=0x12 connection_id=0x2012 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-4
[7.829680] vmbus_server::channels: INFO  new channel offer_id=OfferId(11) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-4 confidential_ring_buffer=false confidential_external_memory=false
[7.829700] vmbus_server::channels: INFO  sending offer to guest channel_id=0x13 connection_id=0x2013 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-5
[7.829710] vmbus_server::channels: INFO  new channel offer_id=OfferId(12) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-5 confidential_ring_buffer=false confidential_external_memory=false
[7.829729] vmbus_server::channels: INFO  sending offer to guest channel_id=0x14 connection_id=0x2014 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-6
[7.829740] vmbus_server::channels: INFO  new channel offer_id=OfferId(13) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-6 confidential_ring_buffer=false confidential_external_memory=false
[7.829780] vmbus_client: INFO  received offer state=Connected channel_id=0x15 interface_id=f8615163-df3e-46c5-913f-f2d2f965ed0e instance_id=50f05d00-56d3-42c7-bf23-407a92a441cf subchannel_index=0x7
[7.829801] vmbus_server::channels: INFO  sending offer to guest channel_id=0x15 connection_id=0x2015 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-7
[7.829811] vmbus_server::channels: INFO  new channel offer_id=OfferId(14) key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-7 confidential_ring_buffer=false confidential_external_memory=false
[7.830955] vmbus_client: INFO  opening channel on host channel_id=0xf key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-1
[7.832650] vmbus_client: INFO  opening channel on host channel_id=0x10 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-2
[7.834618] vmbus_client: INFO  opening channel on host channel_id=0x11 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-3
[7.836009] vmbus_client: INFO  opening channel on host channel_id=0x12 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-4
[7.837116] vmbus_client: INFO  opening channel on host channel_id=0x13 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-5
[7.838102] vmbus_client: INFO  opening channel on host channel_id=0x14 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-6
[7.839216] vmbus_client: INFO  opening channel on host channel_id=0x15 key={f8615163-df3e-46c5-913f-f2d2f965ed0e}-{50f05d00-56d3-42c7-bf23-407a92a441cf}-7
[10.966131] vmbus_client: INFO  opening channel on host channel_id=0x5 key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0
 key={da0a7802-e377-4aac-8e77-0558eb1073f8}-{5620e0c7-8062-4dce-aeb7-520c7ef76171}-0
