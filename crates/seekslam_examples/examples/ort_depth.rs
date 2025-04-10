#![allow(unused)]

//! 使用ort预测深度
//! 模型文件model_path = "../../../assets/ailia-models/depth_anything/depth_anything_v2_vits.onnx"
//! 待预测图片img_path = "../assets/rgb1.png"
//! 输出路径: ./result/ort_depth.png

// 导入标准库路径处理模块
use std::path::Path;

// 导入ORT相关模块
use ort::{inputs, session::Session};
use ort::value::Tensor;

// 图像处理相关模块
use image::{ImageBuffer, Luma};

// 错误处理模块
use anyhow::Result;

// 线性代数库
use ndarray::Array;

fn main() -> Result<()> {
    // 定义模型路径、输入图片路径和输出路径
    let model_path = "../../assets/ailia-models/depth_anything/depth_anything_v2_vits.onnx";
    let img_path = "./assets/rgb1.png";
    let output_path = "./result/ort_depth.png";

    // 创建输出目录（如果不存在）
    std::fs::create_dir_all(Path::new(output_path).parent().unwrap())?;

    // 创建ORT会话并加载模型
    let mut session = Session::builder()?.commit_from_file(model_path)?;

    // 加载输入图片并转换为RGB格式
    let img = image::open(img_path)?;
    
    // 将图片缩放到模型要求的518x518分辨率
    let resized_img = img.resize_exact(518, 518, image::imageops::FilterType::Nearest);

    // 创建四维数组保存预处理数据 [batch=1, channels=3, height=518, width=518]
    let mut input_array = Array::zeros((1, 3, 518, 518));

    // 遍历所有像素进行归一化处理
    for (x, y, pixel) in resized_img.to_rgb8().enumerate_pixels() {
        // 将RGB通道值归一化到0-1范围
        input_array[[0, 0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
        input_array[[0, 1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
        input_array[[0, 2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
    }

    // 将数组转换为ORT张量
    let input_tensor = ort::value::Value::from_array(input_array.into_dyn())?;

    // 运行模型推理
    let outputs = session.run(inputs![input_tensor])?;

    // 处理输出张量
    let output_tensor = outputs["select_36"]
        .try_extract_tensor::<f32>()?;
        
    let (shape , data) = output_tensor;
    let array = Array::from_shape_vec((518, 518), data.to_vec())?;
    
    // 计算深度值范围用于归一化
    let min_depth = array.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max_depth = array.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    

    // 创建灰度图像缓冲区
    let mut img_buffer: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(518, 518);

    // 将深度值归一化到0-255范围并保存到灰度图像缓冲区
    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let depth = array[[y as usize, x as usize]];
        let normalized_depth = ((depth - min_depth) / (max_depth - min_depth) * 255.0) as u8;
        *pixel = Luma([normalized_depth]);
    }

    // 保存深度图
    img_buffer.save(output_path)?;

    Ok(())
}