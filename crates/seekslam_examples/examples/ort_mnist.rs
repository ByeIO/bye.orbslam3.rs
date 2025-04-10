#![allow(unused)]
#![allow(unexpected_cfgs)]

//! 使用ort（C绑定）运行onnx推理mnist

// 导入标准库中的env和process模块
use std::{env, process};

// 导入ort的Session模块
use ort::session::Session;
use ort::inputs;
use ort::value::TensorValueType;
use ort::value::Value;

// 图像处理
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

// 线性代数
use nalgebra;

// 错误处理
use anyhow::Result;

fn main() -> Result<()> {
    // mnist模型 onnx opset 12版本
    let model_path = "./assets/mnist-12.onnx";

    // 从文件中加载模型并创建会话
    let mut session = Session::builder()?.commit_from_file(model_path)?;

    // 加载图片
    let img_path = "./assets/3.jpg";
    let img = image::open(img_path)?;

    // 将图片转换为灰度图
    let gray_img = img.grayscale();

    // 将图片缩放到28x28像素
    let resized_img = gray_img.resize_exact(28, 28, image::imageops::FilterType::Nearest);
    
    // 将像素数据转换为四维数组 [batch=1, channels=1, height=28, width=28]
    let mut input_array = ndarray::Array::zeros((1, 1, 28, 28));

    // 遍历所有像素，注意MNIST是灰度图只需填充通道0
    for (x, y, pixel) in resized_img.pixels() {
        // 归一化到0-1并转换为f32
        input_array[[0, 0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
    }

    // 创建符合形状要求的张量
    let input_tensor = ort::value::Value::from_array(input_array.into_dyn())?;

    // 运行模型推理
    let outputs = session.run(inputs![input_tensor])?;

    // 打印输出张量名称
    // println!("输出张量名称: {:?}", outputs.iter().map(|o| o.name.clone()).collect::<Vec<_>>());

    // 获取模型输出
    let output_tensor = outputs.get("Plus214_Output_0").unwrap();
    // let output_data = output_tensor.try_extract_array::<f32>()?.to_vec();
    let output_data = output_tensor.try_extract_array::<f32>()?;

    // 找到最大概率的类别
    let predicted_class = output_data.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;

    // 输出预测结果
    println!("预测的数字是: {}", predicted_class);

    Ok(())
}