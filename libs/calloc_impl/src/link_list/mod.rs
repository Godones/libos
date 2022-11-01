use core::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;
use calloc_trait::CAllocator;
pub struct LinkAllocator{
    inner:LockedHeap
}


unsafe impl GlobalAlloc for LinkAllocator{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.calloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.cdealloc(ptr, layout)
    }
}

impl CAllocator for LinkAllocator{
    fn new() -> Self {
        Self{
            inner:LockedHeap::empty()
        }
    }
    fn init(&self, heap_start: usize, heap_size: usize) {
        unsafe {
            self.inner.lock().init(heap_start,heap_size);
        }
    }
    fn calloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            self.inner.alloc(layout)
        }
    }
    fn cdealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.inner.dealloc(ptr,layout)
        }
    }
}

impl LinkAllocator{
    pub const fn empty()->Self{
        Self{
            inner:LockedHeap::empty()
        }
    }
}