enum SyscallId {
    Fork = 57,
    Read = 63,
    Write = 64,
    Exit = 93,
    Exec = 221,
}

#[inline(always)]
fn sys_call(syscall_id: SyscallId, arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> i64 {
    let id = syscall_id as usize;
    let mut ret: i64;
    unsafe {
        asm!(
            "ecall"
            : "={x10}"(ret)
            : "{x17}"(id), "{x10}"(arg0), "{x11}"(arg1), "{x12}"(arg2), "{x13}"(arg3)
            : "memory"
            : "volatile"
        );
    }
    ret
}

pub fn sys_fork() -> i64 {
    sys_call(SyscallId::Fork, 0, 0, 0, 0)
}

pub fn sys_write(ch: u8) -> i64 {
    sys_call(SyscallId::Write, ch as usize, 0, 0, 0)
}

pub fn sys_exit(code: usize) -> ! {
    sys_call(SyscallId::Exit, code, 0, 0, 0);
    loop {}
}

pub fn sys_read(fd: usize, base: *const u8, len: usize) -> i64 {
    sys_call(SyscallId::Read, fd, base as usize, len, 0)
}

pub fn sys_exec(path: *const u8) {
    sys_call(SyscallId::Exec, path as usize, 0, 0, 0);
}
