#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;

use alloc::string::String;
use user::io::getc;
use user::syscall::{sys_exec, sys_exit, sys_fork};

#[no_mangle]
pub fn main() {
    println!("Rust user shell");
    let mut line: String = String::new();
    print!(">> ");
    loop {
        let c = getc();
        match c {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    println!("searching for program {}", line);
                    if sys_fork() == 0 {
                        line.push('\0');
                        sys_exec(line.as_ptr());
                        sys_exit(0);
                    }
                    line.clear();
                }
                print!(">> ");
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}
