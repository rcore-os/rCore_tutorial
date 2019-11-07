use riscv::register::{
    sstatus::{
        self,
        Sstatus,
    },
    scause::Scause,
};
use core::mem::zeroed;

#[repr(C)]
pub struct TrapFrame {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub stval: usize,
    pub scause: Scause,
}

#[repr(C)]
pub struct Context {
    content_addr: usize,
}
impl Context {
    pub unsafe fn null() -> Context {
        Context { content_addr: 0, }
    }

    pub unsafe fn new_kernel_thread(
        entry: extern "C" fn(usize) -> !,
        arg: usize,
        kstack_top: usize,
        satp: usize
        ) -> Context {

        ContextContent::new_kernel_thread(entry, arg, kstack_top, satp).push_at(kstack_top)
    }

    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn switch(&mut self, target: &mut Context) {
        asm!(include_str!("process/switch.asm") :::: "volatile");
    }
}

#[repr(C)]
struct ContextContent {
    ra: usize,
    satp: usize,
    s: [usize; 12],
}

impl ContextContent {
    fn new_kernel_thread(
        entry: extern "C" fn(usize) -> !,
        arg: usize,
        kstack_top: usize,
        satp: usize,
        ) -> ContextContent {
        
        let mut content: ContextContent = unsafe { zeroed() };
        content.ra = entry as usize;
        content.satp = satp;
        content.s[0] = arg;
        let mut sstatus_ = sstatus::read();
        sstatus_.set_spp(sstatus::SPP::Supervisor);
        //content.s[1] = status_.bits();
        unsafe {
            asm!("csrr $0, sstatus" : "=r"(content.s[1]) ::: "volatile");
        }
        content
    }

    unsafe fn push_at(self, stack_top: usize) -> Context {
        let ptr = (stack_top as *mut ContextContent).sub(1);
        *ptr = self;
        Context { content_addr: ptr as usize }
    }
}
