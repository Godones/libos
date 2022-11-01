#![no_std]

use cstdio::{print, println};
use calloc_trait::vec::Vec;

#[no_mangle]
pub fn app_main(){
    println!("Hello, world!");
    let mut t = Vec::new();
    for i in 0..100{
        t.push(i);
    }
    t.iter().for_each(|v|{
        print!("{} ",v);
        if v %10 ==0{
            println!()
        }
    });
    println!();
    println!("Vec len: {}",t.len());
}