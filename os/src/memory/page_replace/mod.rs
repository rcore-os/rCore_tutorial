use {
    super::alloc_frame,
    crate::{
        consts::{PAGE_SIZE, PHYSICAL_MEMORY_OFFSET},
        fs::{disk_page_read, disk_page_write},
    },
    alloc::boxed::Box,
    core::fmt::LowerHex,
    lazy_static::*,
    riscv::{
        addr::{Frame, PhysAddr},
        paging::{PageTableEntry, PageTableFlags as EF},
    },
    spin::Mutex,
};

mod fifo;

pub use fifo::FifoPageReplace;

pub trait PageReplace: Send {
    /// 将可被置换的物理页帧纳入算法
    fn push_frame(&mut self, frame: Frame, pg_entry: usize);
    /// 选择要被置换的物理页帧
    fn choose_victim(&mut self) -> Option<(Frame, usize)>;
    /// 1 复制页帧的内容到磁盘
    /// 2 并记录页帧所在磁盘位置到页表项中
    /// 3 返回可用的物理页帧
    fn swap_out_one(&mut self) -> Frame {
        let (frame, entry_loc) = self.choose_victim().unwrap();
        let swap_page: &mut [u8; PAGE_SIZE] =
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
        let new_page: &mut [u8; PAGE_SIZE] = unsafe { frame.as_kernel_mut(PHYSICAL_MEMORY_OFFSET) };
        disk_page_read(entry.addr().as_usize(), new_page);
        entry.flags_mut().set(EF::VALID, true);
        let flags = entry.flags();
        entry.set(frame, flags);
    }
    /// 传递时钟中断（用于积极页面置换策略）
    fn tick(&self);
}

lazy_static! {
    pub static ref PAGE_REPLACE_HANDLER: Mutex<Box<PageReplace>> =
        Mutex::new(Box::new(FifoPageReplace::default()));
}
