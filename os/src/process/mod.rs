pub mod structs;
pub mod scheduler;
pub mod thread_pool;
pub mod processor;

use structs::Thread;
use processor::Processor;
use scheduler::RRScheduler;
use thread_pool::ThreadPool;
use alloc::boxed::Box;
use crate::fs::{
    ROOT_INODE,
    INodeExt
};

pub type Tid = usize;
pub type ExitCode = usize;


static CPU: Processor = Processor::new();

pub fn init() {
    let scheduler = RRScheduler::new(1);
    let thread_pool = ThreadPool::new(100, Box::new(scheduler));
    let idle = Thread::new_kernel(Processor::idle_main as usize);
    idle.append_initial_arguments([&CPU as *const Processor as usize, 0, 0]);
    CPU.init(idle, Box::new(thread_pool));

    for i in 0..5 {
        CPU.add_thread({
            let thread = Thread::new_kernel(hello_thread as usize);
            thread.append_initial_arguments([i, 0, 0]);
            thread
        });
    }

	let data = ROOT_INODE
        .lookup("rust/hello_world")
        .unwrap()
        .read_as_vec()
        .unwrap();
    let user_thread = unsafe { Thread::new_user(data.as_slice()) };
    CPU.add_thread(user_thread);

    println!("++++ setup process!   ++++");
}

#[no_mangle]
pub extern "C" fn hello_thread(arg: usize) -> ! {
    println!("begin of thread {}", arg);
    for i in 0..800 {
        print!("{}", arg);
	}
    println!("\nend  of thread {}", arg);
    CPU.exit(0);
    loop {}
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
