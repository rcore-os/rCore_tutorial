#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;

use alloc::string::String;
use user::io::getc;
use user::syscall::sys_exec;

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
                    line.push('\0');
                    println!("searching for program {}", line);
                    sys_exec(line.as_ptr());
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
