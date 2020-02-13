use crate::sbi::{get_cycle, set_timer};
use riscv::register::{sie, time};

pub static mut TICKS: usize = 0;

static TIMEBASE: u64 = 100000;
pub fn init() {
    unsafe {
        TICKS = 0;
        sie::set_ssoft();
    }
    clock_set_next_event();
    println!("++++ setup timer!     ++++");
}

pub fn clock_set_next_event() {
    set_timer(get_cycle() + TIMEBASE);
}
