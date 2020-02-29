#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

#[no_mangle]
pub fn main() -> usize {
    println!("test begin");
    let byte6 = unsafe { &mut *(0x4000_6000 as *mut u8) };
    *byte6 = 0xFF;
    let byte5 = unsafe { &mut *(0x4000_5000 as *mut u8) };
    *byte5 = 0x01;
    let byte0 = unsafe { &mut *(0x4000_0000 as *mut u8) };
    *byte0 = 0x10;
    let byte1 = unsafe { &mut *(0x4000_1000 as *mut u8) };
    *byte1 = 0x11;
    assert_eq!(*byte6, 0xFF);
    assert_eq!(*byte5, 0x01);
    println!("test end");
    0
}
