pub mod handler;
pub mod attr;
pub mod area;

use area::MemoryArea;
use attr::MemoryAttr;
use crate::memory::paging::PageTableImpl;
use crate::consts::*;
use handler::{
    MemoryHandler,
    Linear
};
use alloc::{
    boxed::Box,
    vec::Vec
};
use crate::memory::access_pa_via_va;

pub struct MemorySet {
    areas: Vec<MemoryArea>,
    page_table: PageTableImpl,
}

impl MemorySet {
    pub fn new() -> Self {
        let mut memory_set = MemorySet {
            areas: Vec::new(),
            page_table: PageTableImpl::new_bare(),
        };
        memory_set.map_kernel_and_physical_memory();
        memory_set
    }
    pub fn map_kernel_and_physical_memory(&mut self) {
        // println!("map_kernel_and_physical_memory!");
        extern "C" {
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
            fn sbss();
            fn ebss();
            fn bootstack();
            fn bootstacktop();
            fn end();
        }
        let offset = PHYSICAL_MEMORY_OFFSET;
        // println!(".section .text");
        self.push(
            stext as usize,
            etext as usize,
            MemoryAttr::new().set_readonly().set_execute(),
            Linear::new(offset),
            None,
        );
        // println!(".section .rodata");
        self.push(
            srodata as usize,
            erodata as usize,
            MemoryAttr::new().set_readonly(),
            Linear::new(offset),
            None,
        );
        // println!(".section .data");
        self.push(
            sdata as usize,
            edata as usize,
            MemoryAttr::new(),
            Linear::new(offset),
            None,
        );
        // println!(".section .bss");
        self.push(
            sbss as usize,
            ebss as usize,
            MemoryAttr::new(),
            Linear::new(offset),
            None,
        );
        // println!(".section .physical memory");
        self.push(
            (end as usize / PAGE_SIZE + 1) * PAGE_SIZE, 
            access_pa_via_va(PHYSICAL_MEMORY_END),
            MemoryAttr::new(),
            Linear::new(offset),
            None,
        );
        // println!(".section .bootstack");
        self.push(
            bootstack as usize,
            bootstacktop as usize,
            MemoryAttr::new(),
            Linear::new(offset),
            None
        );
        // println!(".section .mmio.init_external_interrupt");
        self.push(
            access_pa_via_va(0x0c00_2000),
            access_pa_via_va(0x0c00_3000),
            MemoryAttr::new(),
            Linear::new(offset),
            None
        );
        // println!(".section .mmio.enable_serial_interrupt");
        self.push(
            access_pa_via_va(0x1000_0000),
            access_pa_via_va(0x1000_1000),
            MemoryAttr::new(),
            Linear::new(offset),
            None
        );

    }
    pub fn push(&mut self, start: usize, end: usize, attr: MemoryAttr, handler: impl MemoryHandler, data: Option<(usize, usize)>) {
        // println!("in push: [{:#x},{:#x})", start, end);
        assert!(start <= end, "invalid memory area!");
        assert!(self.test_free_area(start, end), "memory area overlap!");
        let area = MemoryArea::new(start, end, Box::new(handler), attr);
        //println!("before area.map");
        area.map(&mut self.page_table);
        //println!("after area.map");
        if let Some((src, length)) = data {
            area.page_copy(&mut self.page_table, src, length);
        }
        self.areas.push(area);
        
    } 
    
    fn test_free_area(&self, start: usize, end: usize) -> bool {
        self.areas
            .iter()
            .find(|area| area.is_overlap_with(start, end))
            .is_none()
    }
    pub unsafe fn activate(&self) {
        self.page_table.activate();
    }
    pub fn token(&self) -> usize {
        self.page_table.token()
    }
}
