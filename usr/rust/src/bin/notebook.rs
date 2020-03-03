#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::io::getc;
use user::io::putchar;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const BS: u8 = 0x08u8;
const DL: u8 = 0x7fu8;

#[no_mangle]
pub fn main() {
    println!("Welcome to notebook!");
    let mut line_count = 0;
    loop {
        let c = getc();
        match c {
            LF | CR => {
                line_count = 0;
                print!("{}", LF as char);
                print!("{}", CR as char);
            }
            DL => if line_count > 0 {
                    putchar(BS as char);
                    putchar(' ');
                    putchar(BS as char);
                    line_count -= 1;
                }
            _ => {
                line_count += 1;
                print!("{}", c as char);
                
            }
            
        }
        
    }
}
