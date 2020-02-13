use crate::memory::access_pa_via_va;
use core::sync::atomic::spin_loop_hint;

pub fn console_putchar(ch: usize) {
    unsafe {
        while STATUS.read_volatile() & CAN_WRITE == 0 {
            spin_loop_hint();
        }
        DATA.write_volatile(ch as u8);
    }
}

pub fn console_getchar() -> u8 {
    unsafe {
        plic_claim();
        while STATUS.read_volatile() & CAN_READ == 0 {
            spin_loop_hint();
        }
        DATA.read_volatile()
    }
}

pub fn set_timer(stime_value: u64) {
    unsafe {
        clint_mtimecmp(0).write_volatile(stime_value);
    }
}

pub fn get_cycle() -> u64 {
    unsafe { CLINT_MTIME.read_volatile() }
}

pub fn init() {
    unsafe {
        // closed by OpenSBI, so we open them manually
        // see https://github.com/rcore-os/rCore/blob/54fddfbe1d402ac1fafd9d58a0bd4f6a8dd99ece/kernel/src/arch/riscv32/board/virt/mod.rs#L4
        init_external_interrupt();
        enable_serial_interrupt();
    }
}

unsafe fn init_external_interrupt() {
    const HART0_S_MODE_INTERRUPT_ENABLES: *mut u32 = access_pa_via_va(0x0c00_2080) as *mut u32;
    const SERIAL: u32 = 0xa;
    // set uart's enable bit for this hart's S-mode.
    HART0_S_MODE_INTERRUPT_ENABLES.write_volatile(1 << SERIAL);
    // set desired IRQ priorities non-zero (otherwise disabled).
    (PLIC as *mut u32).add(SERIAL as usize).write_volatile(1);
    // set this hart's S-mode priority threshold to 0.
    plic_s_priority(0).write_volatile(0);
}

unsafe fn enable_serial_interrupt() {
    UART16550.add(4).write_volatile(0x0B);
    UART16550.add(1).write_volatile(0x01);
}

fn plic_claim() {
    unsafe {
        let claim = plic_s_claim(0);
        let irq = claim.read_volatile();
        claim.write_volatile(irq);
    }
}

const UART16550: *mut u8 = access_pa_via_va(0x10000000) as *mut u8;
const DATA: *mut u8 = access_pa_via_va(0x10000000) as *mut u8;
const STATUS: *const u8 = access_pa_via_va(0x10000005) as *const u8;
const CAN_READ: u8 = 1 << 0;
const CAN_WRITE: u8 = 1 << 5;

// local interrupt controller, which contains the timer.
const CLINT: usize = access_pa_via_va(0x2000000);
const CLINT_MTIME: *const u64 = (CLINT + 0xBFF8) as *const u64; // cycles since boot.

const fn clint_mtimecmp(hartid: usize) -> *mut u64 {
    (CLINT + 0x4000 + 8 * hartid) as _
}

const PLIC: usize = access_pa_via_va(0x0C00_0000);

const fn plic_s_priority(hartid: usize) -> *mut u32 {
    (PLIC + 0x201000 + hartid * 0x2000) as _
}

const fn plic_s_claim(hartid: usize) -> *mut u32 {
    (PLIC + 0x201004 + hartid * 0x2000) as _
}
