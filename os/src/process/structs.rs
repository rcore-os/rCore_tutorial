use super::{ExitCode, Tid};
use crate::alloc::alloc::{alloc, dealloc, Layout};
use crate::consts::*;
use crate::context::{Context, ContextContent, TrapFrame};
use crate::memory::memory_set::{attr::MemoryAttr, handler::ByFrame, MemorySet};
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::str;
use riscv::register::satp;
use spin::Mutex;
use xmas_elf::{
    header,
    program::{Flags, SegmentData, Type},
    ElfFile,
};

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
    pub vm: Option<Arc<Mutex<MemorySet>>>,
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
                vm: None,
            })
        }
    }

    pub fn get_boot_thread() -> Box<Thread> {
        Box::new(Thread {
            context: Context::null(),
            kstack: KernelStack::new_empty(),
            wait: None,
            vm: None,
        })
    }

    pub fn append_initial_arguments(&self, args: [usize; 3]) {
        unsafe {
            self.context.append_initial_arguments(args);
        }
    }

    pub unsafe fn new_user_vm(data: &[u8]) -> (MemorySet, usize, usize) {
        let elf = ElfFile::new(data).expect("failed to analyse elf!");

        match elf.header.pt2.type_().as_type() {
            header::Type::Executable => {}
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
        (vm, entry_addr, ustack_top)
    }

    pub unsafe fn new_user(data: &[u8], wait_thread: Option<Tid>) -> Box<Thread> {
        let (vm, entry_addr, ustack_top) = unsafe { Self::new_user_vm(data) };

        let kstack = KernelStack::new();

        Box::new(Thread {
            context: Context::new_user_thread(entry_addr, ustack_top, kstack.top(), vm.token()),
            kstack: kstack,
            wait: wait_thread,
            vm: Some(Arc::new(Mutex::new(vm))),
        })
    }

    /// Fork a new process from current one
    pub fn fork(&self, tf: &TrapFrame) -> Box<Thread> {
        let kstack = KernelStack::new();
        let vm = self.vm.as_ref().unwrap().lock().clone();
        let vm_token = vm.token();
        let context = unsafe { Context::new_fork(tf, kstack.top(), vm_token) };
        Box::new(Thread {
            context,
            kstack,
            wait: self.wait.clone(),
            vm: Some(Arc::new(Mutex::new(vm))),
        })
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
