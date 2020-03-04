use super::super::page_replace::PAGE_REPLACE_HANDLER;
use super::attr::MemoryAttr;
use crate::consts::PAGE_SIZE;
use crate::memory::access_pa_via_va;
use crate::memory::alloc_frame;
use crate::memory::paging::PageTableImpl;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::fmt::Debug;
use spin::Mutex;

pub trait MemoryHandler: Debug + 'static {
    fn box_clone(&self) -> Box<dyn MemoryHandler>;
    fn map(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize, attr: &MemoryAttr);
    fn unmap(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize);
    fn page_copy(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize, src: usize, length: usize);
}

impl Clone for Box<dyn MemoryHandler> {
    fn clone(&self) -> Box<dyn MemoryHandler> {
        self.box_clone()
    }
}

#[derive(Debug, Clone)]
pub struct Linear {
    offset: usize,
}

impl Linear {
    pub fn new(off: usize) -> Self {
        Linear { offset: off }
    }
}
impl MemoryHandler for Linear {
    fn box_clone(&self) -> Box<dyn MemoryHandler> {
        Box::new(self.clone())
    }
    fn map(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize, attr: &MemoryAttr) {
        attr.apply(pt.lock().map(va, va - self.offset));
    }
    fn unmap(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize) {
        pt.lock().unmap(va);
    }
    fn page_copy(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize, src: usize, length: usize) {
        let pa = pt
            .lock()
            .get_entry(va)
            .expect("get pa error!")
            .0
            .addr()
            .as_usize();
        assert!(va == access_pa_via_va(pa));
        assert!(va == pa + self.offset);
        unsafe {
            let dst = core::slice::from_raw_parts_mut(va as *mut u8, PAGE_SIZE);
            if length > 0 {
                let src = core::slice::from_raw_parts(src as *const u8, PAGE_SIZE);
                for i in 0..length {
                    dst[i] = src[i];
                }
            }
            for i in length..PAGE_SIZE {
                dst[i] = 0;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ByFrame;
impl ByFrame {
    pub fn new() -> Self {
        ByFrame {}
    }
}
impl MemoryHandler for ByFrame {
    fn box_clone(&self) -> Box<dyn MemoryHandler> {
        Box::new(self.clone())
    }

    fn map(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize, attr: &MemoryAttr) {
        let frame = alloc_frame().expect("alloc_frame failed!");
        let pa = frame.start_address().as_usize();
        attr.apply(pt.lock().map(va, pa));
    }

    fn unmap(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize) {
        pt.lock().unmap(va);
    }
    fn page_copy(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize, src: usize, length: usize) {
        let pa = pt
            .lock()
            .get_entry(va)
            .expect("get pa error!")
            .0
            .addr()
            .as_usize();
        unsafe {
            let dst = core::slice::from_raw_parts_mut(access_pa_via_va(pa) as *mut u8, PAGE_SIZE);
            if length > 0 {
                let src = core::slice::from_raw_parts(src as *const u8, PAGE_SIZE);
                for i in 0..length {
                    dst[i] = src[i];
                }
            }
            for i in length..PAGE_SIZE {
                dst[i] = 0;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ByFrameWithRpa;
impl ByFrameWithRpa {
    pub fn new() -> Self {
        ByFrameWithRpa {}
    }
}
impl MemoryHandler for ByFrameWithRpa {
    fn box_clone(&self) -> Box<dyn MemoryHandler> {
        Box::new(self.clone())
    }

    fn map(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize, attr: &MemoryAttr) {
        let frame = alloc_frame().expect("alloc_frame failed!");
        let pa = frame.start_address().as_usize();
        let mut table = pt.lock();
        let mut entry = table.map(va, pa);
        entry.clear_dirty();
        entry.clear_accessed();
        attr.apply(entry);
        PAGE_REPLACE_HANDLER.lock().push_frame(va, pt.clone());
    }

    fn unmap(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize) {
        pt.lock().unmap(va);
    }
    fn page_copy(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize, src: usize, length: usize) {
        let pa = pt
            .lock()
            .get_entry(va)
            .expect("get pa error!")
            .0
            .addr()
            .as_usize();
        unsafe {
            let dst = core::slice::from_raw_parts_mut(access_pa_via_va(pa) as *mut u8, PAGE_SIZE);
            if length > 0 {
                let src = core::slice::from_raw_parts(src as *const u8, PAGE_SIZE);
                for i in 0..length {
                    dst[i] = src[i];
                }
            }
            for i in length..PAGE_SIZE {
                dst[i] = 0;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ByFrameSwappingOut;
impl ByFrameSwappingOut {
    pub fn new() -> Self {
        ByFrameSwappingOut {}
    }
}
impl MemoryHandler for ByFrameSwappingOut {
    fn box_clone(&self) -> Box<dyn MemoryHandler> {
        Box::new(self.clone())
    }

    fn map(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize, attr: &MemoryAttr) {
        let frame = PAGE_REPLACE_HANDLER
            .lock()
            .swap_out_one()
            .expect("failed to swap out page");
        let pa = frame.start_address().as_usize();
        let mut table = pt.lock();
        let mut entry = table.map(va, pa);
        entry.clear_dirty();
        entry.clear_accessed();
        attr.apply(entry);
        PAGE_REPLACE_HANDLER.lock().push_frame(va, pt.clone());
    }

    fn unmap(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize) {
        pt.lock().unmap(va);
    }
    fn page_copy(&self, pt: Arc<Mutex<PageTableImpl>>, va: usize, src: usize, length: usize) {
        let pa = pt
            .lock()
            .get_entry(va)
            .expect("get pa error!")
            .0
            .addr()
            .as_usize();
        unsafe {
            let dst = core::slice::from_raw_parts_mut(access_pa_via_va(pa) as *mut u8, PAGE_SIZE);
            if length > 0 {
                let src = core::slice::from_raw_parts(src as *const u8, PAGE_SIZE);
                for i in 0..length {
                    dst[i] = src[i];
                }
            }
            for i in length..PAGE_SIZE {
                dst[i] = 0;
            }
        }
    }
}
