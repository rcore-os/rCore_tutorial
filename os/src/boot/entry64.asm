    .section .text.entry
    .globl _start
_start:
    # we start at 0x80000000 M-Mode

    # 1. setup CSRs
    li      t0, 0x80
    csrw    mie, t0             # MTIE=1
    csrw    mip, 0

    la      t0, scratch
    csrw    mscratch, t0        # mscratch = [scratch]

    li      t0, -1
    csrw    medeleg, t0         # delegate all exceptions to S-Mode
    csrw    mideleg, t0         # delegate all interrupts to S-Mode
    csrw    mcounteren, t0      # allow S-Mode reading time register

    li      t0, (1 << 11) | (1 << 7)
    csrw    mstatus, t0         # MPP=S-Mode, MPIE=1

    la      t0, mtrapvec
    csrw    mtvec, t0           # mtvec = [mtrapvec]

    lui     t0, %hi(rust_main)
    addi    t0, t0, %lo(rust_main)
    csrw    mepc, t0            # mepc = [rust_main]

    # 2. setup page table
    la      t0, boot_page_table_sv39
    srli    t0, t0, 12
    li      t1, 8 << 60
    or      t0, t0, t1
    csrw    satp, t0
    sfence.vma

    # 3. go to S-Mode
    lui     sp, %hi(bootstacktop)
    mret


    # machine timer interrupt handler
    # ref: https://github.com/mit-pdos/xv6-riscv/blob/9ead904afef8d060c2cc5cee6bd8e8d223de8c40/kernel/kernelvec.S#L93
    .align 4
mtrapvec:
    # save registers
    csrrw   a0, mscratch, a0
    sd      a1, 0(a0)
    sd      a2, 8(a0)
    sd      a3, 16(a0)

    # schedule the next timer interrupt
    # by adding interval to mtimecmp.
    li      a1, 0x200BFF8   # CLINT_MTIME
    ld      a1, (a1)        # mtime
    li      a2, 1000000     # interval: about 1/10th second in qemu.
    add     a2, a2, a1
    li      a1, 0x2004000   # CLINT_MTIMECMP(hart=0)
    sd      a2, (a1)        # mtimecmp = mtime + interval

    # raise a supervisor software interrupt.
    li      a1, 1 << 1
    csrs    mip, a1         # set SSIP

    # debug: print a char
    # li      a1, 0x10000000
    # li      a2, 'x'
    # sb      a2, (a1)

    # recover registers
    ld      a3, 16(a0)
    ld      a2, 8(a0)
    ld      a1, 0(a0)
    csrrw   a0, mscratch, a0

    mret

    .section .data
scratch:
    .zero 8 * 3


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
