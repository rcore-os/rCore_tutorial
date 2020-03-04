#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

use user::syscall::sys_fork;

#[no_mangle]
pub fn main() -> usize {
    let tid = sys_fork();
    let tid = sys_fork();
    if tid == 0 {
        println!("I am child");
    } else {
        println!("I am father");
    }
    println!("ret tid is: {}", tid);
    0
}
