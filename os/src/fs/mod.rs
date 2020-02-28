mod device;
pub mod stdio;

use alloc::{sync::Arc, vec::Vec};
use lazy_static::*;
use rcore_fs::vfs::*;
use rcore_fs_sfs::SimpleFileSystem;
use rcore_fs::dev::Device;

pub fn disk_test() {
    let disk = device::Disk::new();
    let mut temp = [0u8; 512 * 2];
    temp[0] = 1;
    for i in 1..1024 {
        temp[i] = ((temp[i - 1] as usize * 3) & 255) as u8;
    }
    for i in 0..10 {
        println!("temp[{}] = {}", i, temp[i]);
    }
    disk.write_at(0, &temp);
    let mut partial_write = [0u8; 512];
    partial_write[0] = 1;
    for i in 1..512 {
        partial_write[i] = ((partial_write[i - 1] as usize * 7) & 255) as u8;
    }
    temp[256..256 + 512]
        .copy_from_slice(&partial_write[0..512]);
    disk.write_at(256, &partial_write);
    let mut check = [0u8; 512 * 2];
    disk.read_at(0, &mut check);
    for i in 0..1024 {
        if check[i] != temp[i] {
            println!("check[{}] = {}, temp[{}] = {}", i, check[i], i, temp[i]);
            panic!("disk test failed!");
        }
    }
    // assert!((0..1024).filter(|i| check[*i] != temp[*i]).next() == None, "disk test failed!");
    println!("disk test passed!");
}

lazy_static! {
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        /*
        let device = {
            extern "C" {
                fn _user_img_start();
                fn _user_img_end();
            };
            let start = _user_img_start as usize;
            let end = _user_img_end as usize;
            Arc::new(unsafe { device::MemBuf::new(start, end) })
        };
        */
        let device = Arc::new(device::Disk::new());
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
        // println!("size = {}", size);
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        // println!("begin read_at...");
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
