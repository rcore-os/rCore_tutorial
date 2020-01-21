mod frame_allocator;

use crate::consts::*;
use buddy_system_allocator::LockedHeap;
use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;
use riscv::addr::Frame;
use riscv::register::satp;
use zircon_object::vm::*;

pub fn init(l: usize, r: usize) {
    FRAME_ALLOCATOR.lock().init(l, r);
    init_heap();
    kernel_remap();
    println!("++++ setup memory!    ++++");
}

pub fn alloc_frame() -> Option<Frame> {
    let ret = Some(Frame::of_ppn(FRAME_ALLOCATOR.lock().alloc()));
    trace!("alloc frame => {:x?}", ret);
    ret
}

pub fn dealloc_frame(f: Frame) {
    trace!("dealloc frame: {:x?}", f);
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
    let vmar = VmAddressRegion::new_root();

    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn ebss();
    }

    const OFFSET: usize = KERNEL_BEGIN_VADDR - KERNEL_BEGIN_PADDR;
    let vmo_text = unsafe {
        let vaddr = stext as usize;
        let pages = pages(etext as usize - vaddr);
        VMObjectPhysical::new(vaddr - OFFSET, pages)
    };
    let vmo_rodata = unsafe {
        let vaddr = srodata as usize;
        let pages = pages(erodata as usize - vaddr);
        VMObjectPhysical::new(vaddr - OFFSET, pages)
    };
    let vmo_data = unsafe {
        let vaddr = sdata as usize;
        let pages = pages(ebss as usize - vaddr);
        VMObjectPhysical::new(vaddr - OFFSET, pages)
    };

    vmar.map(
        stext as usize,
        vmo_text.clone(),
        0,
        vmo_text.len(),
        MMUFlags::READ | MMUFlags::EXECUTE,
    )
    .unwrap();
    vmar.map(
        srodata as usize,
        vmo_rodata.clone(),
        0,
        vmo_rodata.len(),
        MMUFlags::READ,
    )
    .unwrap();
    vmar.map(
        sdata as usize,
        vmo_data.clone(),
        0,
        vmo_data.len(),
        MMUFlags::READ | MMUFlags::WRITE,
    )
    .unwrap();

    let table_phys = vmar.table_phys();
    unsafe {
        satp::set(satp::Mode::Sv39, 0, table_phys >> 12);
        riscv::asm::sfence_vma_all();
    }
    core::mem::forget(vmar);
}

#[global_allocator]
static DYNAMIC_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}
