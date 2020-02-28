.equ XLENB, 8
.macro Load a1, a2 
    ld \a1, \a2*XLENB(sp)
.endm
.macro Store a1, a2 
    sd \a1, \a2*XLENB(sp)
.endm
    addi sp, sp, -14*XLENB
    sd sp, 0(a0)
    Store ra, 0
    Store s0, 2
    Store s1, 3
    Store s2, 4
	Store s3, 5
	Store s4, 6
	Store s5, 7
	Store s6, 8
	Store s7, 9
	Store s8, 10
	Store s9, 11
    Store s10, 12
    Store s11, 13
    csrr s11, satp
    Store s11, 1

    ld sp, 0(a1)
    Load s11, 1
    csrw satp, s11
    sfence.vma
    Load ra, 0
    Load s0, 2
    Load s1, 3
    Load s2, 4
	Load s3, 5
	Load s4, 6
	Load s5, 7
	Load s6, 8
	Load s7, 9
	Load s8, 10
	Load s9, 11
    Load s10, 12
    Load s11, 13
    addi sp, sp, 14*XLENB

    sd zero, 0(a1)
    ret
