global_asm!(include_str!("boot/entry64.asm"));

use crate::consts::*;
use crate::memory::{alloc_frame, dealloc_frame};
use crate::memory::{
    access_pa_via_va,
    memory_set::{
        attr::MemoryAttr,
        handler::{ByFrameSwappingOut, ByFrameWithRpa, Linear},
        MemorySet,
    },
};
use crate::memory::paging::PageTableImpl;
use alloc::sync::Arc;
use spin::Mutex;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn end();
    }
    crate::memory::init(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12,
    );
    crate::interrupt::init();
    page_test();
    crate::sbi::shutdown();
}

fn page_test() {
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
    memory_set.push(
        0x4000_0000,
        0x4000_8000,
        MemoryAttr::new(),
        ByFrameWithRpa::new(),
        None,
    );
    memory_set.push(
        0x4000_8000,
        0x4001_0000,
        MemoryAttr::new(),
        ByFrameSwappingOut::new(),
        None,
    );

    unsafe {
        memory_set.activate();
    }

    let table = memory_set.get_table();

    let ptr1 = unsafe { &mut *(0x4000_a000 as *mut u64) };
    *ptr1 = 0xdeaddead;
    let ptr2 = unsafe { &mut *(0x4000_c000 as *mut u64) };
    *ptr2 = 0xdeaddead;

    let mut count = 0;
    println!("test begin");
    count += check_a_to_b(&table, 0x4000_8000, 0x4000_0000);
    count += check_a_to_b(&table, 0x4000_9000, 0x4000_1000);
    count += check_a_to_b(&table, 0x4000_b000, 0x4000_2000);
    count += check_a_to_b(&table, 0x4000_d000, 0x4000_3000);
    count += check_a_to_b(&table, 0x4000_e000, 0x4000_4000);
    count += check_a_to_b(&table, 0x4000_f000, 0x4000_5000);
    count += check_a_to_b(&table, 0x4000_a000, 0x4000_6000);
    count += check_a_to_b(&table, 0x4000_c000, 0x4000_7000);
    println!("test end");
    println!("COUNT: {} / 8", count);
}

fn check_a_to_b(table: &Arc<Mutex<PageTableImpl>>, a: usize, b: usize) -> usize {
    let predicted = table.lock().get_entry(a).unwrap().target();
    let ptr = unsafe { &mut *(b as *mut u64) };
    *ptr = 0xdeaddead;
    let result = table.lock().get_entry(b).unwrap().target();
    if predicted == result {
        1
    } else {
        0
    }
}
