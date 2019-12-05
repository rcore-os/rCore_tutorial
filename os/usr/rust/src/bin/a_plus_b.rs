#![no_std]
#![no_main]
#![feature(alloc)]

extern crate alloc;

#[macro_use]
extern crate rust;

#[no_mangle]
pub fn main() -> usize {
    let a = 1;
    let b = 2;
    println!("a + b = {}", a + b);
    0
}
