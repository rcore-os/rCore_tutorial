
#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

use user::syscall::sys_exec;

#[no_mangle]
pub fn main() -> usize {
    sys_fork();
    sys_fork();
    0
}