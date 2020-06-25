#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

#[no_mangle]
pub fn main() -> usize {
    for _ in 0..1000 {
        let mut temp: u32 = 3;
        for _ in 0..1000 {
            temp = temp * 5 % 100007;
        }
        println!("1000 mutiplies completed, temp = {}", temp);
    }
    println!("mission successed!");
    0
}
