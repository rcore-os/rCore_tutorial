#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

use alloc::string::String;
use user::syscall::{sys_exec, sys_fork};

unsafe fn to_cstr(s: &mut String) -> *const u8 {
    let ptr = s.as_mut_ptr();
    let len = s.len();
    *ptr.add(len) = 0;
    ptr
}

#[no_mangle]
pub fn main() -> usize {
    println!("this is exec_test ^o^");
    sys_exec("/rust/hello_world\0".as_ptr());
    println!("should not arrive here. exec error.");
    0
}
