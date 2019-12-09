global_asm!(include_str!("boot/entry64.asm"));

use crate::consts::*;
use crate::memory::{
    alloc_frame,
    dealloc_frame
};

global_asm!(concat!(
    r#"
    .section .data
    .global _user_img_start
    .global _user_img_end
_user_img_start:
    .incbin ""#,
    env!("USER_IMG"),
    r#""
_user_img_end:
"#
));

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
	crate::fs::init();
    crate::process::init();
    crate::timer::init();
    crate::process::run();
    loop {}
}

