global_asm!(include_str!("boot/entry64.asm"));

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
    println!("First Fit Allocator: {} / 8", FF_grade);
    crate::sbi::shutdown();
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
