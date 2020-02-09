use super::alloc_frame;
use crate::consts::PHYSICAL_MEMORY_OFFSET;
use crate::fs::{disk_page_read, disk_page_write};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::LowerHex;
use lazy_static::*;
use riscv::addr::{Frame, PhysAddr};
use riscv::paging::{PageTableEntry, PageTableFlags as EF};
use spin::Mutex;

pub trait PageReplace: Send {
    /// 将可被置换的物理页帧纳入算法
    fn push_frame(&mut self, frame: Frame, pg_entry: usize);
    /// 选择要被置换的物理页帧
    fn choose_victim(&mut self) -> (Frame, usize);
    /// 1 复制页帧的内容到磁盘
    /// 2 并记录页帧所在磁盘位置到页表项中
    /// 3 返回可用的物理页帧
    fn swap_out_one(&mut self) -> Frame {
        let (frame, entry_loc) = self.choose_victim();
        let swap_page: &mut [u8; (1 << 12)] =
            unsafe { frame.as_kernel_mut(PHYSICAL_MEMORY_OFFSET) };
        let entry: &mut PageTableEntry = unsafe { &mut *(entry_loc as *mut PageTableEntry) };
        let mut flags = entry.flags().clone();
        flags.set(EF::VALID, false);
        let pg_addr = disk_page_write(swap_page);
        let disk_frame = Frame::of_addr(PhysAddr::new(pg_addr));
        entry.set(disk_frame, flags);
        println!("{:#x?}", entry);
        frame
    }
    /// 处理缺页中断
    fn do_pgfault(&self, entry: &mut PageTableEntry) {
        println!("pgfault addr: {:#x}", entry.addr().as_usize());
        let frame = alloc_frame().unwrap();
        let new_page: &mut [u8; (1 << 12)] = unsafe { frame.as_kernel_mut(PHYSICAL_MEMORY_OFFSET) };
        disk_page_read(entry.addr().as_usize(), new_page);
        entry.flags_mut().set(EF::VALID, true);
        let flags = entry.flags();
        entry.set(frame, flags);
    }
    /// 传递时钟中断（用于积极页面置换策略）
    fn tick(&self);
}

pub struct FifoPageReplace {
    frames: Vec<(Frame, usize)>,
}

impl PageReplace for FifoPageReplace {
    fn push_frame(&mut self, frame: Frame, pg_entry: usize) {
        println!("add frame: {:#x?} pg_entry: {:#x}", frame, pg_entry);
        self.frames.push((frame, pg_entry));
    }

    fn choose_victim(&mut self) -> (Frame, usize) {
        // 选择一个已经分配的物理页帧
        self.frames.remove(0)
    }

    fn tick(&self) {}
}

lazy_static! {
    pub static ref PAGE_REPLACE_HANDLER: Mutex<Box<PageReplace>> =
        Mutex::new(Box::new(FifoPageReplace { frames: Vec::new() }));
}
