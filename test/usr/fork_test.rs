#![no_std]
#![no_main]

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

/*
out put:

I am child
ret tid is: 0
thread 3 exited, exit code = 0
I am father
ret tid is: 3
thread 2 exited, exit code = 0
I am child
ret tid is: 0
thread 4 exited, exit code = 0
I am father
ret tid is: 4
thread 1 exited, exit code = 0
*/