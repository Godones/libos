#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(default_alloc_error_handler)]


#[no_mangle]
pub unsafe fn rust_main()->!{
    //启动准备
    cboot::boot();
    //程序运行
    app::app_main();
    loop {}
}




#[panic_handler]
fn panic(_info: &core::panic::PanicInfo)->!{
    unimplemented!()
}