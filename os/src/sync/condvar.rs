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
        self.wait_queue.lock().push_back(current_tid());
        yield_now();
    }

    pub fn notify(&self) {
        let tid = self.wait_queue.lock().pop_front();
        if let Some(tid) = tid {
            wake_up(tid);
        }
        /* yield_now(); */
    }
}
