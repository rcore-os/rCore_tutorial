mod frame_allocator;
mod paging;

use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;
use crate::DYNAMIC_ALLOCATOR;

use riscv::addr::{
    VirtAddr,
    PhysAddr,
    Page,
    Frame
};
use crate::consts::*;

pub fn init(l: usize, r: usize) {
    FRAME_ALLOCATOR.lock().init(l, r);
    init_heap();
    println!("++++ setup memory!    ++++");
}

pub fn alloc_frame() -> Option<Frame> {
    Some(Frame::of_ppn(FRAME_ALLOCATOR.lock().alloc()))
}

pub fn dealloc_frame(f: Frame) {
    FRAME_ALLOCATOR.lock().dealloc(f.number())
}

fn init_heap() {
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

pub fn kernel_remap() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn end();
    }
    println!("kernel text [0x{:x}, 0x{:x})", stext as usize, etext as usize);
    println!("kernel rodata [0x{:x}, 0x{:x})", srodata as usize, erodata as usize);
    println!("kernel data [0x{:x}, 0x{:x})", sdata as usize, edata as usize);
    println!("here is kernel bootstrap stack!");
    println!("kernel bss [0x{:x}, 0x{:x})", sbss as usize, ebss as usize);
}
