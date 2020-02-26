#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::syscall::{
    set_priority, sys_exit as exit, sys_fork as fork, sys_gettime as gettime_msec,
};

fn spin_delay() {
    let mut j = true;
    for i in 0..10 {
        j = !j;
    }
}

#[no_mangle]
pub fn main() -> usize {
    const TOTAL: usize = 5;
    // to get enough accuracy, MAX_TIME (the running time of each process) should > 1000 mseconds.
    let MAX_TIME = 1000;
    set_priority(TOTAL + 1);
    let start_time = gettime_msec();
    for i in 0..TOTAL {
        let pids = fork() as usize;
        if pids == 0 {
            let mut acc = 0;
            set_priority(i + 1);
            loop {
                spin_delay();
                acc += 1;
                if acc % 400 == 0 {
                    let time = gettime_msec() - start_time;
                    if time > MAX_TIME {
                        exit(acc);
                    }
                }
            }
        }
    }
    println!("main: fork ok.");
    return 0;
}

/*
out put:

main: fork ok.
thread 1 exited, exit code = 0
>> thread 6 exited, exit code = 517600
thread 5 exited, exit code = 408400
thread 3 exited, exit code = 210400
thread 4 exited, exit code = 316800
thread 2 exited, exit code = 111600

// 多出来的 `>>` 是由于目前 rcore 的 wait/fork 不完善导致的
// 等一位哥哥来修复
// 到时候测试估计就检察 exit code
// 检察方式（大概）：
// sort(code, code + 5);
// for i in 0..5 {
//     assert!((code[i] * 2 / code[0] + 1) / 2 == i + 1);
// }
*/
