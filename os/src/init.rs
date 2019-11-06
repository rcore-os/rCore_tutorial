use crate::io;
use crate::sbi;
use crate::consts::*;
use crate::memory::{
    alloc_frame,
    dealloc_frame
};
use crate::alloc::{
    boxed::Box,
    vec,
    vec::Vec,
    rc::Rc
};


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
    frame_allocating_test();
    dynamic_allocating_test();
    crate::timer::init();

    unsafe {
        asm!("ebreak"::::"volatile");
    }
    panic!("end of rust_main");
    loop {}
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
