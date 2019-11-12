pub mod frame_allocator;
pub mod paging;
pub mod memory_set;

use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;
use crate::DYNAMIC_ALLOCATOR;
use memory_set::{
    MemorySet,
    attr::MemoryAttr,
    handler::Linear
};
use riscv::addr::{
    VirtAddr,
    PhysAddr,
    Page,
    Frame
};
use crate::consts::*;
use riscv::register::sstatus;
pub fn access_pa_via_va(pa: usize) -> usize {
    pa + PHYSICAL_MEMORY_OFFSET
}

pub fn init(l: usize, r: usize) {
    unsafe {
        sstatus::set_sum();
    }
    FRAME_ALLOCATOR.lock().init(l, r);
    init_heap();
    kernel_remap();
    println!("++++ setup memory!    ++++");
}

pub fn alloc_frame() -> Option<Frame> {
    Some(Frame::of_ppn(FRAME_ALLOCATOR.lock().alloc()))
}

pub fn dealloc_frame(f: Frame) {
    FRAME_ALLOCATOR.lock().dealloc(f.number())
}

pub fn print_frame_status() {
    FRAME_ALLOCATOR.lock().print_allocating_status();
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
    let mut memory_set = MemorySet::new();
    extern "C" {
        fn bootstack();
        fn bootstacktop();
    }
    /*
    memory_set.push(
        bootstack as usize,
        bootstacktop as usize,
        MemoryAttr::new(),
        Linear::new(PHYSICAL_MEMORY_OFFSET),
        None,
    );
    */
    unsafe {
        memory_set.activate();
    }
}
/*
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
*/
