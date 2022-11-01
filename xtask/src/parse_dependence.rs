//! 解析crate的依赖项


use std::process::{Command};

#[derive(Debug)]
pub struct Dependence{
    pub name:String,
    pub version:String
}


impl Dependence {
    pub fn new(name:&str,version:&str)->Self{
        Self{
            name:name.to_string(),
            version:version.to_string()
        }
    }
}

pub fn parse_crate_toml(name:&str)->Vec<Dependence>{
    // 获取crate的依赖项
    let out = Command::new("cargo")
        .arg("tree")
        .args(["--depth","1","-p"])
        .arg(name)
        .output().unwrap();
    let mut ans = Vec::new();
    if out.status.success(){
        let str = core::str::from_utf8(&out.stdout).unwrap();
        let mut lines = str.lines();
        lines.next();
        for line in lines{
            let sp = line.split(" ").collect::<Vec<&str>>();
            let dependence = Dependence::new(sp[1],sp[2]);
            ans.push(dependence);
        }
    }
    ans
}
