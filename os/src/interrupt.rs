use crate::context::TrapFrame;
use crate::memory::access_pa_via_va;
use crate::process::tick;
use crate::timer::{clock_set_next_event, TICKS};
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
    }
    println!("++++ setup interrupt! ++++");
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(&mut tf.sepc),
        Trap::Interrupt(Interrupt::SupervisorSoft) => super_timer(),
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
    unsafe {
        // clear SSIP to acknowledge the interrupt
        asm!("csrc sip, $0" :: "r"(1 << 1));
    }
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
