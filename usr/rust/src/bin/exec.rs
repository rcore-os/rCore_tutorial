#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

use alloc::string::String;
use user::syscall::{sys_exec, sys_fork};

#[no_mangle]
pub fn main() -> usize {
    println!("this is exec_test ^o^");
    sys_exec("/rust/hello_world".as_ptr());
    println!("should not arrive here. exec error.");
    0
}
