use crate::process::{current_tid, wake_up, yield_now, Tid};
use alloc::collections::VecDeque;
use spin::Mutex;

#[derive(Default)]
pub struct Condvar {
    wait_queue: Mutex<VecDeque<Tid>>,
}

impl Condvar {
    pub fn new() -> Self {
        Condvar::default()
    }

    pub fn wait(&self) {
        // println!("tid_wait = {}", current_tid());
        self.wait_queue.lock().push_back(current_tid());
        yield_now();
    }

    pub fn notify(&self) {
        let tid = self.wait_queue.lock().pop_front();
        if let Some(tid) = tid {
            // println!("tid_to_wake_up = {}", tid);
            wake_up(tid);
        } else {
            panic!("no threads to wake up!");
        }
        /* yield_now(); */
    }
}
