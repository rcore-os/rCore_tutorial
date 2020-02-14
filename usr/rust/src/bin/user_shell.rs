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

unsafe fn to_cstr(s: &mut String) -> *const u8 {
    let ptr = s.as_mut_ptr();
    let len = s.len();
    *ptr.add(len) = 0;
    ptr
}

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
                        sys_exec(unsafe { to_cstr(&mut line) });
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
