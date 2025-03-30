#![allow(unused)]

//! 编译预处理

use chrono::prelude::*;

fn main(){
    // 添加文件变更检测避免重复构建
    // println!("cargo:rerun-if-changed=build.rs");
    
    // 获取当前UTC时间并格式化为年月日 时分秒
    let utc: DateTime<Utc> = Utc::now();
    let build_timestamp = utc.format("%Y-%m-%d %H:%M:%S").to_string();
    // 通过Cargo环境变量传递到主程序
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", build_timestamp);
    
    // 获取git哈希值
    let git_output = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(git_output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
