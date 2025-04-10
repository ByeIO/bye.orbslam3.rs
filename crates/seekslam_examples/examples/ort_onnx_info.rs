#![allow(unused)]
#![allow(unexpected_cfgs)]

//! 使用ort（C绑定）运行onnx推理mnist

// 导入标准库中的env和process模块
use std::{env, process}; 

use ort; // 导入ort库
// 导入ort的Session模块
use ort::session::Session; 

// 错误处理
use anyhow::Result;

fn main() -> Result<()> {
    // 根据功能标志注册后端——这并不是使用的关键，可以移除。
    // init()?;

    // 获取命令行参数中的模型路径，如果没有提供则使用默认路径
    let path = "./assets/mobilenetv2-7.onnx" ;
   

    // 从文件中加载模型并创建会话
    let session = Session::builder()?.commit_from_file(path)?;

    // 获取模型的元数据
    let meta = session.metadata()?;
    // 打印模型的名称
    if let Ok(x) = meta.name() {
        println!("Name: {x}");
    }
    // 打印模型的描述
    if let Ok(x) = meta.description() {
        println!("Description: {x}");
    }
    // 打印模型的生产者
    if let Ok(x) = meta.producer() {
        println!("Produced by {x}");
    }

    // 打印模型的输入信息
    println!("Inputs:");
    for (i, input) in session.inputs.iter().enumerate() {
        println!("    {i} {}: {}", input.name, input.input_type);
    }
    // 打印模型的输出信息
    println!("Outputs:");
    for (i, output) in session.outputs.iter().enumerate() {
        println!("    {i} {}: {}", output.name, output.output_type);
    }

    Ok(())
}

// ort的后端配置
use ort::execution_providers::*; 
// 初始化ort的后端配置
pub fn init() -> Result<()> {
    // 根据功能标志设置不同的后端
    #[cfg(feature = "backend-candle")]
    ort::set_api(ort_candle::api());
    #[cfg(feature = "backend-tract")]
    ort::set_api(ort_tract::api());

    // 如果没有指定特定的后端，则使用默认的初始化方式
    #[cfg(all(not(feature = "backend-candle"), not(feature = "backend-tract")))]
    ort::init()
        .with_execution_providers([
            // 根据不同的功能标志启用不同的执行提供者
            #[cfg(feature = "tensorrt")]
            TensorRTExecutionProvider::default().build(),
            #[cfg(feature = "cuda")]
            CUDAExecutionProvider::default().build(),
            #[cfg(feature = "onednn")]
            OneDNNExecutionProvider::default().build(),
            #[cfg(feature = "acl")]
            ACLExecutionProvider::default().build(),
            #[cfg(feature = "openvino")]
            OpenVINOExecutionProvider::default().build(),
            #[cfg(feature = "coreml")]
            CoreMLExecutionProvider::default().build(),
            #[cfg(feature = "rocm")]
            ROCmExecutionProvider::default().build(),
            #[cfg(feature = "cann")]
            CANNExecutionProvider::default().build(),
            #[cfg(feature = "directml")]
            DirectMLExecutionProvider::default().build(),
            #[cfg(feature = "tvm")]
            TVMExecutionProvider::default().build(),
            #[cfg(feature = "nnapi")]
            NNAPIExecutionProvider::default().build(),
            #[cfg(feature = "qnn")]
            QNNExecutionProvider::default().build(),
            #[cfg(feature = "xnnpack")]
            XNNPACKExecutionProvider::default().build(),
            #[cfg(feature = "armnn")]
            ArmNNExecutionProvider::default().build(),
            #[cfg(feature = "migraphx")]
            MIGraphXExecutionProvider::default().build(),
            #[cfg(feature = "vitis")]
            VitisAIExecutionProvider::default().build(),
            #[cfg(feature = "rknpu")]
            RKNPUExecutionProvider::default().build(),
            #[cfg(feature = "webgpu")]
            WebGPUExecutionProvider::default().build()
        ])
		// 提交后端配置
        .commit()?; 

    Ok(())
}