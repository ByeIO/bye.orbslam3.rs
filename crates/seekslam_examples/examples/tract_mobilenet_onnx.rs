#![allow(unused)]

//! 使用tract加载onnx执行推理

// onnx运行时(opset18)
use tract_onnx::prelude::*;

fn main() -> TractResult<()> {
    // 加载ONNX模型
    let model = tract_onnx::onnx()
        // 从路径加载模型
        .model_for_path("./assets/mobilenetv2-7.onnx")?
        // 对模型进行优化
        .into_optimized()?
        // 将模型转换为可运行状态，并固定其输入和输出
        .into_runnable()?;

    // 打开图像，调整大小并将其转换为张量
    let image = image::open("./assets/rgb1.png").unwrap().to_rgb8();
    // 将图像调整为224x224大小
    let resized =
        image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
    // 创建一个4维张量，形状为(1, 3, 224, 224)，并进行归一化处理
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 3, 224, 224), |(_, c, y, x)| {
        // 定义均值和标准差
        let mean = [0.485, 0.456, 0.406][c];
        let std = [0.229, 0.224, 0.225][c];
        // 将像素值归一化到[0,1]，然后减去均值并除以标准差
        (resized[(x as _, y as _)][c] as f32 / 255.0 - mean) / std
    })
    .into();

    // 在输入上运行模型
    let result = model.run(tvec!(image.into()))?;

    // 找到最大值及其索引
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(2..) // 从索引2开始，因为通常分类索引从2开始
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    // 打印结果
    println!("result: {:?}", best);
    Ok(())
}
