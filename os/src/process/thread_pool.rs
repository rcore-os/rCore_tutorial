use crate::alloc::{boxed::Box, vec::Vec};
use crate::process::scheduler::Scheduler;
use crate::process::structs::*;
use crate::process::Tid;

pub struct ThreadInfo {
    pub status: Status,
    pub thread: Option<Box<Thread>>,
}

pub struct ThreadPool {
    pub threads: Vec<Option<ThreadInfo>>,
    scheduler: Box<dyn Scheduler>,
}

impl ThreadPool {
    pub fn new(size: usize, scheduler: Box<dyn Scheduler>) -> ThreadPool {
        ThreadPool {
            threads: {
                let mut v = Vec::new();
                v.resize_with(size, Default::default);
                v
            },
            scheduler,
        }
    }

    fn alloc_tid(&self) -> Tid {
        for (i, info) in self.threads.iter().enumerate() {
            if info.is_none() {
                return i;
            }
        }
        panic!("alloc tid failed!");
    }

    pub fn add(&mut self, _thread: Box<Thread>) -> Tid {
        let tid = self.alloc_tid();
        self.threads[tid] = Some(ThreadInfo {
            status: Status::Ready,
            thread: Some(_thread),
        });
        self.scheduler.push(tid);
        return tid;
    }

    pub fn acquire(&mut self) -> Option<(Tid, Box<Thread>)> {
        if let Some(tid) = self.scheduler.pop() {
            let mut thread_info = self.threads[tid].as_mut().expect("thread not exist!");
            thread_info.status = Status::Running(tid);
            return Some((tid, thread_info.thread.take().expect("thread not exist!")));
        } else {
            return None;
        }
    }

    pub fn retrieve(&mut self, tid: Tid, thread: Box<Thread>) {
        if self.threads[tid].is_none() {
            return;
        }
        let mut thread_info = self.threads[tid].as_mut().expect("thread not exist!");
        thread_info.thread = Some(thread);
        if let Status::Running(_) = thread_info.status {
            thread_info.status = Status::Ready;
            self.scheduler.push(tid);
        }
    }

    pub fn tick(&mut self) -> bool {
        let ret = self.scheduler.tick();
        ret
    }

    pub fn exit(&mut self, tid: Tid) {
        self.threads[tid] = None;
        self.scheduler.exit(tid);
    }

    pub fn wakeup(&mut self, tid: Tid) {
        let proc = self.threads[tid]
            .as_mut()
            .expect("thread not exist when waking up");
        proc.status = Status::Ready;
        self.scheduler.push(tid);
    }
}
