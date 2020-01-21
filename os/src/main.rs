#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate log;
extern crate alloc;

#[macro_use]
mod io;

mod consts;
mod context;
mod hal_impl;
mod init;
mod interrupt;
mod lang_items;
mod memory;
mod sbi;
mod timer;
