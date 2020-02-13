    .section .text.entry
    .globl _start
_start:
    # we start at 0x80000000 M-Mode

    # 1. setup CSRs
    csrw    mie, 0
    csrw    mip, 0
    csrw    mscratch, 0
    li      t0, -1
    csrw    medeleg, t0         # delegate all exceptions to S-Mode
    csrw    mideleg, t0         # delegate all interrupts to S-Mode
    csrw    mcounteren, t0      # allow S-Mode reading time register
    li      t0, 1 << 11
    csrw    mstatus, t0         # MPP = S-Mode
    lui     t0, %hi(rust_main)
    addi    t0, t0, %lo(rust_main)
    csrw    mepc, t0            # mepc = rust_main

    # 2. setup page table
    lui     t0, %hi(boot_page_table_sv39)
    li      t1, 0xffffffffc0000000 - 0x80000000
    sub     t0, t0, t1
    srli    t0, t0, 12
    li      t1, 8 << 60
    or      t0, t0, t1
    csrw    satp, t0
    sfence.vma

    # 3. go to S-Mode
    lui     sp, %hi(bootstacktop)
    mret

    .section .bss.stack
    .align 12
    .global bootstack
bootstack:
    .space 4096 * 4
    .global bootstacktop
bootstacktop:

    .section .data
    .align 12   # page align
boot_page_table_sv39:
    # 0xffffffff_40000000 -> 0x00000000 (1G)
    # 0xffffffff_80000000 -> 0x40000000 (1G)
    # 0xffffffff_c0000000 -> 0x80000000 (1G)
    .zero 8 * 509
    .quad (0x00000 << 10) | 0xcf # VRWXAD
    .quad (0x40000 << 10) | 0xcf # VRWXAD
    .quad (0x80000 << 10) | 0xcf # VRWXAD
