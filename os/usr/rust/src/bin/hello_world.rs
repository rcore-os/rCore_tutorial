#![no_std]
#![no_main]

#[macro_use]
extern crate rust;

#[no_mangle]
pub fn main() {
    for _ in 0..10 {
        println!("Hello world! from user mode program!");
    }
}
