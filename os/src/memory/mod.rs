mod frame_allocator;
pub mod memory_set;
pub mod paging;

use crate::consts::*;
use buddy_system_allocator::LockedHeap;
use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;
use memory_set::{attr::MemoryAttr, handler::Linear, MemorySet};
use riscv::addr::{Frame, Page, PhysAddr, VirtAddr};
use riscv::register::sstatus;

pub fn init() {
    unsafe {
        sstatus::set_sum();
    }
    extern "C" {
        fn end();
    }
    let l = ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1;
    let r = PHYSICAL_MEMORY_END >> 12;
    FRAME_ALLOCATOR.lock().init(l, r);
    init_heap();
    kernel_remap();
    println!("++++ setup memory!    ++++");
}

pub fn init_other() {
    unsafe {
        sstatus::set_sum(); // Allow user memory access
        asm!("csrw satp, $0; sfence.vma" :: "r"(SATP) :: "volatile");
    }
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

pub const fn access_pa_via_va(pa: usize) -> usize {
    pa + PHYSICAL_MEMORY_OFFSET
}

pub fn kernel_remap() {
    let mut memory_set = MemorySet::new();

    extern "C" {
        fn bootstack();
        fn bootstacktop();
    }
    memory_set.push(
        bootstack as usize,
        bootstacktop as usize,
        MemoryAttr::new(),
        Linear::new(PHYSICAL_MEMORY_OFFSET),
        None,
    );
    memory_set.push(
        access_pa_via_va(0x0c00_2000),
        access_pa_via_va(0x0c00_3000),
        MemoryAttr::new(),
        Linear::new(PHYSICAL_MEMORY_OFFSET),
        None,
    );
    memory_set.push(
        access_pa_via_va(0x1000_0000),
        access_pa_via_va(0x1000_1000),
        MemoryAttr::new(),
        Linear::new(PHYSICAL_MEMORY_OFFSET),
        None,
    );

    unsafe {
        memory_set.activate();
        SATP = memory_set.token();
    }
}

// First core stores its SATP here.
// Other cores load it later.
static mut SATP: usize = 0;

#[global_allocator]
static DYNAMIC_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}
