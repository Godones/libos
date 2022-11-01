use std::process::Command;
use clap::{command, Parser};
use crate::parse_dependence::{Dependence, parse_crate_toml};

#[derive(Parser)]
#[derive(Debug)]
#[command(author, version, about, long_about = None)]
pub struct Configuration{
    /// app name to run
    #[clap[long]]
    app:String,
    /// platform to run on
    #[clap(short,long,default_value="qemu")]
    plat:String,
    /// the architecture to run on
    #[clap(short,long,default_value="riscv")]
    arch:String,
}

/// 由于生成一个包含应用程序以及操作系统模块的crate
pub struct Generator{
    config:Configuration,
}


impl Generator{
    pub fn new(config:Configuration)->Self{
        Self{
            config,
        }
    }
    pub fn build(&self){
        let dependencies = parse_crate_toml(self.config.app.as_str());
        self.modify_toml(&dependencies);
        self.build_kernel_app();
    }

    fn modify_toml(&self,dependencies:&Vec<Dependence>){
        // 修改toml文件
        let output = Command::new("cargo")
            .args(["add","--path"])
            .arg("../apps/".to_string()+&self.config.app)
            .args(["--rename","app"])
            .current_dir("kernel-app")
            .output();
        if output.unwrap().status.success(){
            println!("add app to kernel-app");
        }


        match self.config.plat.as_str() {
            "qemu" =>{
                match self.config.arch.as_str() {
                    "riscv"=>{
                        self.cboot(dependencies,"qemu-riscv-virt");
                        let output = Command::new("cargo")
                            .args(["add","--path"])
                            .arg("../plat/".to_string()+"qemu-riscv-virt")
                            .args(["--rename","plat"])
                            .args(["--build"])
                            .current_dir("kernel-app")
                            .output();
                        if output.unwrap().status.success(){
                            println!("add plat for build.rs");
                        }
                    }
                    _ =>{}
                }
            }
            _ =>{}
        }
    }
    fn cboot(&self,dependencies:&Vec<Dependence>,plat:&str){

        dependencies.iter().for_each(|d|{
            if d.name == "calloc-trait"{
                let feature = "alloc";
                // 询问用户需要使用哪个分配器
                let mut class = String::new();
                println!("Please choose a allocator:[buddy/linked-list]");
                std::io::stdin().read_line(&mut class).unwrap();

                println!("choose {} for alloc",class.as_str().trim());
                // 修改cboot的calloc_impl依赖项

                let _output = Command::new("cargo")
                    .arg("rm")
                    .arg("calloc_impl")
                    .current_dir("./libs/cboot")
                    .output().unwrap();
                let output = Command::new("cargo")
                    .args(["add","--path"])
                    .arg("../calloc_impl")
                    .args(["--features",class.as_str()])
                    .current_dir("./libs/cboot")
                    .output();
                if output.as_ref().unwrap().status.success(){
                    println!("modify cboot to support alloc");
                }else {
                    let error = core::str::from_utf8(&output.as_ref().unwrap().stderr).unwrap();
                    println!("modify cboot to support alloc failed :{}",error);
                }

                let output = Command::new("cargo")
                    .args(["add","--path"])
                    .arg("../libs/".to_string()+"cboot")
                    .args(["--features",plat])
                    .args(["--features",feature])
                    .current_dir("kernel-app")
                    .output();
                if output.as_ref().unwrap().status.success(){
                    println!("add cboot to kernel_app");
                }else {
                    let error = core::str::from_utf8(&output.as_ref().unwrap().stderr).unwrap();
                    println!("add cboot failed :{}",error);
                }
            }
        });
    }

    fn build_kernel_app(&self){
        let output = Command::new("cargo")
            .args(["build","--target","riscv64gc-unknown-none-elf","--release","--package","kernel-app"])
            .output();
        if !output.unwrap().status.success(){
            panic!("build kernel-app fail");
        }else {
            println!("build kernel-app success");
        }
    }
    pub fn run() {
        let output = Command::new("qemu-system-riscv64")
            .args(["-machine","virt","-bios","bootloader/rustsbi-qemu.bin","-nographic"])
            .arg("-kernel")
            .arg("target/riscv64gc-unknown-none-elf/release/kernel-app")
            .spawn()
            .expect("no qemu");
        output.wait_with_output().expect("no qemu");
    }
}