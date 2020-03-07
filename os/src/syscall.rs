use crate::context::TrapFrame;
use crate::process;
use crate::fs::file::FileDescriptorType;

pub const SYS_OPEN: usize = 56;
pub const SYS_CLOSE: usize = 57;
pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;
pub const SYS_READ: usize = 63;
pub const SYS_EXEC: usize = 221;

pub fn syscall(id: usize, args: [usize; 3], tf: &mut TrapFrame) -> isize {
    match id {
        SYS_OPEN => sys_open(args[0] as *const u8, args[1] as i32),
        SYS_CLOSE => sys_close(args[0] as i32),
        SYS_READ => unsafe { sys_read(args[0], args[1] as *mut u8, args[2]) },
        SYS_WRITE => unsafe { sys_write(args[0], args[1] as *const u8, args[2]) },
        SYS_EXIT => {
            sys_exit(args[0]);
            0
        }
        SYS_EXEC => sys_exec(args[0] as *const u8),
        _ => {
            panic!("unknown syscall id {}", id);
        }
    }
}

fn sys_open(path: *const u8, flags: i32) -> isize {
    let thread = process::current_thread_mut();
    let fd = thread.alloc_fd() as isize;
    thread.ofile[fd as usize]
        .as_ref()
        .unwrap()
        .lock()
        .open_file(unsafe { from_cstr(path) }, flags);
    fd
}

fn sys_close(fd: i32) -> isize {
    let thread = process::current_thread_mut();
    assert!(thread.ofile[fd as usize].is_some());
    thread.dealloc_fd(fd);
    0
}

fn sys_exit(code: usize) {
    process::exit(code);
}

unsafe fn sys_read(fd: usize, base: *mut u8, len: usize) -> isize {
    if fd == 0 {
        // 如果是标准输入
        unsafe {
            *base = crate::fs::stdio::STDIN.pop() as u8;
        }
        return 1;
    } else {
        let mut thread = process::current_thread_mut();
        assert!(thread.ofile[fd].is_some());
        let mut file = thread.ofile[fd as usize].as_ref().unwrap().lock();
        assert!(file.get_readable());
        match file.get_fdtype() {
            FileDescriptorType::FD_INODE => {
                let mut offset = file.get_offset();
                let s = file
                    .inode
                    .clone()
                    .unwrap()
                    .read_at(offset, core::slice::from_raw_parts_mut(base, len))
                    .unwrap();
                offset += s;
                file.set_offset(offset);
                return s as isize;
            }
            _ => {
                panic!("fdtype not handled!");
            }
        }
    }
}

unsafe fn sys_write(fd: usize, base: *const u8, len: usize) -> isize {
    if fd == 1 {
        assert!(len == 1);
        unsafe { crate::io::putchar(*base as char); }
        return 1;
    } else {
        let thread = process::current_thread_mut();
        assert!(thread.ofile[fd].is_some());
        let mut file = thread.ofile[fd as usize].as_ref().unwrap().lock();
        assert!(file.get_writable());
        match file.get_fdtype() {
            FileDescriptorType::FD_INODE => {
                let mut offset = file.get_offset();
                let s = file
                    .inode
                    .clone()
                    .unwrap()
                    .write_at(offset, core::slice::from_raw_parts(base, len))
                    .unwrap();
                offset += s;
                file.set_offset(offset);
                return s as isize;
            }
            _ => {
                panic!("fdtype not handled!");
            }
        }
        0
    }
}

pub unsafe fn from_cstr(s: *const u8) -> &'static str {
    use core::{slice, str};
    let len = (0usize..).find(|&i| *s.add(i) == 0).unwrap();
    str::from_utf8(slice::from_raw_parts(s, len)).unwrap()
}

fn sys_exec(path: *const u8) -> isize {
    let valid = process::execute(unsafe { from_cstr(path) }, Some(process::current_tid()));
    if valid {
        process::yield_now();
    }
    return 0;
}
