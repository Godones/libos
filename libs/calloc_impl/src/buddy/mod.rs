use core::alloc::{GlobalAlloc, Layout};
use buddy_system_allocator::LockedHeap;
use calloc_trait::CAllocator;

pub struct BuddyAllocator{
    inner:LockedHeap<32>
}





impl CAllocator for BuddyAllocator{
    fn new() -> Self {
        Self{
            inner:LockedHeap::empty()
        }
    }
    fn init(&self, heap_start: usize, heap_size: usize) {
        unsafe{self.inner.lock().init(heap_start,heap_size)}
    }
    fn calloc(&self, layout: Layout) -> *mut u8 {
        unsafe{self.inner.alloc(layout)}
    }
    fn cdealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {self.inner.dealloc(ptr,layout)}
    }
}

impl BuddyAllocator {
    pub const fn empty()->Self{
        Self{
            inner:LockedHeap::empty()
        }
    }
}

unsafe impl GlobalAlloc for BuddyAllocator{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.calloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.cdealloc(ptr, layout)
    }
}