use {super::*, alloc::collections::VecDeque};

#[derive(Default)]
pub struct FifoPageReplace {
    frames: VecDeque<(Frame, usize)>,
}

impl PageReplace for FifoPageReplace {
    fn push_frame(&mut self, frame: Frame, pg_entry: usize) {
        println!("add frame: {:#x?} pg_entry: {:#x}", frame, pg_entry);
        self.frames.push_back((frame, pg_entry));
    }

    fn choose_victim(&mut self) -> Option<(Frame, usize)> {
        // 选择一个已经分配的物理页帧
        self.frames.pop_front()
    }

    fn tick(&self) {}
}
