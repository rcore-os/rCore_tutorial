global_asm!(include_str!("boot/entry64.asm"));
global_asm!(include_str!("link_user.S"));

use crate::consts::*;
use crate::memory::{alloc_frame, dealloc_frame};

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn end();
    }
    crate::memory::init(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12,
    );
    crate::interrupt::init();
    crate::fs::init();
    crate::process::init();
    crate::process::spawn(philosopher_using_mutex);
    crate::timer::init();
    crate::process::run();
    loop {}
}

use crate::process::{sleep, spawn};
use crate::sync::SleepLock as Mutex;
use alloc::vec;
use alloc::{sync::Arc, vec::Vec};

struct Philosopher {
    name: &'static str,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &'static str, left: usize, right: usize) -> Philosopher {
        Philosopher { name, left, right }
    }

    fn eat(&self, table: &Arc<dyn Table>) {
        table.eat(self.name, self.left, self.right);
    }

    fn think(&self) {
        println!("{} is thinking.", self.name);
        sleep(1);
    }
}

trait Table: Send + Sync {
    fn eat(&self, name: &str, left: usize, right: usize);
}

struct MutexTable {
    forks: Vec<Mutex<usize>>,
}

impl Table for MutexTable {
    fn eat(&self, name: &str, left: usize, right: usize) {
        let left = self.forks[left].lock();
        let right = self.forks[right].lock();
        println!("{} is eating, using forks: {}, {}", name, *left, *right);
        sleep(1);
    }
}

fn philosopher(table: Arc<dyn Table>) {
    let philosophers = vec![
        Philosopher::new("1", 0, 1),
        Philosopher::new("2", 1, 2),
        Philosopher::new("3", 2, 3),
        Philosopher::new("4", 3, 4),
        Philosopher::new("5", 0, 4),
    ];

    for p in philosophers {
        let table = table.clone();
        spawn(move || {
            for i in 0..5 {
                p.think();
                p.eat(&table);
                println!("{} iter {} end.", p.name, i);
            }
        })
    }
}

fn philosopher_using_mutex() {
    println!("philosophers using mutex");

    let table = Arc::new(MutexTable {
        forks: vec![
            Mutex::new(0),
            Mutex::new(1),
            Mutex::new(2),
            Mutex::new(3),
            Mutex::new(4),
        ],
    });
    philosopher(table);
}
