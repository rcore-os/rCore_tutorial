use crate::consts::PAGE_BYTES;

pub struct Page([u8; PAGE_BYTES]);

impl Page {
    pub fn clear(&mut self) {
        for i in (0..PAGE_BYTES) {
            self.0[i] = 0;
        }
    }
}
