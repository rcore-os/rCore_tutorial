#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;

use alloc::string::String;
use user::io::getc;
use user::io::putchar;
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
                    println!("searching for program {}", line);
                    line.push('\0');
                    sys_exec(line.as_ptr());
                    line.clear();
                }
                print!(">> ");
            }
            DL => {
                if !line.is_empty() {
                    putchar(BS as char);
                    putchar(' ');
                    putchar(BS as char);
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}
