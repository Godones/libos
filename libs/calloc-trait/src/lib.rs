#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;
pub use alloc::*;
use core::alloc::Layout;



/// 定义内存分配器的接口
pub trait CAllocator{
    fn new()->Self;
    fn init(&self,heap_start:usize,heap_size:usize);
    fn calloc(&self, layout:Layout) ->*mut u8;
    fn cdealloc(&self, ptr:*mut u8, layout:Layout);
}