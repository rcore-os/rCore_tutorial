global_asm!(include_str!("boot/entry64.asm"));
global_asm!(include_str!("link_user.S"));

use crate::consts::*;
use crate::memory::{alloc_frame, dealloc_frame};

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let FF_grade = FirstFitAllocator_test();
    extern "C" {
        fn end();
    }
    crate::memory::init(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12,
    );
    crate::interrupt::init();
    crate::fs::init();
    crate::process::init();
    crate::timer::init();
    page_test();
    println!("First Fit Allocator: {} / 8", FF_grade);
    crate::process::run();
    loop {}
}

use riscv::addr::Frame;

fn alloc(cnt: usize) -> Option<usize> {
    if let Some(frames) = crate::memory::alloc_frames(cnt) {
        return Some(frames.number());
    }
    return None;
}

fn dealloc(ppn: usize, cnt: usize) {
    crate::memory::dealloc_frames(Frame::of_ppn(ppn), cnt)
}

fn FirstFitAllocator_test() -> usize {
    let mut grade: usize = 0;
    crate::memory::init_allocator(1, 6);
    let mut p0 = alloc(5);
    if p0.is_none() {
        return grade;
    }
    let mut p0 = p0.unwrap();
    if !alloc(1).is_none() {
        return grade;
    }
    dealloc(p0 + 2, 3);
    if !alloc(4).is_none() {
        return grade;
    } else {
        grade += 1;
    }
    let mut p1 = alloc(3);
    if p1.is_none() {
        return grade;
    } else {
        grade += 1;
    }
    let mut p1 = p1.unwrap();
    if !alloc(1).is_none() {
        return grade;
    } else {
        grade += 1;
    }
    if p0 + 2 != p1 {
        return grade;
    } else {
        grade += 1;
    }
    let mut p2 = p0 + 1;
    dealloc(p0, 1);
    dealloc(p1, 3);
    p0 = alloc(1).unwrap();
    if p0 != p2 - 1 {
        return grade;
    } else {
        grade += 1;
    }
    dealloc(p0, 1);
    p0 = alloc(2).unwrap();
    if p0 != p2 + 1 {
        return grade;
    } else {
        grade += 1;
    }
    dealloc(p0, 2);
    dealloc(p2, 1);
    let mut p0 = alloc(5);
    if p0.is_none() {
        return grade;
    } else {
        grade += 1;
    }
    if !alloc(1).is_none() {
        return grade;
    } else {
        grade += 1;
    }
    dealloc(p0.unwrap(), 5);
    return grade;
}

use crate::memory::{
    access_pa_via_va,
    memory_set::{
        attr::MemoryAttr,
        handler::{ByFrameSwappingOut, ByFrameWithRpa, Linear},
        MemorySet,
    },
};

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
    loop {}
}

use crate::memory::paging::PageTableImpl;
use alloc::sync::Arc;
use spin::Mutex;
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
