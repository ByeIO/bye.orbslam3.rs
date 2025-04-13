#![allow(unused)]

//! 使用optics模型预测光流
//! 模型: ../../../assets/neuflow_v2/neoflow_things.onnx
//! 输入:input1
//! name: input1
//! tensor: float32[1,3,432,768]
//! input2
//! name: input2
//! tensor: float32[1,3,432,768]
//! 输出:output
//! name: output
//! tensor: float32[1,2,432,768]

// 导入标准库路径处理模块
use std::path::Path;

// 用于计时
use std::time::Instant;
// 标准库
use std::f32::consts::PI;
use std::ops::Add;

// 导入 ORT 相关模块
use ort::{inputs, session::Session};
use ort::value::Tensor;

// 图像处理相关模块
use image::{GenericImageView, ImageBuffer, Rgb, RgbImage, DynamicImage};

// 错误处理模块
use anyhow::{anyhow, Result};

// 线性代数库
use ndarray::{
    Array, ArrayD, ArrayViewD, 
    Dim, Ix1, Ix2, Ix3, Ix4, 
    IxDyn, stack, Axis, IxDynImpl, 
    ArrayBase, OwnedRepr, IntoDimension,
    ArrayView, 
};
use ndarray::s;
// use ndarray_linalg::error::LinalgError;

// 随机数
use rand::rng;

// 随机分布
use rand_distr::{Normal, Distribution};

// 可克隆的迭代器
use itertools::Itertools;

// 克隆插件
use clone_dyn_types::CloneDyn;

// 可用的模型列表
const AVAILABLE_MODELS: [&str; 3] = ["neuflow_mixed", "neuflow_sintel", "neuflow_things"];

// 检查模型文件是否存在，若不存在则处理（这里原Python代码没有实际下载逻辑，直接退出）
fn check_model(model_path: &str) -> Result<()> {
    if Path::new(model_path).exists() {
        println!("模型文件存在!");
        Ok(())
    } else {
        // 从路径中提取模型名称
        let model_name = Path::new(model_path).file_stem().ok_or_else(|| anyhow!("无法获取模型文件名"))?.to_str().ok_or_else(|| anyhow!("无法转换模型文件名"))?;
        if !AVAILABLE_MODELS.contains(&model_name) {
            return Err(anyhow!("无效的模型名称: {}", model_name));
        }
        // 原Python代码中下载部分被注释，这里直接退出
        std::process::exit(0);
    }
}

// 生成颜色轮
fn make_color_wheel() -> ArrayD<u8> {
    let ry = 15;
    let yg = 6;
    let gc = 4;
    let cb = 11;
    let bm = 13;
    let mr = 6;
    let ncols = ry + yg + gc + cb + bm + mr;
    let mut colorwheel = Array::zeros((ncols, 3));

    let mut col = 0;
    // RY段
    colorwheel.slice_mut(s![col..col + ry, 0]).fill(255);
    for (i, val) in (0..ry).enumerate() {
        colorwheel[[col + i, 1]] = ((255.0 * val as f32 / ry as f32) as u8);
    }
    col += ry;
    // YG段
    for (i, val) in (0..yg).enumerate() {
        colorwheel[[col + i, 0]] = (255 - (255.0 * val as f32 / yg as f32) as u8);
    }
    colorwheel.slice_mut(s![col..col + yg, 1]).fill(255);
    col += yg;
    // GC段
    colorwheel.slice_mut(s![col..col + gc, 1]).fill(255);
    for (i, val) in (0..gc).enumerate() {
        colorwheel[[col + i, 2]] = ((255.0 * val as f32 / gc as f32) as u8);
    }
    col += gc;
    // CB段
    for (i, val) in (0..cb).enumerate() {
        colorwheel[[col + i, 1]] = (255 - (255.0 * val as f32 / cb as f32) as u8);
    }
    colorwheel.slice_mut(s![col..col + cb, 2]).fill(255);
    col += cb;
    // BM段
    colorwheel.slice_mut(s![col..col + bm, 2]).fill(255);
    for (i, val) in (0..bm).enumerate() {
        colorwheel[[col + i, 0]] = ((255.0 * val as f32 / bm as f32) as u8);
    }
    col += bm;
    // MR段
    for (i, val) in (0..mr).enumerate() {
        colorwheel[[col + i, 2]] = (255 - (255.0 * val as f32 / mr as f32) as u8);
    }
    colorwheel.slice_mut(s![col..col + mr, 0]).fill(255);

    colorwheel.into_dyn()
}

// 计算光流颜色图
fn compute_color(u: &ArrayD<f32>, v: &ArrayD<f32>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // 获取输入数组的形状（高度和宽度）
    let (h, w) = {
        let shape = u.shape();
        (shape[0], shape[1])
    };
    // 创建一个与输入数组大小相同的图像缓冲区
    let mut img = ImageBuffer::new(w as u32, h as u32);

    // 初始化一个数组，用于标记存在 NaN 值的位置
    let mut nan_idx = Array::zeros((h, w));
    // 遍历 u 数组，标记 NaN 值的位置
    for (i, &val_u) in u.iter().enumerate() {
        let (y, x) = (i / w, i % w);
        if val_u.is_nan() {
            nan_idx[[y, x]] = 1.0;
        }
    }
    // 遍历 v 数组，标记 NaN 值的位置
    for (i, &val_v) in v.iter().enumerate() {
        let (y, x) = (i / w, i % w);
        if val_v.is_nan() {
            nan_idx[[y, x]] = 1.0;
        }
    }

    // 持久变量
    let color_wheel = make_color_wheel();

    // 获取颜色轮的列数
    let ncols = color_wheel.shape()[0];
    // 计算每个像素的径向大小
    let rad = Array::from_shape_fn((h, w), |(y, x)| {
        let u_val = u[[y, x]];
        let v_val = v[[y, x]];
        (u_val.powi(2) + v_val.powi(2)).sqrt()
    });
    // 计算每个像素的角度
    let a = Array::from_shape_fn((h, w), |(y, x)| {
        let u_val = u[[y, x]];
        let v_val = v[[y, x]];
        (-v_val).atan2(-u_val) / PI
    });
    // 将角度映射到颜色轮的索引
    let fk = a.mapv(|a_val| (a_val + 1.0) / 2.0 * (ncols as f32 - 1.0) + 1.0);
    // 获取颜色轮索引的整数部分
    let k0 = fk.mapv(|val| val.floor() as i32);
    // 获取颜色轮索引的上界整数部分
    let k1 = k0.mapv(|val| (val + 1).clamp(1, ncols as i32));
    // 计算颜色轮索引的小数部分
    let k0_float = k0.mapv(|val| val as f32);
    let f: Vec<f32> = fk.iter().zip(&k0_float).map(|(fk_val, k0_val)| fk_val - k0_val).collect();

    // 遍历颜色通道
    for c in 0..3 {
        // 检查索引是否越界
        if c >= color_wheel.shape()[1] {
            panic!("Color channel index {} is out of bounds for color wheel with shape {:?}", c, color_wheel.shape());
        }
        // 获取颜色轮对应通道的颜色值
        let colorwheel_col = color_wheel.slice(s![.., c]);
        // 获取颜色轮索引 k0 对应的颜色值
        let col0 = k0.mapv(|val| colorwheel_col[val as usize] as f32 / 255.0);
        // 获取颜色轮索引 k1 对应的颜色值
        let col1 = k1.mapv(|val| colorwheel_col[val.min((ncols-1).try_into().unwrap()) as usize] as f32 / 255.0);
        // 插值计算最终的颜色值, 返回可克隆的容器
        let col = f.iter().zip(&col0).zip(&col1).map(|((f_val, col0_val), col1_val)| 
            (1.0 - f_val) * col0_val + f_val * col1_val
        ).collect_vec();  

        // 根据径向大小调整颜色值
        let idx = rad.mapv(|rad_val| rad_val <= 1.0);
        
        let col_with_rad = col.into_iter().zip(&rad).map(|(col_val, rad_val)| {
            if *rad_val <= 1.0 {
                1.0 - rad_val * (1.0 - col_val)
            } else {
                col_val
            }
        });

        // 对超出径向范围的颜色值进行衰减
        let not_idx = idx.mapv(|val| !val);
        let col_final = col_with_rad.zip(&not_idx).map(|(col_val, not_idx_val)| if *not_idx_val { col_val * 0.75 } else { col_val });

        // 将计算得到的颜色值赋值到图像缓冲区
        for (i, val) in col_final.enumerate() {
            let (y, x) = (i / w, i % w);
            let pixel_val = ((255.0 * val * (1.0 - nan_idx[[y, x]])) as u8).clamp(0, 255);
            img.put_pixel(x as u32, y as u32, Rgb([pixel_val, pixel_val, pixel_val]));
        }
    }

    // 返回生成的图像
    img
}

// 将光流转换为Middlebury颜色编码图像
fn flow_to_image(flow: &ArrayD<f32>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // 从光流数组中提取u分量（x方向运动）, 使用范围语法保持动态维度
    let u = flow.slice(s![0, 0, .., ..]);
    // 从光流数组中提取v分量（y方向运动）, 使用范围语法保持动态维度
    let v = flow.slice(s![0, 1, .., ..]);
    
    // 修正：使用map2替代zip+mapv组合
    let rad = u.mapv(|u_val| u_val.powi(2))
              .add(&v.mapv(|v_val| v_val.powi(2)))
              .mapv(|sum| sum.sqrt());
    
    // 找出所有像素中的最大光流幅度
    let maxrad = rad.iter().fold(f32::MIN, |acc, &val| acc.max(val));
    let eps = std::f32::EPSILON;
    
    // 将u分量裁剪到[-maxrad+5, maxrad-5]范围内
    let u_clipped = u.mapv(|val| val.clamp(-maxrad + 5.0, maxrad - 5.0));
    // 将v分量裁剪到[-maxrad+5, maxrad-5]范围内
    let v_clipped = v.mapv(|val| val.clamp(-maxrad + 5.0, maxrad - 5.0));
    
    // 将裁剪后的u分量归一化到[-1,1]范围
    let u_normalized = u_clipped.mapv(|val| val / (maxrad + eps)).into_dyn();;
    // 将裁剪后的v分量归一化到[-1,1]范围
    let v_normalized = v_clipped.mapv(|val| val / (maxrad + eps)).into_dyn();;

    // 返回
    compute_color(&u_normalized, &v_normalized)
}

// // 光流估计类
struct NeuFlowV2 {
    session: Session,
    input_names: Vec<String>,
    output_names: Vec<String>,
    input_height: usize,
    input_width: usize,
}

impl NeuFlowV2 {
    // 构造函数：从指定路径加载ONNX模型并初始化NeuFlowV2结构体
    fn new(path: &str) -> Result<Self> {
        // 检查模型文件是否存在及有效性（假设check_model是自定义函数）
        check_model(path)?;
        
        // 使用ort库创建推理会话：通过SessionBuilder加载ONNX模型文件
        // ort库是ONNX Runtime的Rust封装，支持高性能推理
        let session = Session::builder()?.commit_from_file(path)?;

        // 直接指定输入节点名称（根据提示）
        let input_names = vec![
            "input1".to_string(),
            "input2".to_string()
        ];

        // 直接指定输出节点名称（根据提示）
        let output_names = vec![
            "output".to_string()
        ];

        // 获取输入形状（根据提示中的tensor形状）, float32[1,3,432,768]
        let input_shape = [1, 3, 432, 768]; 
        // 432
        let input_height = input_shape[2] as usize; 
        // 768
        let input_width = input_shape[3] as usize;  

        // 返回初始化完成的NeuFlowV2结构体
        Ok(NeuFlowV2 {
            // ONNX推理会话
            session,    
            // 输入节点名称列表      
            input_names,      
            // 输出节点名称列表
            output_names,     
            // 模型要求的输入图像高度
            input_height,     
            // 模型要求的输入图像宽度
            input_width,      
        })
    }

     // 光流估计：输入前后两帧图像，输出光流估计结果
     fn estimate_flow(&mut self, img_prev: &ImageBuffer<Rgb<u8>, Vec<u8>>, img_now: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<ArrayD<f32>> {
        // 预处理输入图像
        let (input_prev, input_now) = self.prepare_inputs(img_prev, img_now)?;
        // 执行模型推理
        let outputs = self.inference(&input_prev, &input_now)?;
        // 处理并返回输出结果
        Ok(self.process_output(&outputs[0]))
    }

    // 预处理输入：准备前后两帧图像的输入张量
    fn prepare_inputs(&mut self, img_prev: &ImageBuffer<Rgb<u8>, Vec<u8>>, img_now: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<(ArrayD<f32>, ArrayD<f32>)> {
        // 获取当前帧图像的尺寸
        let img_height = img_now.height() as usize;
        let img_width = img_now.width() as usize;

        // 分别预处理前后两帧图像
        let input_prev = self.prepare_input(img_prev)?;
        let input_now = self.prepare_input(img_now)?;

        // 返回预处理后的输入张量
        Ok((input_prev, input_now))
    }

    // 准备单帧输入：将图像转换为模型需要的张量格式
    fn prepare_input(&mut self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<ArrayD<f32>> {
        // 将图像缩放到模型要求的尺寸（使用最近邻插值）
        let dynamic_img = DynamicImage::ImageRgb8(img.clone());
        let input_img_dyn = dynamic_img.resize_exact(
            self.input_width as u32, 
            self.input_height as u32, 
            image::imageops::FilterType::Nearest
        );
        // 将缩放后的 DynamicImage 转换回 ImageBuffer<Rgb<u8>, Vec<u8>>
        let input_img: ImageBuffer<Rgb<u8>, Vec<u8>> = input_img_dyn.clone().into_rgb8();

        // 创建4通道的零张量（1, 通道，高度，宽度）
        let mut input_array = Array::zeros((1, 3, self.input_height, self.input_width));

        // 遍历图像像素，归一化并填充到张量中
        for (x, y, pixel) in input_img.enumerate_pixels() {
            // R通道归一化
            input_array[[0, 0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
            // G通道归一化
            input_array[[0, 1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
            // B通道归一化
            input_array[[0, 2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
        }

        // 将固定维度的数组转换为动态维度张量
        let input_tensor = input_array.into_dyn();
        Ok(input_tensor)
    }

    // 执行推理：输入前后两帧的张量，返回输出结果
    fn inference(&mut self, input_prev: &ArrayD<f32>, input_now: &ArrayD<f32>) -> Result<Vec<Tensor<f32>>> {
        // 记录推理开始时间
        let start = Instant::now();
        // 将输入张量转换为ORT支持的Value类型
        let input_tensor_prev = ort::value::Value::from_array(input_prev.clone().into_dyn())?;
        let input_tensor_now = ort::value::Value::from_array(input_now.clone().into_dyn())?;

        // 执行模型推理（输入两个张量）
        let outputs = self.session.run(inputs![input_tensor_prev, input_tensor_now])?;

        // 计算并打印推理耗时
        let elapsed = start.elapsed();
        println!("推理时间: {:.2} ms", elapsed.as_secs_f64() * 1000.0);

        // 提取所有输出张量
        use ort::value::{ Value, TensorValueType };
        let mut result: Vec<Tensor<f32>> = Vec::new(); 
        for output_name in &self.output_names {
            let (shape, data) = outputs[output_name.clone()].try_extract_tensor::<f32>()?;
            
            // 将 ort::Shape 转换为 ndarray 兼容的维度（动态维度）
            let nd_shape: Vec<usize> = shape.iter().map(|&d| d as usize).collect();
            let array = Array::from_shape_vec(IxDyn(&nd_shape), data.to_vec())?;
            
            let output = Value::from_array(array)?;
            result.push(output);
        }

        // 返回值
        Ok(result)
    }

    // 处理输出：将输出张量转换为合适的格式
    fn process_output(&mut self, output: &Tensor<f32>) -> ArrayD<f32> {
        // 解构输出张量（形状和数据）
        let (shape, data) = output.try_extract_tensor::<f32>().unwrap();
        // 根据形状创建4维数组（1, 通道，高度，宽度）
        let array = Array::from_shape_vec(
            Dim::<[usize; 4]>::new([1 as usize, shape[1] as usize, shape[2] as usize, shape[3] as usize]), 
            data.to_vec()
        ).unwrap();
        // 转换为动态维度张量
        let array_dyn = array.into_dyn();
        let flow = array_dyn.to_shape(
            Dim::<[usize; 4]>::new([1 as usize, shape[1] as usize, shape[2] as usize, shape[3] as usize])
        ).unwrap();
        // 映射所有值（此处未做实际转换，保留原值）
        let resized_flow = flow.mapv(|val| val);
        let resized_flow_dyn = resized_flow.into_dyn();
        // 返回
        resized_flow_dyn
    }

}

fn main() -> Result<()> {
    // 模型文件路径
    let model_path = "../../assets/ailia-models/neuflow_v2/neuflow_sintel.onnx";
    println!("模型文件路径{}", model_path);

    // 初始化模型
    let mut estimator = NeuFlowV2::new(model_path)?;

    // 加载第一张图片
    let img1 = image::open("./assets/frame_0016.png")?.to_rgb8();
    // 加载第二张图片
    let img2 = image::open("./assets/frame_0025.png")?.to_rgb8();

    // 估计光流
    let flow = estimator.estimate_flow(&img1, &img2)?;

    // 绘制光流图
    let flow_img = flow_to_image(&flow);

    // 创建结果文件夹
    std::fs::create_dir_all(Path::new("./result"))?;

    // 保存结果图片
    flow_img.save("./result/ort_optics.png")?;
    println!("光流预测结果已保存到 ./result/ort_optics.png");

    Ok(())
}