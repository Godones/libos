#![no_std]
#![feature(linkage)]
#![feature(asm_const)]
#![feature(naked_functions)]

///! 这个lib用于提供操作系统启动早期的一些初始化任务

#[cfg(feature = "qemu-riscv-virt")]
use qemu_riscv_virt::*;
#[cfg(feature = "qemu-loongarch")]
use qemu_riscv_virt::*;

#[cfg(feature = "qemu-riscv-virt")]
boot0!(stack=0x1000);
#[cfg(feature = "qemu-loongarch")]
boot0!(stack=0x1000);


#[cfg(feature = "alloc")]
use calloc_trait::CAllocator;

#[cfg(feature = "alloc")]
static mut HEAP_DATA:[u8;0xa000]= [0u8; 0xa000];

#[cfg(feature = "alloc")]
#[global_allocator]
pub static ALLOCATOR:calloc_impl::Allocator = calloc_impl::Allocator::empty();




pub fn boot(){
    // 初始化标准输入输出
    #[cfg(feature = "qemu-riscv-virt")]
    cconsole::init_stdio(&Console);
    #[cfg(feature = "qemu-loongarch")]
    cconsole::init_stdio(&Console);
    #[cfg(feature = "alloc")]
    unsafe{
        ALLOCATOR.init(HEAP_DATA.as_ptr() as usize,HEAP_DATA.len());
    }
}

#[no_mangle]
#[linkage="weak"]
fn rust_main(){}

