use core::sync::atomic::*;

global_asm!(include_str!("boot/entry64.asm"));
global_asm!(include_str!("link_user.S"));

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize) -> ! {
    if hartid != 0 {
        while !AP_CAN_INIT.load(Ordering::Relaxed) {
            spin_loop_hint();
        }
        other_main();
    }
    crate::memory::init();
    crate::interrupt::init();
    crate::interrupt::init_board();
    crate::fs::init();
    crate::process::init();
    crate::timer::init();

    AP_CAN_INIT.store(true, Ordering::Relaxed);

    crate::process::run();
    loop {}
}

fn other_main() {
    crate::interrupt::init();
    crate::memory::init_other();
    crate::timer::init();
    crate::process::run();
    loop {}
}

static AP_CAN_INIT: AtomicBool = AtomicBool::new(false);
