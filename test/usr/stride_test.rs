#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::syscall::{
    set_priority, sys_exit as exit, sys_fork as fork, sys_getpid as getpid,
    sys_gettime as gettime_msec, sys_wait as waitpid,
};

fn spin_delay() {
    let mut j = true;
    for i in 0..200 {
        j = !j;
    }
}

#[no_mangle]
pub fn main() -> usize {
    const TOTAL: usize = 5;
    // to get enough accuracy, MAX_TIME (the running time of each process) should >1000 mseconds.
    let MAX_TIME = 1000;
    let mut acc: [usize; TOTAL] = [0; TOTAL];
    let mut status: [i32; TOTAL] = [0; TOTAL];
    let mut pids: [usize; TOTAL] = [0; TOTAL];
    set_priority(TOTAL + 1);
    for i in 0..TOTAL {
        acc[i] = 0;
        pids[i] = fork();
        if pids[i] == 0 {
            set_priority(i + 1);
            acc[i] = 0;
            loop {
                spin_delay();
                acc[i] += 1;
                if acc[i] % 4000 == 0 {
                    let time = gettime_msec();
                    if time > MAX_TIME {
                        println!("child pid {}, acc {}, time {}", getpid(), acc[i], time);
                        exit(acc[i]);
                    }
                }
            }
        }
    }
    println!("main: fork ok, now need to wait pids.");
    for i in 0..TOTAL {
        status[i] = 0;
        waitpid(pids[i], &mut status[i]);
        println!(
            "main: pid {}, acc {}, time {}",
            pids[i],
            status[i],
            gettime_msec()
        );
    }
    println!("main: wait pids over");
    print!("stride sched correct result:");
    for i in 0..TOTAL {
        println!(" {}", (status[i] * 2 / status[0] + 1) / 2);
    }
    println!("");
    return 0;
}

/*
out put:

main: fork ok, now need to wait pids.
stride sched correct result: 1 2 3 4 5
all user-mode processes have quit.
init check memory pass.
*/
