use crate::sbi::set_timer;
use core::time::Duration;
use riscv::register::{sie, time};

const TICK_INTERVAL: u64 = 100000;

pub fn init() {
    unsafe {
        // enable timer interrupt
        sie::set_stimer();
    }
    clock_set_next_event();
    println!("++++ setup timer!     ++++");
}

pub fn clock_set_next_event() {
    set_timer(get_cycle() + TICK_INTERVAL);
}

fn get_cycle() -> u64 {
    time::read() as u64
}

/// Get current time (duration from start).
pub fn now() -> Duration {
    Duration::from_micros(get_cycle() / 10)
}
