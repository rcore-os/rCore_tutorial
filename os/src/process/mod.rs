pub mod processor;
pub mod scheduler;
pub mod structs;
pub mod thread_pool;

use crate::fs::{INodeExt, ROOT_INODE};
use crate::interrupt::cpuid;
use alloc::boxed::Box;
use alloc::sync::Arc;
use processor::Processor;
use scheduler::RRScheduler;
use spin::Mutex;
use structs::Thread;
use thread_pool::ThreadPool;

pub type Tid = usize;
pub type ExitCode = usize;

static CPU: [Processor; 4] = [
    Processor::new(),
    Processor::new(),
    Processor::new(),
    Processor::new(),
];

pub fn init() {
    let scheduler = RRScheduler::new(1);
    let thread_pool = Arc::new(Mutex::new(ThreadPool::new(100, Box::new(scheduler))));
    for cpuid in 0..4 {
        CPU[cpuid].init(thread_pool.clone());
    }

    execute("rust/user_shell", None);

    println!("++++ setup process!   ++++");
}

pub fn execute(path: &str, host_tid: Option<Tid>) -> bool {
    let find_result = ROOT_INODE.lookup(path);
    match find_result {
        Ok(inode) => {
            let data = inode.read_as_vec().unwrap();
            let user_thread = unsafe { Thread::new_user(data.as_slice(), host_tid) };
            CPU[cpuid()].add_thread(user_thread);
            true
        }
        Err(_) => {
            println!("command not found!");
            false
        }
    }
}

pub fn tick() {
    CPU[cpuid()].tick();
}

pub fn run() -> ! {
    CPU[cpuid()].run();
}

pub fn exit(code: usize) {
    CPU[cpuid()].exit(code);
}

pub fn yield_now() {
    CPU[cpuid()].yield_now();
}
pub fn sleep() {
    CPU[cpuid()].sleep();
}
pub fn wake_up(tid: Tid) {
    CPU[cpuid()].wake_up(tid);
}
pub fn current_tid() -> usize {
    CPU[cpuid()].current_tid()
}
