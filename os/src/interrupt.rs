use crate::context::TrapFrame;
use crate::memory::access_pa_via_va;
use crate::process::tick;
use crate::timer::clock_set_next_event;
use riscv::register::sie;
use riscv::register::{
    scause::{Exception, Interrupt, Trap},
    sscratch, sstatus, stvec,
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
    let hart0_s_mode_interrupt_mth: *mut u32 = access_pa_via_va(0x0c20_1000) as *mut u32;
    hart0_s_mode_interrupt_mth.write_volatile(0);

    let hart0_s_mode_interrupt_priority_serial_irq: *mut u32 = access_pa_via_va(0x0c00_0000+4*0xa) as *mut u32;
    hart0_s_mode_interrupt_priority_serial_irq.write_volatile(1);

    let hart0_s_mode_interrupt_enables: *mut u32 = access_pa_via_va(0x0c00_2080) as *mut u32;
    hart0_s_mode_interrupt_enables.write_volatile(1 << 0xa);
}

pub unsafe fn enable_serial_interrupt() {
    let uart16550: *mut u8 = access_pa_via_va(0x10000000) as *mut u8;
    uart16550.add(4).write_volatile(0x0B);
    uart16550.add(1).write_volatile(0x01);
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(&mut tf.sepc),
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        Trap::Exception(Exception::InstructionPageFault) => page_fault(tf),
        Trap::Exception(Exception::LoadPageFault) => page_fault(tf),
        Trap::Exception(Exception::StorePageFault) => page_fault(tf),
        Trap::Exception(Exception::UserEnvCall) => syscall(tf),
        Trap::Interrupt(Interrupt::SupervisorExternal) => external(),
        _ => panic!("undefined trap!"),
    }
}

fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}

fn super_timer() {
    clock_set_next_event();
    tick();
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

fn external() {
    let _ = try_serial();
}

fn try_serial() -> bool {
    match super::io::getchar_option() {
        Some(ch) => {
            if ch == '\r' {
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
pub fn disable_and_store() -> usize {
    let sstatus: usize;
    unsafe {
        llvm_asm!("csrci sstatus, 1 << 1" : "=r"(sstatus) ::: "volatile");
    }
    sstatus
}

#[inline(always)]
pub fn restore(flags: usize) {
    unsafe {
        llvm_asm!("csrs sstatus, $0" :: "r"(flags) :: "volatile");
    }
}

#[inline(always)]
pub fn enable_and_wfi() {
    unsafe {
        llvm_asm!("csrsi sstatus, 1 << 1; wfi" :::: "volatile");
    }
}
