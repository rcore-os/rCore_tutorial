use crate::consts::MAX_PHYSICAL_PAGES;
use spin::Mutex;
use super::paging;

pub struct segment_tree_allocator {
    a: [u8; MAX_PHYSICAL_PAGES << 1],
    M: usize,
    n: usize,
    offset: usize
}

pub static SEGMENT_TREE_ALLOCATOR: Mutex<segment_tree_allocator> = Mutex::new(segment_tree_allocator {
    a: [0; MAX_PHYSICAL_PAGES << 1],
    M: 0,
    n: 0,
    offset: 0
});

impl segment_tree_allocator {
    pub fn init(&mut self, l: usize, r: usize) {
        self.offset = l - 1;
        self.n = r - l;
        self.M = 1;
        while self.M < self.n + 2 {
            self.M = self.M << 1;
        }
        for i in (1..(self.M << 1)) { self.a[i] = 1; }
        for i in (1..self.n) { self.a[self.M + i] = 0; }
        for i in (1..self.M).rev() { self.a[i] = self.a[i << 1] & self.a[(i << 1) | 1]; }
    }
    pub fn alloc(&mut self) -> usize {
        // assume that we never run out of physical memory
        let mut p = 1;
        while p < self.M {
            if self.a[p << 1] == 0 { p = p << 1; } else { p = (p << 1) | 1; }
        }
        let result = p + self.offset - self.M;
        self.a[p] = 1;
        p >>= 1;
        while p > 0 {
            self.a[p] = self.a[p << 1] & self.a[(p << 1) | 1];
            p >>= 1;
        }
        result
    }
    pub fn dealloc(&mut self, n: usize) {
        let mut p = n + self.M - self.offset;
        assert!(self.a[p] == 1);
        self.a[p] = 0;
        p >>= 1;
        while p > 0 {
            self.a[p] = self.a[p << 1] & self.a[(p << 1) | 1];
            p >>= 1;
        }
    }
}

