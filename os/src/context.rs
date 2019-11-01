use riscv::register::{
    sstatus::Sstatus,
    scause::Scause
};

#[repr(C)]
pub struct TrapFrame {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub stval: usize,
    pub scause: Scause,
}

