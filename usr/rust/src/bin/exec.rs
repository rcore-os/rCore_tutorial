#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

use alloc::string::String;
use user::syscall::{sys_exec, sys_fork};

#[no_mangle]
pub fn main() -> usize {
    sys_exec("/rust/hello_world".as_ptr() as *const u8);
    println!("should not arrive here. exec error.");
    0
}
