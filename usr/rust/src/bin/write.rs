#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

use user::io::*;
use user::syscall::{
    sys_open,
    sys_close,
    sys_read,
    sys_write,
};

#[no_mangle]
pub fn main() -> usize {
    let write_fd = sys_open("temp\0".as_ptr(), O_WRONLY);
    let mut text = "Hello world!\0";
    sys_write(write_fd as usize, text.as_ptr(), text.len());
    println!("write to file 'temp' successfully...");
    sys_close(write_fd as i32);

    let read_fd = sys_open("temp\0".as_ptr(), O_RDONLY);
    let mut read: [u8; 20] = [0; 20];
    sys_read(read_fd as usize, &read[0] as *const u8, 20);
    println!("read from file 'temp' successfully...");
    print!("content = ");
    for i in 0..20 {
        putchar(read[i] as char);
        if read[0] as u8 == 0 {
            break;
        }
    }
    putchar('\n');
    sys_close(read_fd as i32);
    0
}
