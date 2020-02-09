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
static ENTRYS: Mutex<[usize; DISK_PAGES]> = Mutex::new([0usize; DISK_PAGES]);

pub fn disk_page_write(page: &[u8], entry_loc: usize) -> usize {
    let pos = (page.as_ptr() as usize) & (PAGE_DISK_SIZE - 1);
    println!("page loc : {:#x} pos is {:#x}", page.as_ptr() as usize, pos);
    let mut buffer = BUFFER.lock();
    buffer[pos..pos + PAGE_SIZE].copy_from_slice(page);
    ENTRYS.lock()[pos / PAGE_SIZE] = entry_loc;
    pos
}

pub fn disk_page_read(pos: usize, page: &mut [u8]) -> usize {
    let buffer = BUFFER.lock();
    page.copy_from_slice(&buffer[pos..pos + PAGE_SIZE]);
    ENTRYS.lock()[pos / PAGE_SIZE].clone()
}
