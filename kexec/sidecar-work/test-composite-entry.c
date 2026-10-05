// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#define _GNU_SOURCE
#include <assert.h>
#include <elf.h>
#include <inttypes.h>
#include <setjmp.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <ucontext.h>

struct physical_range {
    uint64_t base;
    uint64_t size;
};

struct composite_header {
    uint64_t magic;
    uint32_t version;
    uint32_t header_size;
    struct physical_range kernel;
    uint64_t kernel_entry;
    struct physical_range sidecar;
    uint64_t sidecar_entry;
    struct physical_range bootstrap;
    uint64_t image_size;
    uint64_t reserved[2];
};

static sigjmp_buf return_point;
static volatile sig_atomic_t caught_signal;
static uintptr_t trapped_rip;
static uintptr_t trapped_rsi;
static uintptr_t trapped_rdi;
static uint64_t crash_registers[6];
static volatile sig_atomic_t intercept_crash;
static volatile sig_atomic_t crash_notified;

static void trap(int signal_number, siginfo_t *info, void *context)
{
    ucontext_t *state = context;
    (void)info;
    if (intercept_crash && signal_number == SIGSEGV) {
        const unsigned char *instruction = (void *)state->uc_mcontext.gregs[REG_RIP];
        uint32_t msr = state->uc_mcontext.gregs[REG_RCX];
        if (instruction[0] == 0x0f && instruction[1] == 0x30
            && msr >= 0x40000100 && msr <= 0x40000105) {
            crash_registers[msr - 0x40000100] =
                (uint32_t)state->uc_mcontext.gregs[REG_RAX]
                | ((uint64_t)(uint32_t)state->uc_mcontext.gregs[REG_RDX] << 32);
            state->uc_mcontext.gregs[REG_RIP] += 2;
            if (msr != 0x40000105)
                return;
            crash_notified = 1;
            siglongjmp(return_point, 1);
        }
    }
    caught_signal = signal_number;
    trapped_rip = state->uc_mcontext.gregs[REG_RIP];
    trapped_rsi = state->uc_mcontext.gregs[REG_RSI];
    trapped_rdi = state->uc_mcontext.gregs[REG_RDI];
    siglongjmp(return_point, 1);
}

int main(int argc, char **argv)
{
    const uintptr_t base = 0x400000000ULL;
    Elf64_Ehdr header;
    Elf64_Phdr programs[4];
    uint64_t memory_size = 0;
    FILE *file;
    unsigned char *memory, *boot_params, *entry;
    struct composite_header layout;
    struct sigaction action = { .sa_sigaction = trap, .sa_flags = SA_SIGINFO | SA_ONSTACK };
    stack_t signal_stack = { .ss_size = 65536 };
    size_t read_patches = 0, write_patches = 0;

    if (argc < 2 || argc > 3) {
        fprintf(stderr, "usage: %s composite.elf [layout|loader]\n", argv[0]);
        return 2;
    }
    file = fopen(argv[1], "rb");
    assert(file);
    assert(fread(&header, sizeof(header), 1, file) == 1);
    assert(!memcmp(header.e_ident, ELFMAG, SELFMAG));
    assert(header.e_machine == EM_X86_64 && header.e_type == ET_EXEC);
    assert(header.e_phnum == 4 && header.e_phentsize == sizeof(programs[0]));
    assert(fseek(file, header.e_phoff, SEEK_SET) == 0);
    assert(fread(programs, sizeof(programs), 1, file) == 1);
    for (size_t index = 0; index < 4; index++) {
        assert(programs[index].p_type == PT_LOAD);
        assert(programs[index].p_paddr < 128 * 1024 * 1024);
        assert(programs[index].p_filesz <= programs[index].p_memsz);
        assert(programs[index].p_memsz <= 128 * 1024 * 1024 - programs[index].p_paddr);
        uint64_t end = programs[index].p_paddr + programs[index].p_memsz;
        if (end > memory_size)
            memory_size = end;
    }
    memory = mmap((void *)base, memory_size, PROT_READ | PROT_WRITE | PROT_EXEC,
                  MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE, -1, 0);
    assert(memory != MAP_FAILED && (uintptr_t)memory == base);
    for (size_t index = 0; index < 4; index++) {
        assert(fseek(file, programs[index].p_offset, SEEK_SET) == 0);
        assert(fread(memory + programs[index].p_paddr, 1, programs[index].p_filesz, file)
               == programs[index].p_filesz);
    }
    assert(fclose(file) == 0);
    memcpy(&layout, memory, sizeof(layout));
    assert(layout.header_size == sizeof(layout) && layout.image_size == memory_size);
    assert(layout.kernel_entry < memory_size && header.e_entry < memory_size);
    assert(layout.bootstrap.base == 0x200000);
    Elf64_Ehdr *bootstrap = (void *)(memory + layout.bootstrap.base);
    assert(bootstrap->e_entry + 256 < layout.bootstrap.size);
    entry = memory + layout.bootstrap.base + bootstrap->e_entry;
    for (size_t index = 0; index < 253; index++) {
        if (!memcmp(entry + index, "\x0f\x20\xe0", 3)) {
            memcpy(entry + index, "\x31\xc0\x90", 3);
            read_patches++;
        } else if (!memcmp(entry + index, "\x0f\x22\xe0", 3)) {
            memset(entry + index, 0x90, 3);
            write_patches++;
        }
    }
    assert(read_patches == 1 && write_patches == 1);
    memory[layout.kernel_entry] = 0xcc;
    boot_params = mmap(NULL, 4096, PROT_READ | PROT_WRITE,
                       MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    assert(boot_params != MAP_FAILED);
    boot_params[0x210] = 0xd0;
    if (argc == 3) {
        intercept_crash = 1;
        if (!strcmp(argv[2], "layout")) {
            ((struct composite_header *)memory)->version++;
        } else if (!strcmp(argv[2], "loader")) {
            boot_params[0x210] = 0;
        } else {
            fprintf(stderr, "unknown injected failure: %s\n", argv[2]);
            return 2;
        }
    }
    signal_stack.ss_sp = malloc(signal_stack.ss_size);
    assert(signal_stack.ss_sp && sigaltstack(&signal_stack, NULL) == 0);
    sigemptyset(&action.sa_mask);
    assert(sigaction(SIGTRAP, &action, NULL) == 0);
    assert(sigaction(SIGSEGV, &action, NULL) == 0);
    assert(sigaction(SIGILL, &action, NULL) == 0);
    assert(sigaction(SIGBUS, &action, NULL) == 0);
    if (!sigsetjmp(return_point, 1)) {
        __asm__ volatile("mov %0, %%rsi; jmp *%1"
                         : : "r"(boot_params), "r"(memory + header.e_entry) : "rsi", "memory");
    }
    if (intercept_crash) {
        const char *tag = !strcmp(argv[2], "layout") ? "KXLAYOUT" : "KXLOADER";
        uint64_t expected = 0;
        for (size_t index = 0; index < 8; index++)
            expected = (expected << 8) | (unsigned char)tag[index];
        if (!crash_notified || crash_registers[1] != expected)
            return 1;
        assert(crash_registers[3] >= base && crash_registers[3] < base + memory_size);
        assert(crash_registers[4] > 0 && crash_registers[4] < 512);
        printf("Reported %s: %.*s\n", tag, (int)crash_registers[4], (char *)(uintptr_t)crash_registers[3]);
        return 0;
    }
    printf("signal=%d rip=%#" PRIxPTR " bootstrap_offset=%#" PRIxPTR
           " rsi=%#" PRIxPTR " expected_rsi=%#" PRIxPTR "\n",
           caught_signal, trapped_rip, trapped_rip - base - layout.bootstrap.base,
           trapped_rsi, (uintptr_t)boot_params);
    if (caught_signal != SIGTRAP || trapped_rip != base + layout.kernel_entry + 1
        || trapped_rsi != (uintptr_t)boot_params || trapped_rdi != 0)
        return 1;
    puts("Composite bootstrap reached Linux entry with the original boot parameters");
    return 0;
}
