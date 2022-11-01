#![no_std]
use spin::Once;

pub trait Stdio{
    fn put_char(&self, c: u8);
    fn get_char(&self) -> u8;
    fn put_str(&self,str:&str){
        str.as_bytes().iter().for_each(|&u|{
            self.put_char(u)
        })
    }
}

static mut STDIO:Once<&'static dyn Stdio> = Once::new();

pub fn init_stdio(stdio:&'static dyn Stdio){
    unsafe{
        STDIO.call_once(|| stdio);
    }
}


pub fn put_str(c:&str){
    unsafe{
        STDIO.call_once(|| panic!("stdio not initialized"));
        STDIO.get().unwrap().put_str(c);
    }
}

pub fn get_char()->u8{
    unsafe{
        STDIO.call_once(|| panic!("stdio not initialized"));
        STDIO.get().unwrap().get_char()
    }
}
