use core::cell::UnsafeCell;
use crate::alloc::boxed::Box;
use crate::process::Tid;
use crate::process::structs::*;
use crate::process::thread_pool::ThreadPool;
use crate::interrupt::{
    disable_and_store,
    enable_and_wfi,
    restore,
};

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
                inner.idle.switch_to(
                    &mut *inner.current.as_mut().unwrap().1
                );
                
                let (tid, thread) = inner.current.take().unwrap();
                println!("\nthread {} is switched out!", tid);

                inner.pool.retrieve(tid, thread);
            }
            else {
                enable_and_wfi();
                disable_and_store();
            }
        }
    }

    pub fn tick(&self) {
        let inner = self.inner();
        if !inner.current.is_none() {
            if inner.pool.tick() {
                let flags = disable_and_store();
                inner.current
                    .as_mut()
                    .unwrap()
                    .1
                    .switch_to(&mut inner.idle);
                restore(flags);
            }
        }
    }

    pub fn exit(&self, code: usize) -> ! {
        let inner = self.inner();
        let tid = inner.current.as_ref().unwrap().0;

        inner.pool.exit(tid, code);
        //println!("thread {} exited, exit code = {}", tid, code);

        inner.current
            .as_mut()
            .unwrap()
            .1
            .switch_to(&mut inner.idle);

        loop {}
    }
}
