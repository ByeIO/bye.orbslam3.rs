#![allow(unused)]

//! 使用wonnx（纯Rust）加载MNIST的ONNX模型。

// 标准库
use std::collections::HashMap;
use std::convert::TryInto;
use std::path::Path;
use std::time::Instant;

// 图像处理
use image::{imageops::FilterType, ImageBuffer, Pixel, Rgb};

// ONNX运行时
use wonnx::utils::OutputTensor;

// 线性代数
use nalgebra::{DMatrix, DVector};

// 参数管理
async fn run() {
    // 执行模型推理并获取结果
    let probabilities = execute_gpu().await.unwrap();
    // 获取模型输出的第一个张量
    let (_, probabilities) = probabilities.into_iter().next().unwrap();
    // 将模型输出的张量转换为Vec<f32>
    let probabilities: Vec<f32> = probabilities.try_into().unwrap();
    // 打印模型输出的概率值
    println!("steps: {:#?}", probabilities);
    // 打印概率值的长度
    println!("steps: {:#?}", probabilities.len());

    // 将概率值与索引绑定并转换为向量
    let mut probabilities = probabilities.iter().enumerate().collect::<Vec<_>>();

    // 根据概率值对向量进行降序排序
    probabilities.sort_unstable_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    // 打印推理结果（概率最高的索引）
    println!("Infered result: {}", probabilities[0].0);
}

// 硬件管理
async fn execute_gpu() -> Option<HashMap<String, OutputTensor>> {
    // 创建输入数据的HashMap
    let mut input_data = HashMap::new();

    // 加载图像数据
    let image = load_image();
    // 将图像数据插入到输入数据的HashMap中
    input_data.insert("Input3".to_string(), image.as_slice().into());

    // 构造模型路径
    // let model_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    //     .join("../../../assets/wonnx_data/models")
    //     .join("opt-mnist.onnx");
    let model_path = Path::new("../../assets/wonnx_data/models").join("opt-mnist.onnx");
    // 加载ONNX模型
    let session = wonnx::Session::from_path(model_path).await.unwrap();

    // 记录推理前的时间
    let time_pre_compute = Instant::now();
    // 执行模型推理
    let result = session.run(&input_data).await.unwrap();
    // 记录推理后的时间
    let time_post_compute = Instant::now();
    // 打印推理时间
    println!(
        "time: post_compute: {:#?}",
        time_post_compute - time_pre_compute
    );
    // 返回模型推理结果
    Some(result)
}

// 使用tokio的main宏
#[tokio::main] 
async fn main() {
    // 初始化日志
    // env_logger::init();
    // 记录主函数开始的时间
    let time_pre_compute = Instant::now();
    // 执行异步任务
    run().await;
    // 打印主函数的执行时间
    println!("time: main: {:#?}", time_pre_compute.elapsed());
}

// 加载图像数据
pub fn load_image() -> DMatrix<f32> {
    // 打开图像文件并将其转换为RGB格式，同时调整大小为28x28
    let image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = image::open(
        Path::new("../../assets/wonnx_data/images")
            .join("7.jpg"),
    )
    .unwrap()
    .resize_exact(28, 28, FilterType::Nearest)
    .to_rgb8();

    // 构造一个形状为(1, 1, 28, 28)的矩阵
    let mut matrix = DMatrix::zeros(1, 784); // 28*28 = 784

    for j in 0..28 {
        for i in 0..28 {
            let pixel = image_buffer.get_pixel(i as u32, j as u32);
            let channels = pixel.channels();
            // 将像素值从[0, 255]范围归一化到[0, 1]范围
            let value = (channels[0] as f32) / 255.0; // 只取灰度值
            matrix[(0, j * 28 + i)] = value;
        }
    }

    matrix
}
