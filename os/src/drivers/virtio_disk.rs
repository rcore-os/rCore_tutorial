use crate::memory::{
    access_pa_via_va,
    get_pa_via_va,
    paging
};
use spin::Mutex;
use crate::consts::{
    PAGE_SIZE,
    PHYSICAL_MEMORY_OFFSET
};
use crate::sync::condvar;
use riscv::register::{
    sstatus,
    sie
};
use crate::interrupt::{
    disable_and_store,
    restore,
    enable_and_wfi,
    disable_timer_and_store,
    restore_timer,
    enable_and_store,
};

const VIRTIO_MMIO_0: usize = 0x10001000;
const VIRTIO_MMIO_MAGIC_VALUE: usize = 0x000;
const VIRTIO_MMIO_VERSION: usize = 0x004;
const VIRTIO_MMIO_DEVICE_ID: usize = 0x008;
const VIRTIO_MMIO_VENDOR_ID: usize = 0x00c;
const VIRTIO_MMIO_DEVICE_FEATURES: usize = 0x010;
const VIRTIO_MMIO_DRIVER_FEATURES: usize = 0x020;
const VIRTIO_MMIO_GUEST_PAGE_SIZE: usize = 0x028;
const VIRTIO_MMIO_QUEUE_SEL: usize = 0x030;
const VIRTIO_MMIO_QUEUE_NUM_MAX: usize = 0x034;
const VIRTIO_MMIO_QUEUE_NUM: usize = 0x038;
const VIRTIO_MMIO_QUEUE_ALIGN: usize = 0x03c;
const VIRTIO_MMIO_QUEUE_PFN: usize = 0x040;
const VIRTIO_MMIO_QUEUE_READY: usize = 0x044;
const VIRTIO_MMIO_QUEUE_NOTIFY: usize = 0x050;
const VIRTIO_MMIO_INTERRUPT_STATUS: usize = 0x060;
const VIRTIO_MMIO_INTERRUPT_ACK: usize = 0x064;
const VIRTIO_MMIO_STATUS: usize = 0x070;

const VIRTIO_MMIO_MAGIC_NUMBER: u32 = 0x74726976;
const VIRTIO_MMIO_VENDOR_NUMBER: u32 = 0x554d4551;

// VIRTIO_MMIO_STATUS register bits
const VIRTIO_CONFIG_S_ACKNOWLEDGE: u32 = 1;
const VIRTIO_CONFIG_S_DRIVER: u32 = 2;
const VIRTIO_CONFIG_S_DRIVER_OK: u32 = 4;
const VIRTIO_CONFIG_S_FEATURES_OK: u32 = 8;

// device feature bits
const VIRTIO_BLK_F_RO: usize = 5;
const VIRTIO_BLK_F_SCSI: usize = 7;
const VIRTIO_BLK_F_CONFIG_WCE: usize = 11;
const VIRTIO_BLK_F_MQ: usize = 12;
const VIRTIO_F_ANY_LAYOUT: usize = 27;
const VIRTIO_RING_F_INDIRECT_DESC: usize = 28;
const VIRTIO_RING_F_EVENT_IDX: usize = 29;

const NUM: usize = 8;

fn get_reg<T>(offset: usize) -> *mut T {
    let p = access_pa_via_va(VIRTIO_MMIO_0 + offset);
    p as *mut T 
} 

#[no_mangle]
fn reg_read<T>(offset: usize) -> T where T: Copy{
    unsafe {
        let p = get_reg::<T>(offset);
        *p
    }
}

#[no_mangle]
fn reg_write<T>(offset: usize, v: T) {
    unsafe {
        let p = get_reg::<T>(offset);
        // println!("before reg_write!");
        *p = v;
        // println!("after reg_write!");
    }
}

#[repr(C)]
struct VRingDesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}
const VRING_DESC_F_NEXT: u16 = 1;
const VRING_DESC_F_WRITE: u16 = 2;

#[repr(C)]
struct VRingUsedElem {
    id: u32,
    len: u32,
}

const VIRTIO_BLK_T_IN: u32 = 0; // read the disk
const VIRTIO_BLK_T_OUT: u32 = 1; // write the disk

#[repr(C)]
struct UsedArea {
    flags: u16,
    id: u16,
    elems: [VRingUsedElem; NUM],
}

#[repr(C)]
pub struct Buf {
    blockno: u64,
    pub data: [u8; 512],
    disk: u8,
    // sleep_lock: condvar::Condvar,
    completed: bool,
}

impl Buf {
    pub fn new(blockno: u64) -> Self {
        Buf {
            blockno,
            data: [0; 512],
            disk: 0,
            completed: false,
        }
    }
}

#[repr(C)]
#[derive(Clone,Copy)]
struct BufInfo {
    buf: usize, // va of struct Buf
    status: u8,
}

#[repr(C)]
#[repr(align(4096))]
struct VirtioDisk {
    pages: [u8; 2 * PAGE_SIZE],
    free: [u8; NUM],
    used_idx: u16,
    init: bool,
    buf_info: [BufInfo; NUM],
}

#[repr(C)]
struct VirtioBlockOuthdr {
    op: u32,
    reserved: u32,
    sector: u64,
}

static DISK: Mutex<VirtioDisk> = Mutex::new(VirtioDisk {
    pages: [0; 2 * PAGE_SIZE],
    free: [0; NUM],
    used_idx: 0,
    init: false,
    buf_info: [BufInfo { buf: 0, status: 0 }; NUM],
});

impl VirtioDisk {
    fn get_desc_array(&mut self) -> &mut [VRingDesc] {
        unsafe { core::slice::from_raw_parts_mut(&mut self.pages[0] as *mut u8 as *mut VRingDesc, NUM) }
    }
    
    fn get_avail_array(&mut self) -> &mut [u16] {
        let desc = self.get_desc_array();
        let avail = &mut desc[0] as *mut VRingDesc as usize + NUM * core::mem::size_of::<VRingDesc>();
        unsafe { core::slice::from_raw_parts_mut(avail as *mut u16, NUM + 2) }
    }

    fn get_used(&mut self) -> &mut UsedArea {
        let slice = unsafe { core::slice::from_raw_parts_mut(&mut self.pages[PAGE_SIZE] as *mut u8 as *mut UsedArea, 1) };
        &mut slice[0]
    }

    fn alloc_desc(&mut self) -> i32 {
        let p = (0..NUM).filter(|i| self.free[*i] == 1).next();
        if let Some(a) = p {
            self.free[a] = 0;
            a as i32
        } else {
            -1
        }
    }

    fn free_desc(&mut self, i: usize) {
        assert!(i < NUM, "free_desc intr 1");
        assert!(self.free[i] == 0, "free_desc intr 2");
        let desc_array = self.get_desc_array();
        desc_array[i].addr = 0;
        self.free[i] = 1;
    }

    fn free_chain(&mut self, i: usize) {
        //let desc_array = self.get_desc_array();
        let desc_array = unsafe { core::slice::from_raw_parts_mut(&mut self.pages[0] as *mut u8 as *mut VRingDesc, NUM) };
        let mut p: usize = i;
        loop {
            // println!("p = {}", p);
            assert!(p < NUM, "free_chain intr 1");
            assert!(self.free[p] == 0, "free_desc intr 2");
            desc_array[p].addr = 0;
            self.free[p] = 1;
            if desc_array[p].flags & VRING_DESC_F_NEXT != 0 {
                p = desc_array[p].next as usize;
            } else {
                break;
            }
        }
        // println!("free_chain exited!");
    }

    fn alloc_3desc(&mut self, idx: &mut[usize; 3]) -> i32 {
        for i in 0..3 {
            let id = self.alloc_desc();
            if id < 0 {
                for j in 0..i {
                    self.free_desc(idx[j]);
                }
                return -1;
            } else {
                idx[i] = id as usize;
            }
        }
        0
    }

    #[no_mangle]
    fn virtio_disk_issue(&mut self, buf: &mut Buf, write: bool, idx0: &mut usize) {
        let sector: u64 = buf.blockno;
        let mut idx: [usize; 3] = [0; 3];
        assert!(self.alloc_3desc(&mut idx) == 0); // assuming desc space is sufficient
        let virtio_hdr = VirtioBlockOuthdr {
            op: if write { VIRTIO_BLK_T_OUT } else { VIRTIO_BLK_T_IN },
            reserved: 0,
            sector,
        };

        self.buf_info[idx[0]].buf = buf as *const _ as usize; 
        self.buf_info[idx[0]].status = 0xf;
        let buf_status_va = &self.buf_info[idx[0]].status as *const _ as usize;

        let desc_array = self.get_desc_array();
        desc_array[idx[0]] = VRingDesc {
            addr: paging::PageTableImpl::kvmpa(&virtio_hdr as *const _ as usize) as u64,
            len: core::mem::size_of::<VRingDesc>() as u32,
            flags: VRING_DESC_F_NEXT,
            next: idx[1] as u16
        };
        desc_array[idx[1]] = VRingDesc {
            addr: paging::PageTableImpl::kvmpa(&buf.data[0] as *const _ as usize) as u64,
            len: 512,
            flags: if write { VRING_DESC_F_NEXT } else { VRING_DESC_F_NEXT | VRING_DESC_F_WRITE },
            next: idx[2] as u16
        };
        desc_array[idx[2]] = VRingDesc {
            addr: paging::PageTableImpl::kvmpa(buf_status_va) as u64,
            len: 1,
            flags: VRING_DESC_F_WRITE,
            next: 0
        };

        let avail_array = self.get_avail_array();
        avail_array[2 + avail_array[1] as usize % NUM] = idx[0] as u16;
        compiler_memory_barrier();
        avail_array[1] += 1;
        compiler_memory_barrier();
        

        buf.disk = 1;

        *idx0 = idx[0];
        reg_write::<u32>(VIRTIO_MMIO_QUEUE_NOTIFY, 0x0);
        // println!("after writing notify");
        // println!("issue successfully!");
    }

    fn virtio_disk_clean(&mut self, idx: usize) {
        self.buf_info[idx].buf = 0;
        self.free_chain(idx);
    }

    fn virtio_disk_intr(&mut self) {
        // println!("into virtio_disk_intr inside!");
        let slice = unsafe { core::slice::from_raw_parts_mut(&mut self.pages[PAGE_SIZE] as *mut u8 as *mut UsedArea, 1) };
        let used = &mut slice[0];
        let num: u16 = NUM as u16;
        // println!("self.used_idx = {}", self.used_idx);
        // println!("used.id = {}", used.id);
        while self.used_idx % num != used.id % num {
            let id: usize = used.elems[self.used_idx as usize].id as usize;
            // println!("used_length = {}", used.elems[self.used_idx as usize].len);
            // println!("status = {}", self.buf_info[id].status);
            assert!(self.buf_info[id].status == 0, "virtio_disk_intr status");
            let buf = unsafe { &mut *(self.buf_info[id].buf as *mut u8 as *mut Buf) };
            buf.disk = 0;
            // println!("before notify!");
            // buf.sleep_lock.notify();
            buf.completed = true;
            // println!("after notify!");
            self.used_idx = (self.used_idx + 1) % num;
        }
        // println!("exit virtio_disk_intr inside!");
    }
}

#[no_mangle]
pub fn virtio_disk_rw(buf: &mut Buf, write: bool) {
    let status = disable_and_store();
    let stimer = riscv::register::sie::read().stimer();
    unsafe { riscv::register::sie::clear_stimer(); }
    // println!("virtio_disk_rw blockno = {}, write = {}", buf.blockno, write);
    
    let mut idx0: usize = 0;
    DISK.lock()
        .virtio_disk_issue(buf, write, &mut idx0);

    // buf.sleep_lock.wait(); 
    
    enable_and_store();
    //reg_write::<u32>(VIRTIO_MMIO_QUEUE_NOTIFY, 0x0);

    // println!("start waiting...");
    loop {
        if buf.completed {
            break;
        }
    }
    // println!("end waiting...");
    restore(status);
    
    DISK.lock()
        .virtio_disk_clean(idx0);
    
    if stimer {
        unsafe { riscv::register::sie::set_stimer(); }
    }
}

pub fn init() {
    
    assert_eq!(reg_read::<u32>(VIRTIO_MMIO_MAGIC_VALUE), VIRTIO_MMIO_MAGIC_NUMBER, "magic is wrong!");
    assert_eq!(reg_read::<u32>(VIRTIO_MMIO_VERSION), 0x1, "not legacy ver of virtio!");
    assert_eq!(reg_read::<u32>(VIRTIO_MMIO_DEVICE_ID), 0x2, "not virtio_blk device!");
    assert_eq!(reg_read::<u32>(VIRTIO_MMIO_VENDOR_ID), VIRTIO_MMIO_VENDOR_NUMBER, "vendor id is wrong!");
    println!("virtio_disk found!");

    let mut status: u32 = 0;

    status |= VIRTIO_CONFIG_S_ACKNOWLEDGE;
    reg_write::<u32>(VIRTIO_MMIO_STATUS, status);
    
    status |= VIRTIO_CONFIG_S_DRIVER;
    reg_write::<u32>(VIRTIO_MMIO_STATUS, status);

    let mut features: u32 = reg_read::<u32>(VIRTIO_MMIO_DEVICE_FEATURES);
    // println!("features = {:#x}", features);
    features &= !(1 << VIRTIO_BLK_F_RO);
    features &= !(1 << VIRTIO_BLK_F_SCSI);
    features &= !(1 << VIRTIO_BLK_F_CONFIG_WCE);
    features &= !(1 << VIRTIO_BLK_F_MQ);
    features &= !(1 << VIRTIO_F_ANY_LAYOUT);
    features &= !(1 << VIRTIO_RING_F_EVENT_IDX);
    features &= !(1 << VIRTIO_RING_F_INDIRECT_DESC);
    reg_write::<u32>(VIRTIO_MMIO_DEVICE_FEATURES, features);

    status |= VIRTIO_CONFIG_S_FEATURES_OK;
    reg_write::<u32>(VIRTIO_MMIO_STATUS, status);

    status |= VIRTIO_CONFIG_S_DRIVER_OK;
    reg_write::<u32>(VIRTIO_MMIO_STATUS, status);

    reg_write::<u32>(VIRTIO_MMIO_GUEST_PAGE_SIZE, PAGE_SIZE as u32);

    let mut disk = DISK.lock();
    reg_write::<u32>(VIRTIO_MMIO_QUEUE_SEL, 0u32);
    let max = reg_read::<u32>(VIRTIO_MMIO_QUEUE_NUM_MAX);
    assert!(max > 0, "virtio disk has no virtqueue 0!");
    assert!(max >= NUM as u32, "virtqueue max size is too small!");
    // assert!(reg_read::<u32>(VIRTIO_MMIO_QUEUE_ALIGN) == 4096, "virtqueue alignment {} != 4096!");
    // println!("virtqueue alignment = {}", reg_read::<u32>(VIRTIO_MMIO_QUEUE_ALIGN));
    reg_write::<u32>(VIRTIO_MMIO_QUEUE_ALIGN, 4096);
    reg_write::<u32>(VIRTIO_MMIO_QUEUE_NUM, NUM as u32);
    reg_write::<u32>(VIRTIO_MMIO_QUEUE_PFN, (get_pa_via_va(&disk.pages[0] as *const u8 as usize) / PAGE_SIZE) as u32);

    for i in 0..NUM {
        disk.free[i] = 1;
    }
    disk.init = true;
    println!("++++ setup disk!      ++++");
}

pub fn virtio_disk_intr() {
    // println!("into virtio_disk_intr!");
    DISK.lock().virtio_disk_intr();
}

pub fn virtio_disk_test() {
    // println!("into virtio_disk_test!");
    let mut buf = Buf {
        blockno: 100,
        data: [0; 512],
        disk: 1,
        // sleep_lock: condvar::Condvar::new(),
        completed: false,
    };
    for i in 0..256 {
        buf.data[i] = i as u8;
    }
    virtio_disk_rw(&mut buf, true);
    let mut chk_buf = Buf {
        blockno: 100,
        data: [0; 512],
        disk: 1,
        completed: false,
    };
    virtio_disk_rw(&mut chk_buf, false);
    for i in 0..256 {
        assert_eq!(chk_buf.data[i], i as u8);
        // println!("data[{}] = {}", i, chk_buf.data[i]);
    }
    println!("passed the disk I/O test!");
    loop {}
}

fn compiler_memory_barrier() {
    //no re-ordering of reads and writes across this point is allowed
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
