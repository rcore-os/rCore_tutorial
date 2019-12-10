use crate::context::TrapFrame;
use crate::process;

pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;

pub fn syscall(id: usize, args: [usize; 3], tf: &mut TrapFrame) -> isize {
    match id {
        SYS_WRITE => {
            print!("{}", args[0] as u8 as char);
            0
        },
        SYS_EXIT => {
            sys_exit(args[0]);
            0
        },
        _ => {
            panic!("unknown syscall id {}", id);
        },
    }
}

fn sys_exit(code: usize) {
    process::exit(code);
}
