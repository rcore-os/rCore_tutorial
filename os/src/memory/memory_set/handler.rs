use super::attr::MemoryAttr;
use crate::consts::PAGE_SIZE;
use crate::memory::access_pa_via_va;
use crate::memory::alloc_frame;
use crate::memory::paging::PageTableImpl;
use alloc::boxed::Box;
use core::fmt::Debug;

pub trait MemoryHandler: Debug + 'static {
    fn box_clone(&self) -> Box<dyn MemoryHandler>;
    fn map(&self, pt: &mut PageTableImpl, va: usize, attr: &MemoryAttr);
    fn unmap(&self, pt: &mut PageTableImpl, va: usize);
    fn page_copy(&self, pt: &mut PageTableImpl, va: usize, va_offset: usize, src: usize, length: usize);
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
    fn map(&self, pt: &mut PageTableImpl, va: usize, attr: &MemoryAttr) {
        attr.apply(pt.map(va, va - self.offset));
    }
    fn unmap(&self, pt: &mut PageTableImpl, va: usize) {
        pt.unmap(va);
    }
    fn page_copy(&self, pt: &mut PageTableImpl, va: usize, va_offset: usize, src: usize, length: usize) {
        let pa = pt.get_entry(va).expect("get pa error!").0.addr().as_usize();
        assert!(va == access_pa_via_va(pa));
        assert!(va == pa + self.offset);
        unsafe {
            let dst = core::slice::from_raw_parts_mut((va+va_offset) as *mut u8, PAGE_SIZE);
            if length > 0 {
                let src = core::slice::from_raw_parts(src as *const u8, PAGE_SIZE);
                dst[..length].clone_from_slice(&src[..length]);
            }
            #[allow(clippy::needless_range_loop)]
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

    fn map(&self, pt: &mut PageTableImpl, va: usize, attr: &MemoryAttr) {
        let frame = alloc_frame().expect("alloc_frame failed!");
        let pa = frame.start_address().as_usize();
        attr.apply(pt.map(va, pa));
    }

    fn unmap(&self, pt: &mut PageTableImpl, va: usize) {
        pt.unmap(va);
    }
    fn page_copy(&self, pt: &mut PageTableImpl, va: usize, va_offset: usize, src: usize, length: usize) {
        let pa = pt.get_entry(va).expect("get pa error!").0.addr().as_usize();
        unsafe {
            let dst = core::slice::from_raw_parts_mut((access_pa_via_va(pa)+va_offset) as *mut u8, PAGE_SIZE);
            if length > 0 {
                let src = core::slice::from_raw_parts(src as *const u8, PAGE_SIZE);
                dst[..length].clone_from_slice(&src[..length]);
            }
            #[allow(clippy::needless_range_loop)]
            for i in length..PAGE_SIZE {
                dst[i] = 0;
            }
        }
    }
}
