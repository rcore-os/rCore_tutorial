use super::{ExitCode, Tid};
use crate::alloc::alloc::{alloc, dealloc, Layout};
use crate::consts::*;
use crate::context::Context;
use crate::memory::memory_set::{attr::MemoryAttr, handler::ByFrame, MemorySet};
use alloc::boxed::Box;
use core::str;
use riscv::register::satp;
use xmas_elf::{
    header,
    program::{Flags, SegmentData, Type},
    ElfFile,
};
use crate::fs::file::File;
use spin::Mutex;
use alloc::sync::Arc;

#[derive(Clone)]
pub enum Status {
    Ready,
    Running(Tid),
    Sleeping,
    Exited(ExitCode),
}

pub struct Thread {
    pub context: Context,
    pub kstack: KernelStack,
    pub wait: Option<Tid>,
    pub ofile: [Option<Arc<Mutex<File>>>; NOFILE],
}

impl Thread {
    pub fn switch_to(&mut self, target: &mut Thread) {
        unsafe {
            self.context.switch(&mut target.context);
        }
    }

    pub fn new_kernel(entry: usize) -> Box<Thread> {
        unsafe {
            let kstack_ = KernelStack::new();
            Box::new(Thread {
                context: Context::new_kernel_thread(entry, kstack_.top(), satp::read().bits()),
                kstack: kstack_,
                wait: None,
                ofile: [None; NOFILE],
            })
        }
    }

    pub fn get_boot_thread() -> Box<Thread> {
        Box::new(Thread {
            context: Context::null(),
            kstack: KernelStack::new_empty(),
            wait: None,
            ofile: [None; NOFILE],
        })
    }

    pub fn append_initial_arguments(&self, args: [usize; 3]) {
        unsafe {
            self.context.append_initial_arguments(args);
        }
    }

    pub unsafe fn new_user(data: &[u8], wait_thread: Option<Tid>) -> Box<Thread> {
        let elf = ElfFile::new(data).expect("failed to analyse elf!");

        match elf.header.pt2.type_().as_type() {
            header::Type::Executable => {
                // println!("it really a executable!");
            }
            header::Type::SharedObject => {
                panic!("shared object is not supported!");
            }
            _ => {
                panic!("unsupported elf type!");
            }
        }
        let entry_addr = elf.header.pt2.entry_point() as usize;
        let mut vm = elf.make_memory_set();

        let mut ustack_top = {
            let (ustack_bottom, ustack_top) =
                (USER_STACK_OFFSET, USER_STACK_OFFSET + USER_STACK_SIZE);
            vm.push(
                ustack_bottom,
                ustack_top,
                MemoryAttr::new().set_user(),
                ByFrame::new(),
                None,
            );
            ustack_top
        };

        let kstack = KernelStack::new();

        let mut thread = Thread {
            context: Context::new_user_thread(entry_addr, ustack_top, kstack.top(), vm.token()),
            kstack: kstack,
            wait: wait_thread,
            ofile: [None; NOFILE],
        };
        for i in 0..3 {
            thread.ofile[i] = Some(Arc::new(Mutex::new(File::default())));
        }
        Box::new(thread)
        
    }

    // 分配文件描述符
    pub fn alloc_fd(&mut self) -> i32 {
        let mut fd = 0;
        for i in 0usize..NOFILE {
            if self.ofile[i].is_none() {
                fd = i;
                break;
            }
        }
        self.ofile[fd] = Some(Arc::new(Mutex::new(File::default())));
        fd as i32
    }
    // 回收文件描述符
    pub fn dealloc_fd(&mut self, fd: i32) {
        assert!(self.ofile[fd as usize].is_some());
        self.ofile[fd as usize] = None;
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
    pub fn new_empty() -> Self {
        KernelStack(0)
    }
    pub fn top(&self) -> usize {
        self.0 + KERNEL_STACK_SIZE
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        if self.0 != 0 {
            unsafe {
                dealloc(
                    self.0 as _,
                    Layout::from_size_align(KERNEL_STACK_SIZE, KERNEL_STACK_SIZE).unwrap(),
                );
            }
        }
    }
}

trait ElfExt {
    fn make_memory_set(&self) -> MemorySet;
}

impl ElfExt for ElfFile<'_> {
    fn make_memory_set(&self) -> MemorySet {
        let mut memory_set = MemorySet::new();
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
