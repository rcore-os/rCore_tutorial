mod frame_allocator;
mod paging;

use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;

pub fn init(l: usize, r: usize) {
    FRAME_ALLOCATOR.lock().init(l, r);
    println!("++++ setup memory!    ++++");
}

pub fn alloc() -> usize { FRAME_ALLOCATOR.lock().alloc() }
pub fn dealloc(n: usize) { FRAME_ALLOCATOR.lock().dealloc(n); }
