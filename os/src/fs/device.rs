use rcore_fs::dev::*;
use spin::RwLock;
use crate::drivers::virtio_disk;

pub struct MemBuf(RwLock<&'static mut [u8]>);

impl MemBuf {
    pub unsafe fn new(begin: usize, end: usize) -> Self {
        use core::slice;
        MemBuf(RwLock::new(slice::from_raw_parts_mut(
            begin as *mut u8,
            end - begin,
        )))
    }
}

impl Device for MemBuf {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        let slice = self.0.read();
        let len = buf.len().min(slice.len() - offset);
        buf[..len].copy_from_slice(&slice[offset..offset + len]);
        Ok(len)
    }
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        let mut slice = self.0.write();
        let len = buf.len().min(slice.len() - offset);
        slice[offset..offset + len].copy_from_slice(&buf[..len]);
        Ok(len)
    }
    fn sync(&self) -> Result<()> {
        Ok(())
    }
}

pub struct Disk;
impl Disk {
    pub fn new() -> Self {
        Disk {}
    }
}

impl Device for Disk {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        let b_sector = offset / 512;
        let e_sector = (offset + buf.len() + 511) / 512;
        let mut l = offset;
        for sector in b_sector..e_sector {
            let mut disk_buf = virtio_disk::Buf::new(sector as u64);
            virtio_disk::virtio_disk_rw(&mut disk_buf, false);
            let mut r = (l / 512 + 1) * 512;
            r = core::cmp::min(r, offset + buf.len());
            buf[l - offset..r - offset]
                .copy_from_slice(&disk_buf.data[l & 511..{ if r % 512 == 0 { 512 } else {r % 512} }]);
            l = r;
        }
        Ok(buf.len())
    }
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        let b_sector = offset / 512;
        let e_sector = (offset + buf.len() + 511) / 512;
        let mut l = offset;
        for sector in b_sector..e_sector {
            let mut disk_buf = virtio_disk::Buf::new(sector as u64);
            let mut r = (l / 512 + 1) * 512;
            r = core::cmp::min(r, offset + buf.len());
            if l & 511 != 0 || r & 511 != 0 {
                // write partially, read from disk first
                virtio_disk::virtio_disk_rw(&mut disk_buf, false);
            }
            disk_buf.data[l & 511..{ if r % 512 == 0 {512} else {r % 512} }]
                .copy_from_slice(&buf[l - offset..r - offset]);
            disk_buf.clear_completed();
            virtio_disk::virtio_disk_rw(&mut disk_buf, true);
            l = r;
        }
        Ok(buf.len())
    }
    fn sync(&self) -> Result<()> {
        Ok(())
    }
}
