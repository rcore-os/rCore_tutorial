use super::{attr::MemoryAttr, handler::MemoryHandler};
use crate::consts::PAGE_SIZE;
use crate::memory::paging::{PageRange, PageTableImpl};
use alloc::boxed::Box;

#[derive(Debug, Clone)]
pub struct MemoryArea {
    pub start: usize,
    pub end: usize,
    pub handler: Box<dyn MemoryHandler>,
    pub attr: MemoryAttr,
}

impl MemoryArea {
    pub fn map(&self, pt: &mut PageTableImpl) {
        for page in PageRange::new(self.start, self.end) {
            self.handler.map(pt, page, &self.attr);
        }
    }
    fn unmap(&self, pt: &mut PageTableImpl) {
        for page in PageRange::new(self.start, self.end) {
            self.handler.unmap(pt, page);
        }
    }

    pub fn is_overlap_with(&self, start_addr: usize, end_addr: usize) -> bool {
        let p1 = self.start / PAGE_SIZE;
        let p2 = (self.end - 1) / PAGE_SIZE + 1;
        let p3 = start_addr / PAGE_SIZE;
        let p4 = (end_addr - 1) / PAGE_SIZE + 1;
        !((p1 >= p4) || (p2 <= p3))
    }

    pub fn new(
        start_addr: usize,
        end_addr: usize,
        handler: Box<dyn MemoryHandler>,
        attr: MemoryAttr,
    ) -> Self {
        MemoryArea {
            start: start_addr,
            end: end_addr,
            handler: handler,
            attr: attr,
        }
    }

    pub fn page_copy(&self, pt: &mut PageTableImpl, src: usize, length: usize) {
        let mut l = length;
        let mut s = src;
        for page in PageRange::new(self.start, self.end) {
            self.handler
                .page_copy(pt, page, s, if l < PAGE_SIZE { l } else { PAGE_SIZE });
            s += PAGE_SIZE;
            if l >= PAGE_SIZE {
                l -= PAGE_SIZE;
            }
        }
    }
}
