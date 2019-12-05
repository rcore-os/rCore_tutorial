use crate::context::Context;
use crate::alloc::alloc::{
    alloc,
    dealloc,
    Layout,
};
use crate::consts::*;
use riscv::register::{
    satp,
    sstatus,
};
use crate::process::{ Tid, ExitCode };
use crate::alloc::{ sync::Arc, boxed::Box };
use xmas_elf::{
    header,
    program::{ Flags, SegmentData, Type },
    ElfFile,
};
use crate::memory::memory_set::{
    MemorySet,
    handler::ByFrame,
    attr::MemoryAttr,
};
use core::str;

pub struct Thread {
    pub context: Context,
    pub kstack: KernelStack,
    pub proc: Option<Arc<Process>>,
    pub wait: Option<Tid>,
}
impl Thread {
    pub fn new_idle() -> Box<Thread> {
        unsafe {
            let kstack = KernelStack::new();
            Box::new(Thread {
                context: Context::null(),
                kstack,
                proc: None,
                wait: None
            })
        }
    }

    pub fn new_kernel(entry: usize, arg: usize) -> Box<Thread> {
        unsafe {
            let kstack_ = KernelStack::new();
            Box::new(Thread {
                context: Context::new_kernel_thread(entry, arg, kstack_.top(), satp::read().bits()),
                kstack: kstack_,
                proc: None,
                wait: None
            })
        }
    }

    pub unsafe fn new_user(data: &[u8], wait_thread: Option<Tid>) -> Box<Thread> {
        let elf = ElfFile::new(data).expect("failed to analyse elf!");

        match elf.header.pt2.type_().as_type() {
            header::Type::Executable => {
                // println!("it really a executable!");
            },
            header::Type::SharedObject => {
                panic!("shared object is not supported!");
            },
            _ => {
                panic!("unsupported elf type!");
            }
        }

        let entry_addr = elf.header.pt2.entry_point() as usize;
        // println!("user entry addr = {:#x}", entry_addr);
        let mut vm = elf.make_memory_set();

        let mut ustack_top = {
            let (ustack_bottom, ustack_top) = (USER_STACK_OFFSET, USER_STACK_OFFSET + USER_STACK_SIZE);
            vm.push(
                ustack_bottom,
                ustack_top,
                MemoryAttr::new().set_user(),
                ByFrame::new(),
                None,
            );
            ustack_top
        };
        // println!("ustack = {:#x}, {:#x}", USER_STACK_OFFSET, ustack_top);


        let kstack = KernelStack::new();
        // println!("kstack top = {:#x}", kstack.top());

        Box::new(
            Thread {
                context: Context::new_user_thread(entry_addr, ustack_top, kstack.top(), vm.token()),
                kstack: kstack,
                proc: Some(
                    Arc::new(
                        Process {
                            vm: Arc::new(vm)
                        }
                    ),
                ),
                wait: wait_thread
            }
        )
    }

    pub fn switch_to(&mut self, target: &mut Thread) {
        unsafe {
            self.context.switch(&mut target.context);
        }
    }
}
        
    
pub struct KernelStack(usize);
impl KernelStack {
    pub fn new() -> Self {
        let bottom = unsafe {
            alloc(Layout::from_size_align(KERNEL_STACK_SIZE, KERNEL_STACK_SIZE).unwrap()) as usize
        };
        KernelStack(bottom)
    }
    pub fn top(&self) -> usize {
        self.0 + KERNEL_STACK_SIZE
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.0 as _,
                Layout::from_size_align(KERNEL_STACK_SIZE, KERNEL_STACK_SIZE).unwrap(),
            );
        }
    }
}

#[derive(Clone)]
pub enum Status {
    Ready,
    Running(Tid),
    Sleeping,
    Exited(ExitCode),
}

pub struct Process {
    vm: Arc<MemorySet>,
}

trait ElfExt {
    fn make_memory_set(&self) -> MemorySet;
}

impl ElfExt for ElfFile<'_> {
    fn make_memory_set(&self) -> MemorySet {
        let mut memory_set = MemorySet::new();
        // println!("new memory set initialized!");
        for ph in self.program_iter() {
            if ph.get_type() != Ok(Type::Load) {
                continue;
            }
            let vaddr = ph.virtual_addr() as usize;
            let mem_size = ph.mem_size() as usize;
            let data = match ph.get_data(self).unwrap() {
                SegmentData::Undefined(data) => data,
                _ => unreachable!(),
            };
            // println!("user segment vaddr[{:#x}, {:#x}) link in {:#x} length = {:#x}", vaddr, vaddr + mem_size, data.as_ptr() as usize, data.len()); 
            memory_set.push(
                vaddr,
                vaddr + mem_size,
                ph.flags().to_attr(),
                ByFrame::new(),
                Some((data.as_ptr() as usize, data.len())),
            );
        }
        memory_set
    }
}

trait ToMemoryAttr {
    fn to_attr(&self) -> MemoryAttr;
}

impl ToMemoryAttr for Flags {
    fn to_attr(&self) -> MemoryAttr {
        let mut flags = MemoryAttr::new().set_user();
        if self.is_execute() {
            flags = flags.set_execute();
        }
        flags
    }
}
