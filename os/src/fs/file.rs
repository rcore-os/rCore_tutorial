use crate::fs::ROOT_INODE;
use alloc::sync::Arc;
use rcore_fs::vfs::INode;

#[derive(Copy, Clone, Debug)]
pub enum FileDescriptorType {
    FdNone,
    FdInode,
    #[allow(dead_code)]
    FdDevice,
}

#[derive(Clone)]
pub struct File {
    fdtype: FileDescriptorType,
    readable: bool,
    writable: bool,
    pub inode: Option<Arc<dyn INode>>,
    offset: usize,
}

impl File {
    pub fn default() -> Self {
        File {
            fdtype: FileDescriptorType::FdNone,
            readable: false,
            writable: false,
            inode: None,
            offset: 0,
        }
    }
    pub fn set_readable(&mut self, v: bool) {
        self.readable = v;
    }
    pub fn set_writable(&mut self, v: bool) {
        self.writable = v;
    }
    pub fn get_readable(&self) -> bool {
        self.readable
    }
    pub fn get_writable(&self) -> bool {
        self.writable
    }
    pub fn set_fdtype(&mut self, t: FileDescriptorType) {
        self.fdtype = t;
    }
    pub fn get_fdtype(&self) -> FileDescriptorType {
        self.fdtype
    }
    pub fn set_offset(&mut self, o: usize) {
        self.offset = o;
    }
    pub fn get_offset(&self) -> usize {
        self.offset
    }

    #[allow(unused_unsafe)]
    pub fn open_file(&mut self, path: &'static str, flags: i32) {
        self.set_fdtype(FileDescriptorType::FdInode);
        self.set_readable(true);
        if (flags & 1) > 0 {
            self.set_readable(false);
        }
        if (flags & 3) > 0 {
            self.set_writable(true);
        }
        unsafe {
            self.inode = Some(ROOT_INODE.lookup(path).unwrap().clone());
        }
        self.set_offset(0);
    }
}
