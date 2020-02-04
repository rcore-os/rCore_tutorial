#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::io::getc;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;

#[no_mangle]
pub fn main() {
    println!("Welcome to notebook!");
    loop {
        let c = getc();
        match c {
            LF | CR => {
                print!("{}", LF as char);
                print!("{}", CR as char)
            }
            _ => print!("{}", c as char),
        }
    }
}
