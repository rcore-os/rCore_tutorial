use crate::context::Context;
use crate::alloc::alloc::{
    alloc,
    dealloc,
    Layout,
};
use crate::consts::*;
use riscv::register::satp;

pub struct Thread {
    pub context: Context,
    pub kstack: KernelStack,
}
impl Thread {
    pub fn new_idle() -> Thread {
        unsafe {
            Thread {
                context: Context::null(),
                kstack: KernelStack::new(),
            }
        }
    }

    pub fn new_kernel(entry: extern "C" fn(usize) -> !, arg: usize) -> Thread {
        unsafe {
            let kstack_ = KernelStack::new();
            Thread {
                context: Context::new_kernel_thread(entry, arg, kstack_.top(), satp::read().bits()),
                kstack: kstack_,
            }
        }
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

