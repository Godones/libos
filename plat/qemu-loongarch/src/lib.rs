#![no_std]
use core::fmt::Write;
use spin::Once;
use uart_16550::MmioSerialPort;
use cconsole::Stdio;

const UART_ADDR:usize = 0x1000_0000;


static mut UART:Once<MmioSerialPort> = Once::new();


pub struct Console;

impl Stdio for Console{
    fn put_char(&self, c: u8) {
        unsafe{
            UART.call_once(|| MmioSerialPort::new(UART_ADDR));
            UART.get_mut().unwrap().write_char(c as char).expect("TODO: panic message");
        }
    }
    fn get_char(&self) -> u8 {
        unsafe {
            UART.call_once(|| MmioSerialPort::new(UART_ADDR));
            UART.get_mut().unwrap().receive()
        }
    }
}

/// 定义内核入口。
///
/// 将设置一个启动栈，并在启动栈上调用高级语言入口。
#[macro_export]
macro_rules! boot0 {
    (stack = $stack:expr) => {
        #[naked]
        #[no_mangle]
        #[link_section = ".text.entry"]
        unsafe extern "C" fn _start() -> ! {
            const STACK_SIZE: usize = $stack;

            #[link_section = ".bss.uninit"]
            static mut STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];

            core::arch::asm!(
                "la.global $t0, {stack}",
                "addi.d $t0, $t0,{stack_size}",
                "move $sp, $t0",
                "bl main",
                stack_size = const STACK_SIZE,
                stack      =   sym STACK,
                main       =   sym rust_main,
                options(noreturn),
            )
        }
    };
}

pub fn panic(_info: &core::panic::PanicInfo)->!{
    loop {

    }
}

