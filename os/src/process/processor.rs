use crate::context::{Context, ContextContent};
use crate::interrupt::*;
use crate::process::structs::*;
use crate::process::thread_pool::ThreadPool;
use crate::process::Tid;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::cell::UnsafeCell;
use spin::Mutex;

pub struct ProcessorInner {
    pool: Arc<Mutex<ThreadPool>>,
    context: Context,
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

    pub fn init(&self, pool: Arc<Mutex<ThreadPool>>) {
        unsafe {
            *self.inner.get() = Some(ProcessorInner {
                pool,
                context: Context::null(),
                current: None,
            });
        }
    }

    fn inner(&self) -> &mut ProcessorInner {
        unsafe { &mut *self.inner.get() }
            .as_mut()
            .expect("Processor is not initialized!")
    }

    pub fn add_thread(&self, thread: Box<Thread>) {
        self.inner().pool.lock().add(thread);
    }

    pub fn run(&self) -> ! {
        let inner = self.inner();
        disable_and_store();

        loop {
            inner.current = inner.pool.lock().acquire();
            if let Some((tid, thread)) = &mut inner.current {
                // println!("\n>>>> will switch_to thread {} in idle_main!", tid);
                unsafe {
                    inner.context.switch(&mut thread.context);
                }
                // println!("\n<<<< switch_back to idle in idle_main!");
                let (tid, thread) = inner.current.take().unwrap();
                inner.pool.lock().retrieve(tid, thread);
            } else {
                enable_and_wfi();
                disable_and_store();
            }
        }
    }

    pub fn tick(&self) {
        let inner = self.inner();
        if !inner.current.is_none() {
            if inner.pool.lock().tick() {
                self.yield_now();
            }
        }
    }

    pub fn exit(&self, code: usize) -> ! {
        disable_and_store();
        let inner = self.inner();
        let tid = inner.current.as_ref().unwrap().0;

        inner.pool.lock().exit(tid);
        println!("thread {} exited, exit code = {}", tid, code);

        if let Some(wait) = inner.current.as_ref().unwrap().1.wait {
            inner.pool.lock().wakeup(wait);
        }

        self.yield_now();
        unreachable!();
    }

    pub fn yield_now(&self) {
        let inner = self.inner();
        unsafe {
            let flags = disable_and_store();
            let current_context = &mut inner.current.as_mut().unwrap().1.context;
            current_context.switch(&mut inner.context);
            restore(flags);
        }
    }

    pub fn wake_up(&self, tid: Tid) {
        let inner = self.inner();
        inner.pool.lock().wakeup(tid);
    }

    pub fn sleep(&self) {
        let inner = self.inner();
        inner.pool.lock().threads[self.current_tid()]
            .as_mut()
            .unwrap()
            .status = Status::Sleeping;
        self.yield_now();
    }

    pub fn current_tid(&self) -> usize {
        if let Some((tid, _)) = self.inner().current {
            tid
        } else {
            0
        }
    }
}
