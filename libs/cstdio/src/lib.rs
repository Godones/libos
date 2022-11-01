#![no_std]

use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use cconsole::{put_str};


pub struct Stdio;
impl Write for Stdio{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        put_str(s);
        Ok(())
    }
}

lazy_static!{
    pub static ref STDOUT: Mutex<Stdio> = Mutex::new(Stdio);
}

pub fn __print(args: core::fmt::Arguments) {
    STDOUT.lock().write_fmt(args).unwrap();
}


/// 格式化打印。
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::__print(core::format_args!($($arg)*));
    }
}

/// 格式化打印并换行。
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {{
        $crate::__print(core::format_args!($($arg)*));
        $crate::println!();
    }}
}
