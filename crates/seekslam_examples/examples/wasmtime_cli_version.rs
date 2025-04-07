#![allow(unused)]

//! 测试wasmtime_cli, 打印版本号

// 错误处理
use anyhow::Result;

// wasmtime-cli的接口(魔改wasmtime_cli库)
use wasmtime_cli::cli::WasmtimeCli;

fn main()->anyhow::Result<(), anyhow::Error>{
    WasmtimeCli::run("-V")?;
    anyhow::Ok(())
}
