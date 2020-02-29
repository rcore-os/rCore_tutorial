use crate::context::TrapFrame;
use crate::memory::access_pa_via_va;
use crate::process::tick;
use crate::timer::{clock_set_next_event, now};
use core::time::Duration;
use riscv::register::sie;
use riscv::register::{
    scause::{self, Exception, Interrupt, Trap},
    sepc, sscratch, sstatus, stvec,
};

global_asm!(include_str!("trap/trap.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            fn __alltraps();
        }
        sscratch::write(0);

        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);

        sstatus::set_sie();

        // enable external interrupt
        sie::set_sext();

        // closed by OpenSBI, so we open them manually
        // see https://github.com/rcore-os/rCore/blob/54fddfbe1d402ac1fafd9d58a0bd4f6a8dd99ece/kernel/src/arch/riscv32/board/virt/mod.rs#L4
        init_external_interrupt();
        enable_serial_interrupt();

    }
    println!("++++ setup interrupt! ++++");
}

pub unsafe fn init_external_interrupt() {
    let HART0_S_MODE_INTERRUPT_ENABLES: *mut u32 = access_pa_via_va(0x0c00_2080) as *mut u32;
    const SERIAL: u32 = 0xa;
    const VIRTIO0: u32 = 0x1;
    HART0_S_MODE_INTERRUPT_ENABLES.write_volatile(1 << SERIAL | 1 << VIRTIO0);
}

pub unsafe fn enable_serial_interrupt() {
    let UART16550: *mut u8 = access_pa_via_va(0x10000000) as *mut u8;
    UART16550.add(4).write_volatile(0x0B);
    UART16550.add(1).write_volatile(0x01);
}

pub fn plic_claim() -> i32 {
    let irq = access_pa_via_va(0x0c20_1004) as *const i32;
    unsafe { *irq }
}

pub fn plic_complete(irq: i32) {
    unsafe {
        *(access_pa_via_va(0x0c20_1004) as *mut i32) = irq;
    }
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    // println!("scause = {:#x}", tf.scause.bits());
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(&mut tf.sepc),
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        Trap::Exception(Exception::InstructionPageFault) => page_fault(tf),
        Trap::Exception(Exception::LoadPageFault) => page_fault(tf),
        Trap::Exception(Exception::StorePageFault) => page_fault(tf),
        Trap::Exception(Exception::UserEnvCall) => syscall(tf),
        Trap::Interrupt(Interrupt::SupervisorExternal) => external(tf),
        _ => panic!("undefined trap!"),
    }
}

fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}

fn super_timer() {
    // println!("T");
    clock_set_next_event();
    tick(now());
}

fn page_fault(tf: &mut TrapFrame) {
    println!(
        "{:?} va = {:#x} instruction = {:#x}",
        tf.scause.cause(),
        tf.stval,
        tf.sepc
    );
    panic!("page fault!");
}

fn syscall(tf: &mut TrapFrame) {
    tf.sepc += 4;
    let ret = crate::syscall::syscall(tf.x[17], [tf.x[10], tf.x[11], tf.x[12]], tf);
    tf.x[10] = ret as usize;
}

fn external(tf: &mut TrapFrame) {
    // println!("into external"); 
    if tf.scause.is_interrupt() && (tf.scause.bits() & 0xff == 0x9) {
        // println!("supervisorExternal!");
        let irq = plic_claim();
        // println!("irq = {}", irq);
        if irq == 0x01 {
            crate::drivers::virtio_disk::virtio_disk_intr();
        } else if irq == 0x0a {
            try_serial();
        } else {
            // println!("irq = {}", irq);
        }
        if irq > 0 {
            plic_complete(irq);
        }
    } else {
        panic!("unhandled external!");   
    }
}

fn try_serial() -> bool {
    match super::io::getchar_option() {
        Some(ch) => {
            if (ch == '\r') {
                crate::fs::stdio::STDIN.push('\n');
            } else {
                crate::fs::stdio::STDIN.push(ch);
            }
            true
        }
        None => false,
    }
}
#[inline(always)]
pub fn r_medeleg() -> usize {
    let mut ret: usize = 0;
    unsafe {
        asm!("csrr $0, medeleg" : "=r"(ret) ::: "volatile");
    }
    ret
}
#[inline(always)]
pub fn r_mideleg() -> usize {
    let mut ret: usize = 0;
    unsafe {
        asm!("csrr $0, mideleg" : "=r"(ret) ::: "volatile");
    }
    ret
}
#[inline(always)]
pub fn disable_timer_and_store() -> usize {
    let sie: usize;
    let bitmask: usize = 1 << 5;
    unsafe {
        asm!("csrrc $0, sie, $1" : "=r"(sie) : "r"(bitmask):: "volatile");
    }
    sie
}
#[inline(always)]
pub fn restore_timer(sie: usize) {
    unsafe {
        asm!("csrs sie, $0" :: "r"(sie) :: "volatile");
    }
}
#[inline(always)]
pub fn enable_and_store() -> usize {
    let sstatus: usize;
    unsafe {
        asm!("csrsi sstatus, 1 << 1" : "=r"(sstatus) ::: "volatile");
    }
    sstatus
}
#[inline(always)]
pub fn disable_and_store() -> usize {
    let sstatus: usize;
    unsafe {
        asm!("csrci sstatus, 1 << 1" : "=r"(sstatus) ::: "volatile");
    }
    sstatus
}

#[inline(always)]
pub fn restore(flags: usize) {
    unsafe {
        asm!("csrs sstatus, $0" :: "r"(flags) :: "volatile");
    }
}

#[inline(always)]
pub fn enable_and_wfi() {
    unsafe {
        asm!("csrsi sstatus, 1 << 1; wfi" :::: "volatile");
    }
}

#[inline(always)]
pub fn enable() {
    unsafe {
        asm!("csrsi sstatus, 1 << 1" :::: "volatile");
    }
}
