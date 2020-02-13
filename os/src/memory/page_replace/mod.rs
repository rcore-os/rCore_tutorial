use {
    super::{alloc_frame, paging::PageTableImpl},
    crate::{
        consts::{PAGE_SIZE, PHYSICAL_MEMORY_OFFSET},
        fs::{disk_page_read, disk_page_write},
    },
    alloc::{
        boxed::Box,
        sync::{Arc, Weak},
    },
    core::fmt::LowerHex,
    lazy_static::*,
    riscv::{
        addr::{Frame, PhysAddr},
        asm::sfence_vma,
        paging::{PageTableEntry, PageTableFlags as EF},
    },
    spin::Mutex,
};

mod fifo;

pub use fifo::FifoPageReplace;

pub trait PageReplace: Send {
    /// 将可被置换的物理页帧纳入算法
    fn push_frame(&mut self, vaddr: usize, weak_pt: Weak<Mutex<PageTableImpl>>);
    /// 选择要被置换的物理页帧
    fn choose_victim(&mut self) -> Option<(usize, Weak<Mutex<PageTableImpl>>)>;
    /// 1 复制页帧的内容到磁盘
    /// 2 并记录页帧所在磁盘位置到页表项中
    /// 3 返回可用的物理页帧
    fn swap_out_one(&mut self) -> Option<Frame> {
        let mut check = true;
        while check {
            let (vaddr, weak_pt) = self.choose_victim().expect("failed to get frame");
            if let Some(pt) = weak_pt.upgrade() {
                let mut table = pt.lock();
                if let Some(entry) = table.get_entry(vaddr) {
                    let frame = Frame::of_addr(PhysAddr::new(entry.target()));
                    entry.set_present(false);
                    if entry.dirty() {
                        let swap_page: &mut [u8; PAGE_SIZE] =
                            unsafe { frame.as_kernel_mut(PHYSICAL_MEMORY_OFFSET) };
                        entry.set_target(disk_page_write(swap_page));
                        entry.set_replaced(true);
                    }
                    entry.update();
                    println!("swap out, entry: {:#x?}, vaddr {:x}", entry.0, vaddr);
                    return Some(frame);
                } else {
                    check = true;
                }
            } else {
                check = true;
            }
        }
        None
    }
    /// 处理缺页中断
    // TODO use crate::memory::PageEntry
    fn do_pgfault(&self, entry: &mut PageTableEntry, vaddr: usize) {
        println!("pgfault addr: {:#x?}", entry);
        let frame = alloc_frame().expect("failed to alloc frame");
        let mut flags = entry.flags().clone();
        if flags.contains(EF::RESERVED1) {
            println!("need swap in");
            let new_page: &mut [u8; PAGE_SIZE] =
                unsafe { frame.as_kernel_mut(PHYSICAL_MEMORY_OFFSET) };
            disk_page_read(entry.ppn() * PAGE_SIZE, new_page);
        }
        flags |= EF::VALID | EF::READABLE;
        flags &= !EF::RESERVED1;
        entry.set(frame, flags);
        unsafe {
            sfence_vma(0, vaddr & !(PAGE_SIZE - 1));
        }
        println!("swap in, entry: {:#x?}", entry);
    }
    /// 传递时钟中断（用于积极页面置换策略）
    fn tick(&self);
}

lazy_static! {
    pub static ref PAGE_REPLACE_HANDLER: Mutex<Box<PageReplace>> =
        Mutex::new(Box::new(FifoPageReplace::default()));
}
