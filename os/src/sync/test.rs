//! Dining philosophers problem
//!
//! The code is borrowed from [RustDoc - Dining Philosophers](https://doc.rust-lang.org/1.6.0/book/dining-philosophers.html)

use crate::process::{sleep, spawn};
use crate::sync::Condvar;
use alloc::vec;
use alloc::{sync::Arc, vec::Vec};
use core::time::Duration;
// use spin::Mutex;
use crate::sync::SleepLock as Mutex;

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
        sleep(Duration::from_secs(1));
    }
}

trait Table: Send + Sync {
    fn eat(&self, name: &str, left: usize, right: usize);
}

struct MutexTable {
    forks: Vec<Mutex<()>>,
}

impl Table for MutexTable {
    fn eat(&self, name: &str, left: usize, right: usize) {
        let _left = self.forks[left].lock();
        let _right = self.forks[right].lock();

        println!("{} is eating.", name);
        sleep(Duration::from_secs(1));
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

pub fn philosopher_using_mutex() {
    println!("philosophers using mutex");

    let table = Arc::new(MutexTable {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });
    philosopher(table);
}
