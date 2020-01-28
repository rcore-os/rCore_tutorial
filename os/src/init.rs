global_asm!(include_str!("boot/entry64.asm"));

use crate::consts::*;
use crate::memory::{
    alloc_frame,
    dealloc_frame
};

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    crate::interrupt::init();

	extern "C" {
		fn end();
	}
	crate::memory::init(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
    );
    // You can uncomment below lines for testing effects of PageTable
    //write_readonly_test();
    //execute_unexecutable_test();
    //read_invalid_test();

    crate::timer::init();
    loop {}
}

fn write_readonly_test() {
    extern "C" {
        fn srodata();
    }
    unsafe {
        let ptr = srodata as usize as *mut u8;
        *ptr = 0xab;
    }
}

fn execute_unexecutable_test() {
    extern "C" {
        fn sbss();
    }
    unsafe {
        asm!("jr $0" :: "r"(sbss as usize) :: "volatile");
    }
}

fn read_invalid_test() {
    println!("{}", unsafe { *(0x12345678 as usize as *const u8) });
}
