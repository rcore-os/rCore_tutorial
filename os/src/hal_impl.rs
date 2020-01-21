use crate::memory::{alloc_frame, dealloc_frame};
use riscv::paging::PageTableEntry;
use riscv::register::satp;

#[no_mangle]
extern "C" fn hal_frame_alloc() -> Option<usize> {
    alloc_frame().map(|f| f.start_address().as_usize())
}

#[no_mangle]
extern "C" fn hal_frame_dealloc(paddr: &mut usize) {
    dealloc_frame(riscv::addr::Frame::of_addr(riscv::addr::PhysAddr::new(
        *paddr,
    )));
}

#[no_mangle]
unsafe extern "C" fn hal_pt_map_kernel(pt: *mut PageTableEntry) {
    // map physical memory region
    let current_root =
        (PMEM_BASE + satp::read().frame().start_address().as_usize()) as *const PageTableEntry;
    for i in 504..508 {
        let entry = current_root.add(i).read();
        pt.add(i).write(entry);
    }
}

#[export_name = "hal_pmem_base"]
static PMEM_BASE: usize = crate::consts::PHYSICAL_MEMORY_OFFSET;
