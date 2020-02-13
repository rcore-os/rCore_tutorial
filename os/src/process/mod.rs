pub mod processor;
pub mod scheduler;
pub mod structs;
pub mod thread_pool;

use crate::fs::{INodeExt, ROOT_INODE};
use alloc::boxed::Box;
use processor::Processor;
use scheduler::RRScheduler;
use structs::Thread;
use thread_pool::ThreadPool;

pub type Tid = usize;
pub type ExitCode = usize;

static CPU: Processor = Processor::new();

pub fn init() {
    let scheduler = RRScheduler::new(1);
    let thread_pool = ThreadPool::new(100, Box::new(scheduler));
    let idle = Thread::new_kernel(Processor::idle_main as usize);
    idle.append_initial_arguments([&CPU as *const Processor as usize, 0, 0]);
    CPU.init(idle, Box::new(thread_pool));
    // CPU.add_thread(Thread::new_kernel(crate::drivers::virtio_disk::virtio_disk_test as usize));
    execute("rust/user_shell", None);

    println!("++++ setup process!   ++++");
}

pub fn execute(path: &str, host_tid: Option<Tid>) -> bool {
    // println!("before lookup!");
    let find_result = ROOT_INODE.lookup(path);
    // println!("after loopup!");
    match find_result {
        Ok(inode) => {
            // println!("Ok(inode)!");
            let data = inode.read_as_vec().unwrap();
            // println!("ok data!");
            let user_thread = unsafe { Thread::new_user(data.as_slice(), host_tid) };
            CPU.add_thread(user_thread);
            // println!("CPU.add_thread");
            true
        }
        Err(_) => {
            println!("command not found!");
            false
        }
    }
}

pub fn tick() {
    CPU.tick();
}

pub fn run() {
    CPU.run();
}

pub fn exit(code: usize) {
    CPU.exit(code);
}

pub fn yield_now() {
    CPU.yield_now();
}

pub fn wake_up(tid: Tid) {
    CPU.wake_up(tid);
}

pub fn current_tid() -> usize {
    CPU.current_tid()
}

pub fn add_thread(thread: Box<Thread>) -> usize {
    CPU.add_thread(thread)
}

pub fn current_thread() -> &'static Box<Thread> {
    CPU.current_thread()
}
