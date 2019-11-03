use crate::io;
use crate::sbi;

use crate::consts::*;

global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn end();
    }
    println!("kernel end vaddr = 0x{:x}", end as usize);
    println!(
        "free physical memory ppn = [0x{:x}, 0x{:x})",
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
    );
    crate::interrupt::init();

    crate::memory::init(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
    );
    println!("alloc ppn = 0x{:x}", crate::memory::alloc());
    let t: usize = crate::memory::alloc();
    println!("alloc ppn = 0x{:x}", t);
    println!("alloc ppn = 0x{:x}", crate::memory::alloc());
    println!("dealloc ppn = 0x{:x}", t);
    crate::memory::dealloc(t);
    println!("alloc ppn = 0x{:x}", crate::memory::alloc());
    println!("alloc ppn = 0x{:x}", crate::memory::alloc());

    crate::timer::init();

    unsafe {
        asm!("ebreak"::::"volatile");
    }
    panic!("end of rust_main");
    loop {}
}
