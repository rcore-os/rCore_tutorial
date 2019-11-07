pub mod structs;
pub mod processor;
pub mod scheduler;
pub mod thread_pool;

use structs::Thread;
use processor::Processor;
use scheduler::RRScheduler;
use thread_pool::ThreadPool;
use crate::alloc::{
    vec::Vec,
    boxed::Box,
};
pub fn init() {
    /*
    let mut loop_thread = Thread::new_idle();
    let mut hello_thread = Thread::new_kernel(hello_thread, 666);
    loop_thread.switch_to(&mut hello_thread);
    */
    let scheduler = RRScheduler::new(1);
    let thread_pool = ThreadPool::new(100, Box::new(scheduler));
    CPU.init(Thread::new_idle(), Box::new(thread_pool));
    for i in 0..5 {
        CPU.add_thread(
            Thread::new_kernel(hello_thread, i)
        );
    }
    CPU.run();
}

#[no_mangle]
pub extern "C" fn hello_thread(arg: usize) -> ! {
    println!("begin of thread {}", arg);
    for i in 0..300 {
        print!("{}", arg);
    }
    println!("\nend  of thread {}", arg);
    CPU.exit(0);
}

pub type Tid = usize;
pub type ExitCode = usize;

static CPU: Processor = Processor::new();

pub fn tick() {
    CPU.tick();
}

pub fn exit(code: usize) {
    CPU.exit(code);
}
