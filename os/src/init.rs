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
	crate::process::init();
    crate::timer::init();
    loop {}
}

