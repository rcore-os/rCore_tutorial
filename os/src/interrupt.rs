use riscv::register::{
    scause,
    sepc,
    stvec,
    sscratch,
    sstatus
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
    println!("rust_trap!");
    let cause = scause::read().cause();
    let epc = sepc::read();
    println!("trap: cause: {:?}, epc: 0x{:#x}", cause, epc);

    tf.sepc += 2;
}
