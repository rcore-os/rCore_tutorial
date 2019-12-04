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
use lazy_static::lazy_static;
use core::cell::UnsafeCell;
use crate::fs::{
    ROOT_INODE,
    INodeExt
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
    println!("CPU init successfully!");

    /*
    println!("hello_thread is at {:#x}", hello_thread as usize);
    for i in 0..5 {
        CPU.add_thread(
            Thread::new_kernel(hello_thread as usize, i)
        );
    }
    */

    /*
    extern "C" {
        fn _user_img_start();
        fn _user_img_end();
    }
    let data = unsafe {
        core::slice::from_raw_parts(
            _user_img_start as *const u8,
            _user_img_end as usize - _user_img_start as usize,
        )
    };
    let user_thread = unsafe { Thread::new_user(data) };
    CPU.add_thread(user_thread);
    */

    /*
    let data = ROOT_INODE
        .lookup("rust/user_shell")
        .unwrap()
        .read_as_vec()
        .unwrap();
    println!("size of program {:#x}", data.len());
    let user_thread = unsafe { Thread::new_user(data.as_slice()) };
    CPU.add_thread(user_thread);
    */
    println!("++++ setup process!   ++++");
}

pub fn run() {
    CPU.run();
}

#[no_mangle]
pub extern "C" fn hello_thread(arg: usize) -> ! {
    println!("begin of thread {}", arg);
    /*
    let a = 1000000;
    let b = 10000;
    for i in 0..a {
        if (i + 1) % b == 0 {
            println!("arg = {}, i = {}/{}", arg, i + 1, a);
        }
    }
    */
    for i in 0..800 {
        print!("{}", arg);
    }
    println!("\nend  of thread {}", arg);
    CPU.exit(0);
    loop {}
}

pub type Tid = usize;
pub type ExitCode = usize;


static CPU: Processor = Processor::new();

pub fn tick() {
    //println!("ready CPU.tick()");
    CPU.tick();
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
