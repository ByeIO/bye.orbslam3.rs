#![allow(unused)]

//! 使用tract加载onnx预测深度图

// onnx运行时(opset18)
use tract_onnx::prelude::*;

// 线性代数
use tract_ndarray::{Array, Array4};

// 图片处理
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgb};

fn main() -> TractResult<()> {
    // 加载图片并调整大小
    let img_path = "./assets/rgb1.png";
    let model_path = "../../assets/ailia-models/depth_anything/depth_anything_v2_vits_opset15.onnx";

    // 加载图片并转换为 RGB 格式
    let image = image::open(img_path)?.to_rgb8();
    let (orig_w, orig_h) = image.dimensions();

    // 调整图片大小到模型输入尺寸
    let resized = image::imageops::resize(&image, 384, 384, ::image::imageops::FilterType::Triangle);

    // 将图片转换为张量
    let image_tensor: Array4<f32> = Array4::from_shape_fn((1, 3, 384, 384), |(_, c, y, x)| {
        resized[(x as _, y as _)][c as usize] as f32 / 255.0
    });

    // 将张量转换为 tract 的 Tensor 类型
    let image_tensor = Tensor::from(image_tensor);

    // FIXME : 加载 ONNX 模型
    let _model = tract_onnx::onnx()
        .model_for_path(model_path)?
        .into_optimized()?;
    
    println!("模型优化成功");
    
    let model = _model.into_runnable()?;
    
    println!("模型预热成功");

    // 运行模型
    let result = model.run(tvec!(image_tensor.into()))?;

    // 获取深度图
    let depth = result[0].to_array_view::<u8>()?;
    let depth = depth.to_owned();

    // 将深度图调整回原始尺寸
    let depth_resized = image::imageops::resize(
        &DynamicImage::ImageLuma8(
            ImageBuffer::from_raw(384, 384, depth.clone().into_raw_vec()).unwrap(),
        ),
        orig_w as u32,
        orig_h as u32,
        ::image::imageops::FilterType::Triangle,
    );

    // 对深度图进行归一化并转换为彩色图
    let depth_array: Vec<u8> = depth_resized
        .pixels()
        .map(|p| {
            let value = (p[0] as u8 - depth.clone().into_iter().min().unwrap()) / (depth.clone().into_iter().max().unwrap() - depth.clone().into_iter().min().unwrap()) * 255;
            value
        })
        .collect();
    let depth_color = DynamicImage::ImageLuma8(
        ImageBuffer::from_raw(orig_w, orig_h, depth_array).unwrap(),
    );

    // 保存结果图片
    depth_color.save("./result/tract_depth_onnx.png")?;

    Ok(())
}
