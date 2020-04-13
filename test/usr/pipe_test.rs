#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

use user::io::*;
use user::syscall::{
    sys_pipe,
    sys_close,
    sys_read,
    sys_write,
    sys_fork,
};
use alloc::string::String;

#[no_mangle]
pub fn main() -> usize {
    let mut pipefd: [i32; 2] = [0; 2];
    sys_pipe(&mut pipefd);
    println!("fd_read = {}, fd_write = {}", pipefd[0], pipefd[1]);
    let pid = sys_fork();
    if pid == 0 {
        // child process, read from pipe
        // close write end of pipe
        sys_close(pipefd[1]);
        let mut string = String::from("");
        let ch: u8 = 0;
        loop {
            sys_read(pipefd[0] as usize, &ch as *const u8, 1);
            if ch == 0 {
                break;
            }
            string.push(ch as char);
        }
        println!("message received in child process = {}", string);
    } else {
        // parent process, write to pipe
        // close read end of pipe
        sys_close(pipefd[0]);
        let string = String::from("Hello world!");
        for ch in string.bytes() {
            sys_write(pipefd[1] as usize, &ch as *const u8, 1);
        }
        let ch: u8 = 0;
        sys_write(pipefd[1] as usize, &ch as *const u8, 1);
        println!("message sent to child process pid {}!", pid);
    }
    0
}
