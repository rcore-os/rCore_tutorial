mod device;
pub mod stdio;

use crate::consts::PAGE_SIZE;
use alloc::{sync::Arc, vec::Vec};
use lazy_static::*;
use rcore_fs::vfs::*;
use rcore_fs_sfs::SimpleFileSystem;

lazy_static! {
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        let device = {
            extern "C" {
                fn _user_img_start();
                fn _user_img_end();
            };
            let start = _user_img_start as usize;
            let end = _user_img_end as usize;
            Arc::new(unsafe { device::MemBuf::new(start, end) })
        };
        let sfs = SimpleFileSystem::open(device).expect("failed to open SFS");
        sfs.root_inode()
    };
}

pub trait INodeExt {
    fn read_as_vec(&self) -> Result<Vec<u8>>;
}

impl INodeExt for dyn INode {
    fn read_as_vec(&self) -> Result<Vec<u8>> {
        let size = self.metadata()?.size;
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        self.read_at(0, buf.as_mut_slice())?;
        Ok(buf)
    }
}

pub fn init() {
    println!("available programs in rust/ are:");
    let mut id = 0;
    let mut rust_dir = ROOT_INODE.lookup("rust").unwrap();
    while let Ok(name) = rust_dir.get_entry(id) {
        id += 1;
        println!("  {}", name);
    }
    println!("++++ setup fs!        ++++")
}

use spin::Mutex;
const DISK_PAGES: usize = 512;
const PAGE_DISK_SIZE: usize = PAGE_SIZE * DISK_PAGES;

static BUFFER: Mutex<[u8; PAGE_DISK_SIZE]> = Mutex::new([0u8; PAGE_DISK_SIZE]);
static ALLOCATOR: Mutex<[u8; DISK_PAGES / 8]> = Mutex::new([0u8; DISK_PAGES / 8]);

fn alloc_pos() -> usize {
    let mut idx = 0usize;
    let mut allocator = ALLOCATOR.lock();
    for mut byte in allocator.iter_mut() {
        if *byte | 0 == 0xFF {
            idx += 8;
            continue;
        }
        for i in 0..8 {
            if (0x80 >> i) & *byte == 0 {
                *byte |= (0x80 >> i);
                idx += i;
                return idx;
            }
        }
    }
    unimplemented!()
}

fn dealloc_pos(pos: usize) {
    let idx = pos / 8;
    let offset = pos % 8;
    let mut allocator = ALLOCATOR.lock();
    allocator[idx] &= !(0x80 >> offset);
}

pub fn disk_page_write(page: &[u8]) -> usize {
    // allocate a position in disk for this page
    let pos = alloc_pos() * PAGE_SIZE;
    println!(
        "    frame loc : {:#x}, disk pos is {:#x}",
        page.as_ptr() as usize,
        pos
    );
    let mut buffer = BUFFER.lock();
    buffer[pos..pos + PAGE_SIZE].copy_from_slice(page);
    pos
}

pub fn disk_page_read(pos: usize, page: &mut [u8]) {
    print!("    disk pos : {:#x}, ", pos);
    let buffer = BUFFER.lock();
    page.copy_from_slice(&buffer[pos..pos + PAGE_SIZE]);
    dealloc_pos(pos / PAGE_SIZE);
}
