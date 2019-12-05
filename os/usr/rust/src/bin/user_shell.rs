#![no_std]
#![no_main]
#![feature(alloc)]

extern crate alloc;

#[macro_use]
extern crate rust;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;

use rust::io::getc;
use rust::syscall::sys_exec;
use alloc::string::String;

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
                   sys_exec(line.as_ptr());
                   line.clear();
               }
               print!(">> ");
           },
           _ => {
               print!("{}", c as char);
               line.push(c as char);
           }
       }
   }
}
