    .section .text.entry
    .globl _start
_start:
    lui     t0, %hi(boot_page_table_sv39)
    li      t1, 0xffffffffc0000000 - 0x80000000
    sub     t0, t0, t1
    srli    t0, t0, 12
    li      t1, 8 << 60
    or      t0, t0, t1
    csrw    satp, t0
    sfence.vma

    lui sp, %hi(bootstacktop)

    lui t0, %hi(rust_main)
    addi t0, t0, %lo(rust_main)
    jr t0

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
    # 0xffffffff_c0000000 -> 0x80000000 (1G)
    .zero 8 * 511
    .quad (0x80000 << 10) | 0xcf # VRWXAD
