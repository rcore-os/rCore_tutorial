use riscv::register::{
    scause,
    sepc,
    stvec,
    sscratch
};

pub fn init() {
    unsafe {
        sscratch::write(0);
        stvec::write(trap_handler as usize, stvec::TrapMode::Direct);
    }
    println!("++++ setup interrupt! ++++");
}

fn trap_handler() -> ! {
    let cause = scause::read().cause();
    let epc = sepc::read();
    println!("trap: cause: {:?}, epc: 0x{:#x}", cause, epc);
    panic!("trap handled!");
}
