use {
    super::*,
    alloc::{collections::VecDeque, sync::Weak},
    spin::Mutex,
};

#[derive(Default)]
pub struct FifoPageReplace {
    frames: VecDeque<(usize, Weak<Mutex<PageTableImpl>>)>,
}

impl PageReplace for FifoPageReplace {
    fn push_frame(&mut self, vaddr: usize, weak_pt: Weak<Mutex<PageTableImpl>>) {
        println!("add vaddr: {:#x?}", vaddr);
        self.frames.push_back((vaddr, weak_pt));
    }

    fn choose_victim(&mut self) -> Option<(usize, Weak<Mutex<PageTableImpl>>)> {
        // 选择一个已经分配的物理页帧
        self.frames.pop_front()
    }

    fn tick(&self) {}
}
