#![allow(unused)]

//! 转换proto3文件到rust代码

// protobuf处理
use prost;
use prost_build::Config;

fn main() {
    // FIXME: 使用命令行工具转换

    // let mut config = Config::new();
    
    // // 将所有生成的类型合并到onnx.rs
    // config.include_file("onnx");
    
    // // 设置proto文件查找路径
    // config.include_file("assets/onnx_opset22");
    
    // // 指定输出目录（根据Cargo.toml位置调整）
    // config.out_dir("result/");
    
    // // 编译所有proto文件到单个onnx.rs
    // config
    //     .compile_protos(
    //         &[
    //             "./assets/onnx_opset22/onnx.in.proto",
    //         ],
    //         &["./assets/onnx_opset22"], // proto文件根目录
    //     )
    //     .unwrap();
}