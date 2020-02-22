#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::syscall::{sys_exit as exit, sys_yield as yield_now, sys_fork as fork, sys_wait as waitpid};

#[no_mangle]
pub fn main() -> isize {
    let magic: usize = 0x10384;
    let mut pid: usize = 0;
    let mut code: i32 = 0;
    println!("I am the parent. Forking the child...");
    pid = fork() as usize;
    if (pid == 0) {
        println!("I am the child.");
        yield_now();
        yield_now();
        yield_now();
        yield_now();
        yield_now();
        yield_now();
        yield_now();
        exit(magic);
    }
    else {
        println!("I am parent, fork a child pid {}", pid);
    }
    if pid <= 0 {
        panic!("pid <= 0");
    }
    println!("I am the parent, waiting now..");
    let wait_pid = waitpid(pid, &mut code);
    println!("{}, {:x}", wait_pid, code);
    if wait_pid != 0 || code != magic as i32 {
        panic!("wait_test1 fail");
    }
    if !(waitpid(pid, &mut code) != 0) {
        panic!("wait_test2 fail");
    }
    println!("waitpid {} ok.", pid);
    println!("wait_test pass.");
    return 0;
}

/*
out put:

I am the parent. Forking the child...
I am the child.
I am parent, fork a child pid 2
I am the parent, waiting now..
thread 2 exited, exit code = 66436
waitpid 2 ok.
wait_test pass.
thread 1 exited, exit code = 0
*/
