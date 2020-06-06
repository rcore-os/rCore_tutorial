use crate::sync::condvar::*;
use alloc::{collections::VecDeque, sync::Arc};
use lazy_static::*;
use spin::Mutex;

pub struct Stdin {
    buf: Mutex<VecDeque<char>>,
    pushed: Condvar,
}

impl Stdin {
    pub fn new() -> Self {
        Stdin {
            buf: Mutex::new(VecDeque::new()),
            pushed: Condvar::new(),
        }
    }

    pub fn push(&self, ch: char) {
        self.buf.lock().push_back(ch);
        self.pushed.notify();
    }

    pub fn pop(&self) -> char {
        loop {
            let ret = self.buf.lock().pop_front();
            match ret {
                Some(ch) => {
                    return ch;
                }
                None => {
                    self.pushed.wait();
                }
            }
        }
    }
}

lazy_static! {
    pub static ref STDIN: Arc<Stdin> = Arc::new(Stdin::new());
}
