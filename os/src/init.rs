global_asm!(include_str!("boot/entry64.asm"));

use crate::io;
use crate::sbi;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn _start();
        fn bootstacktop();
    }
    println!("_start vaddr = 0x{:x}", _start as usize);
    println!("bootstacktop vaddr = 0x{:x}", bootstacktop as usize);
    println!("hello world!");
    panic!("you want to do nothing!");
    loop {}
}
