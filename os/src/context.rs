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
    pub content_addr: usize,
}
impl Context {
    pub unsafe fn null() -> Context {
        Context { content_addr: 0, }
    }

    pub unsafe fn new_kernel_thread(
        entry: usize,
        arg: usize,
        kstack_top: usize,
        satp: usize
        ) -> Context {

        ContextContent::new_kernel_thread(entry, arg, kstack_top, satp).push_at(kstack_top)
    }

    pub unsafe fn new_user_thread(
        entry: usize,
        ustack_top: usize,
        kstack_top: usize,
        satp: usize
    ) -> Self {
        ContextContent::new_user_thread(entry, ustack_top, satp).push_at(kstack_top)
    }

    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn switch(&mut self, target: &mut Context) {
        asm!(include_str!("process/switch.asm") :::: "volatile");
    }
}

#[repr(C)]
pub struct ContextContent {
    pub ra: usize,
    satp: usize,
    s: [usize; 12],
    tf: TrapFrame,
}

extern "C" {
    fn __trapret();
}

impl ContextContent {
    fn new_kernel_thread(
        //entry: extern "C" fn(usize) -> !,
        entry: usize,
        arg: usize,
        kstack_top: usize,
        satp: usize,
        ) -> ContextContent {
        
        let mut content = ContextContent {
            ra: __trapret as usize,
            satp,
            s: [0; 12],
            tf: {
                let mut tf: TrapFrame = unsafe { zeroed() };
                tf.x[2] = kstack_top;
                tf.x[10] = arg;
                //tf.sepc = entry as usize;
                tf.sepc = entry;
                tf.sstatus = sstatus::read();
                tf.sstatus.set_spp(sstatus::SPP::Supervisor);
                tf.sstatus.set_spie(true);
                tf.sstatus.set_sie(false);
                tf
            }
        };
        content

        /*
        let mut content: ContextContent = unsafe { zeroed() };
        content.ra = entry as usize;
        content.satp = satp;
        content.s[0] = arg;
        let mut sstatus_ = sstatus::read();
        sstatus_.set_spp(sstatus::SPP::Supervisor);
        sstatus_.set_sie(true);
        unsafe {
            asm!("csrr $0, sstatus" : "=r"(content.s[1]) ::: "volatile");
        }
        content
        */
    }

    fn new_user_thread(
        entry: usize,
        ustack_top: usize,
        satp: usize
    ) -> Self {
        ContextContent {
            ra: __trapret as usize,
            satp,
            s: [0; 12],
            tf: {
                let mut tf: TrapFrame = unsafe { zeroed() };
                tf.x[2] = ustack_top;
                tf.sepc = entry;
                tf.sstatus = sstatus::read();
                tf.sstatus.set_spie(true);
                tf.sstatus.set_sie(false);
                tf.sstatus.set_spp(sstatus::SPP::User);
                tf
            }
        }
    }

    unsafe fn push_at(self, stack_top: usize) -> Context {
        let ptr = (stack_top as *mut ContextContent).sub(1);
        *ptr = self;
        Context { content_addr: ptr as usize }
    }
}
