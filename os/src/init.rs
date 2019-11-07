use crate::io;
use crate::sbi;
use crate::consts::*;
use crate::memory::{
    alloc_frame,
    dealloc_frame,
    print_frame_status,
};
use crate::alloc::{
    boxed::Box,
    vec,
    vec::Vec,
    rc::Rc
};


global_asm!(include_str!("boot/entry64.asm"));

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
    extern "C" {
        fn end();
        fn _user_img_start();
        fn _user_img_end();
    }
    println!("kernel end vaddr = 0x{:x}", end as usize);
    println!("user_img is at [{:#x}, {:#x})", _user_img_start as usize, _user_img_end as usize);
    loop {}
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
    //print_frame_status();
    //frame_allocating_test();
    //dynamic_allocating_test();
    //write_readonly_test();
    //execute_unexecutable_test();
    //read_invalid_test();
    crate::timer::init();

    crate::process::init(); 
    panic!("end of rust_main");
    loop {}
}

fn ebreak_test() {
    unsafe {
        asm!("ebreak" :::: "volatile");
    }
}
fn frame_allocating_test() {
    println!("alloc {:#x?}", alloc_frame());
    let f = alloc_frame();
    println!("alloc {:#x?}", f);
    println!("alloc {:#x?}", alloc_frame());
    println!("dealloc {:#x?}", f);
    dealloc_frame(f.unwrap());
    println!("alloc {:#x?}", alloc_frame());
    println!("alloc {:#x?}", alloc_frame());
}

fn dynamic_allocating_test() {
    let heap_value = Box::new(5);
    assert!(*heap_value == 5);
    println!("heap_value assertion successfully!");
    println!("heap_value is at {:p}", heap_value);
    
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    for i in 0..500 {
        assert!(vec[i] == i);
    }
    println!("vec assertion successfully!");
    println!("vec is at {:p}", vec.as_slice());

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

