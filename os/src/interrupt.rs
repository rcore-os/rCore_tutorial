use riscv::register::{
    scause::{
        self,
        Trap,
        Exception,
        Interrupt
    },
    sepc,
    stvec,
    sscratch,
    sstatus::{
        self,
        Sstatus
    }
};
use crate::timer::{
    TICKS,
    clock_set_next_event
};

use crate::context::TrapFrame;

global_asm!(include_str!("trap/trap.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            fn __alltraps();
        }        
        sscratch::write(0);
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);

	sstatus::set_sie();
    }
    println!("++++ setup interrupt! ++++");
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    match tf.scause.cause() {
        //Trap::Exception(Exception::Breakpoint) => breakpoint(&mut tf.sepc),
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => unknown(&mut tf.sstatus, &mut tf.scause.bits(), &mut tf.sepc),

    }
}

fn unknown(sstatus:&mut Sstatus, scause:&mut usize, sepc: &mut usize ) {
    println!("sstatus sie {:?}", sstatus.sie());
    println!("sstatus spie {:?}", sstatus.spie());
    println!("scause @0x{:x}", scause);
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}
fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}

fn super_timer() {
    clock_set_next_event();
    unsafe {
        TICKS += 1;
        if TICKS == 100 {
            TICKS = 0;
            println!("* 100 ticks *");
        }
    }
}

