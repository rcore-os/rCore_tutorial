#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

#[no_mangle]
pub fn main() -> usize {
    let a = 1;
    let b = 2;
    println!("a + b = {}", a + b);
    0
}
