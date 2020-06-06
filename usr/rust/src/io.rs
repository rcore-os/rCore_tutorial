use crate::syscall::sys_read;
use crate::syscall::sys_write;
use core::fmt::{self, Write};

pub fn putchar(ch: char) {
    sys_write(STDOUT, &ch as *const char as *const u8, 1);
}

pub fn puts(s: &str) {
    for ch in s.chars() {
        putchar(ch);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;

pub fn getc() -> u8 {
    let mut c = 0u8;
    assert_eq!(sys_read(STDIN, &mut c, 1), 1);
    c
}

pub const O_RDONLY: i32 = 0; // 只读
pub const O_WRONLY: i32 = 1; // 只写
pub const O_RDWR: i32 = 2; // 可读可写
pub const O_CREAT: i32 = 64; // 打开文件时若文件不存在，创建它
pub const O_APPEND: i32 = 1024; // 从文件结尾开始写入
