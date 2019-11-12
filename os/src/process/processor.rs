use core::cell::UnsafeCell;
use alloc::boxed::Box;
use crate::process::Tid;
use crate::process::structs::*;
use crate::process::thread_pool::ThreadPool;
use crate::interrupt::*;
use crate::context::ContextContent;

pub struct ProcessorInner {
    pool: Box<ThreadPool>,
    idle: Box<Thread>,
    current: Option<(Tid, Box<Thread>)>,
}    

pub struct Processor {
    inner: UnsafeCell<Option<ProcessorInner>>,
}

unsafe impl Sync for Processor {}

impl Processor {
    pub const fn new() -> Processor {
        Processor {
            inner: UnsafeCell::new(None),
        }
    }

    pub fn init(&self, idle: Box<Thread>, pool: Box<ThreadPool>) {
        unsafe {
            *self.inner.get() = Some(
                ProcessorInner {
                    pool,
                    idle,
                    current: None,
                }
            );
        }
    }

    fn inner(&self) -> &mut ProcessorInner {
        unsafe { &mut *self.inner.get() }
            .as_mut()
            .expect("Processor is not initialized!")
    }

    pub fn add_thread(&self, thread: Box<Thread>) {
        self.inner().pool.add(thread);
    }

    pub fn run(&self) -> ! {
        let inner = self.inner();
        disable_and_store();

        loop {
            if let Some(thread) = inner.pool.acquire() {
                inner.current = Some(thread);
                println!("\n>>>> will switch_to thread {} in CPU.run()!", inner.current.as_mut().unwrap().0);
                //println!("current content = {:#x}", &(inner.current.as_mut().unwrap().1.context) as *const _ as usize);
                //println!("current content_addr = {:#x}", inner.current.as_mut().unwrap().1.context.content_addr);

                /*
                println!("inner.idle.context_addr = {:#x}", inner.idle.context.content_addr);
                println!("idle ra = {:#x}", unsafe { (*(inner.idle.context.content_addr as *const ContextContent)).ra });
                */
                inner.idle.switch_to(
                    &mut *inner.current.as_mut().unwrap().1
                );
                
                println!("<<<< switch_back to idle in CPU.run()!");
                let (tid, thread) = inner.current.take().unwrap();
                //println!("thread {} is switched out!", tid);

                inner.pool.retrieve(tid, thread);
            }
            else {
                enable_and_wfi();
                disable_and_store();
            }
        }
    }

    pub fn tick(&self) {
        //println!("CPU.tick() starts.");
        let inner = self.inner();
        //println!("inner got!");
        if !inner.current.is_none() {
            //println!("inner.current is not none!");
            if inner.pool.tick() {
                let flags = disable_and_store();
                //println!("sie status is = {}", riscv::register::sstatus::read().sie());
                //println!("\nready switch to inner.idle in CPU.tick().");
                //println!("current context = {:#x}", &inner.current.as_mut().unwrap().1.context as *const _ as usize);
                //println!("current content_addr = {:#x}", inner.current.as_mut().unwrap().1.context.content_addr);
                //println!("idle context = {:#x}", &inner.idle.context as *const _ as usize);
                //println!("idle content_addr = {:#x}", inner.idle.context.content_addr);
                //println!("current satp = {:#x}", riscv::register::satp::read().bits());
                println!("");
                inner.current
                    .as_mut()
                    .unwrap()
                    .1
                    .switch_to(&mut inner.idle);
                //println!("ready to restore flags!");
                restore(flags);
            }
        }
        else {
            //println!("inner.current.is_none() is true!");
        }
        //println!("CPU.tick() ends.");
    }

    pub fn exit(&self, code: usize) -> ! {
        disable_and_store();
        let inner = self.inner();
        let tid = inner.current.as_ref().unwrap().0;

        inner.pool.exit(tid, code);
        println!("thread {} exited, exit code = {}", tid, code);
        //println!("satp = {:#x}", riscv::register::satp::read().bits());
        //println!("inner.idle.context_addr = {:#x}", inner.idle.context.content_addr);
        //println!("idle ra = {:#x}", unsafe { (*(inner.idle.context.content_addr as *const ContextContent)).ra });
        inner.current
            .as_mut()
            .unwrap()
            .1
            .switch_to(&mut inner.idle);

        loop {}
    }
}
