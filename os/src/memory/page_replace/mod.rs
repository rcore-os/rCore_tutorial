use {
    super::{alloc_frame, paging::PageTableImpl},
    crate::{
        consts::{PAGE_SIZE, PHYSICAL_MEMORY_OFFSET},
        fs::{disk_page_read, disk_page_write},
    },
    alloc::{boxed::Box, sync::Arc},
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
    fn push_frame(&mut self, vaddr: usize, weak_pt: Arc<Mutex<PageTableImpl>>);
    /// 选择要被置换的物理页帧
    fn choose_victim(&mut self) -> Option<(usize, Arc<Mutex<PageTableImpl>>)>;
    /// 1 (可选)复制页帧的内容到磁盘
    /// 2 并记录页帧所在磁盘位置到页表项中
    /// 3 返回可用的物理页帧
    fn swap_out_one(&mut self) -> Option<Frame> {
        while let Some((vaddr, pt)) = self.choose_victim() {
            let mut table = pt.lock();
            if let Some(entry) = table.get_entry(vaddr) {
                println!("SWAP_OUT:");
                let frame = Frame::of_addr(PhysAddr::new(entry.target()));
                entry.set_present(false);
                if entry.accessed() {
                    let swap_page: &mut [u8; PAGE_SIZE] =
                        unsafe { frame.as_kernel_mut(PHYSICAL_MEMORY_OFFSET) };
                    entry.set_target(disk_page_write(swap_page));
                    entry.set_replaced(true);
                }
                entry.update();
                println!("    vaddr {:x}", vaddr);
                return Some(frame);
            }
        }
        None
    }
    /// 处理缺页中断
    // TODO use crate::memory::PageEntry
    fn do_pgfault(&mut self, entry: &mut PageTableEntry, vaddr: usize) {
        let frame = self.swap_out_one().expect("failed to alloc frame");
        let mut flags = entry.flags().clone();
        if flags.contains(EF::RESERVED1) {
            println!("SWAP_IN:");
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
        println!(
            "frame loc: {:#x}",
            entry.addr().as_usize() + PHYSICAL_MEMORY_OFFSET
        );
    }
    /// 传递时钟中断（用于积极页面置换策略）
    fn tick(&self);
}

lazy_static! {
    pub static ref PAGE_REPLACE_HANDLER: Mutex<Box<dyn PageReplace>> =
        Mutex::new(Box::new(FifoPageReplace::default()));
}
