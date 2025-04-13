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

// 导入 ORT 相关模块
use ort::{inputs, session::Session};
use ort::value::Tensor;

// 图像处理相关模块
use image::{GenericImageView, ImageBuffer, Rgb, RgbImage};

// 错误处理模块
use anyhow::{anyhow, Result};

// 线性代数库
use ndarray::{
    Array, ArrayD, ArrayViewD, 
    Dim, Ix1, Ix2, Ix3, Ix4, 
    IxDyn, stack, Axis, IxDynImpl, 
    ArrayBase, OwnedRepr
};
use ndarray::s;

// 随机数
use rand::rng;

// 随机分布
use rand_distr::{Normal, Distribution};

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
    let (h, w) = u.shape();
    // 创建一个与输入数组大小相同的图像缓冲区
    let mut img = ImageBuffer::new(w as u32, h as u32);

    // 初始化一个数组，用于标记存在NaN值的位置
    let mut nan_idx = Array::zeros((h, w));
    // 遍历u数组，标记NaN值的位置
    for (i, &val_u) in u.iter().enumerate() {
        let (y, x) = u.index().index(i);
        if val_u.is_nan() {
            nan_idx[[y, x]] = 1.0;
        }
    }
    // 遍历v数组，标记NaN值的位置
    for (i, &val_v) in v.iter().enumerate() {
        let (y, x) = v.index().index(i);
        if val_v.is_nan() {
            nan_idx[[y, x]] = 1.0;
        }
    }

    // 获取颜色轮的列数
    let ncols = make_color_wheel().shape()[0];
    // 计算每个像素的径向大小
    let rad = u.zip(v).mapv(|(u_val, v_val)| (u_val.powi(2) + v_val.powi(2)).sqrt());
    // 计算每个像素的角度
    let a = u.zip(v).mapv(|(u_val, v_val)| (-v_val.atan2(-u_val) / std::f32::consts::PI));
    // 将角度映射到颜色轮的索引
    let fk = a.zip(&Array::from_elem((h, w), 1.0)).mapv(|(a_val, _)| ((a_val + 1.0) / 2.0 * (ncols as f32 - 1.0) + 1.0));
    // 获取颜色轮索引的整数部分
    let k0 = fk.mapv(|val| val.floor() as i32);
    // 获取颜色轮索引的上界整数部分
    let k1 = k0.mapv(|val| (val + 1).clamp(1, ncols as i32));
    // 计算颜色轮索引的小数部分
    let f = fk.zip(&k0.mapv(|val| val as f32)).mapv(|(fk_val, k0_val)| fk_val - k0_val);

    // 遍历颜色通道
    for c in 0..3 {
        // 获取颜色轮对应通道的颜色值
        let colorwheel_col = make_color_wheel().slice(s![.., c]);
        // 获取颜色轮索引k0对应的颜色值
        let col0 = colorwheel_col.gather(&k0.mapv(|val| val as usize)).mapv(|val| val as f32 / 255.0);
        // 获取颜色轮索引k1对应的颜色值
        let col1 = colorwheel_col.gather(&k1.mapv(|val| val as usize)).mapv(|val| val as f32 / 255.0);
        // 插值计算最终的颜色值
        let col = f.zip(&col0).zip(&col1).mapv(|((f_val, col0_val), col1_val)| (1.0 - f_val) * col0_val + f_val * col1_val);

        // 根据径向大小调整颜色值
        let idx = rad.zip(&Array::from_elem((h, w), 1.0)).mapv(|(rad_val, _)| rad_val <= 1.0);
        let col_with_rad = col.zip(&idx).mapv(|(col_val, idx_val)| if idx_val { 1.0 - rad_val * (1.0 - col_val) } else { col_val });
        // 对超出径向范围的颜色值进行衰减
        let not_idx = idx.mapv(|val|!val);
        let col_final = col_with_rad.zip(&not_idx).mapv(|(col_val, not_idx_val)| if not_idx_val { col_val * 0.75 } else { col_val });

        // 将计算得到的颜色值赋值到图像缓冲区
        for (i, &val) in col_final.iter().enumerate() {
            let (y, x) = col_final.index().index(i);
            let pixel_val = ((255.0 * val * (1.0 - nan_idx[[y, x]])) as u8).clamp(0, 255);
            img.put_pixel(x as u32, y as u32, Rgb([pixel_val, pixel_val, pixel_val]));
        }
    }

    // 返回生成的图像
    img
}

fn main(){}

// // 将光流转换为Middlebury颜色编码图像
// fn flow_to_image(flow: &ArrayD<f32>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
//     let u = flow.slice(s![.., 0]);
//     let v = flow.slice(s![.., 1]);
//     let rad = u.zip(v).mapv(|(u_val, v_val)| (u_val.powi(2) + v_val.powi(2)).sqrt());
//     let maxrad = rad.iter().fold(f32::MIN, |acc, &val| acc.max(val));
//     let eps = std::f32::EPSILON;
//     let u_clipped = u.mapv(|val| val.clamp(-maxrad + 5.0, maxrad - 5.0));
//     let v_clipped = v.mapv(|val| val.clamp(-maxrad + 5.0, maxrad - 5.0));
//     let u_normalized = u_clipped.mapv(|val| val / (maxrad + eps));
//     let v_normalized = v_clipped.mapv(|val| val / (maxrad + eps));

//     compute_color(&u_normalized, &v_normalized)
// }

// // 光流估计类
// struct NeuFlowV2 {
//     session: Session,
//     input_names: Vec<String>,
//     output_names: Vec<String>,
//     input_height: usize,
//     input_width: usize,
// }

// impl NeuFlowV2 {
//     fn new(path: &str) -> Result<Self> {
//         check_model(path)?;
//         let session = Session::builder()?.commit_from_file(path)?;

//         let mut input_names = Vec::new();
//         let model_inputs = session.get_inputs();
//         for i in 0..model_inputs.len() {
//             input_names.push(model_inputs[i].name().to_string());
//         }

//         let mut output_names = Vec::new();
//         let model_outputs = session.get_outputs();
//         for i in 0..model_outputs.len() {
//             output_names.push(model_outputs[i].name().to_string());
//         }

//         let input_shape = model_inputs[0].shape();
//         let input_height = input_shape[2] as usize;
//         let input_width = input_shape[3] as usize;

//         Ok(NeuFlowV2 {
//             session,
//             input_names,
//             output_names,
//             input_height,
//             input_width,
//         })
//     }

//     fn estimate_flow(&self, img_prev: &ImageBuffer<Rgb<u8>, Vec<u8>>, img_now: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<ArrayD<f32>> {
//         let (input_prev, input_now) = self.prepare_inputs(img_prev, img_now)?;
//         let outputs = self.inference(&input_prev, &input_now)?;
//         Ok(self.process_output(outputs[0]))
//     }

//     fn prepare_inputs(&self, img_prev: &ImageBuffer<Rgb<u8>, Vec<u8>>, img_now: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<(ArrayD<f32>, ArrayD<f32>)> {
//         let img_height = img_now.height() as usize;
//         let img_width = img_now.width() as usize;

//         let input_prev = self.prepare_input(img_prev)?;
//         let input_now = self.prepare_input(img_now)?;

//         Ok((input_prev, input_now))
//     }

//     fn prepare_input(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<ArrayD<f32>> {
//         let input_img = img.resize_exact(self.input_width as u32, self.input_height as u32, image::imageops::FilterType::Nearest);
//         let mut input_array = Array::zeros((3, self.input_height, self.input_width));

//         for (x, y, pixel) in input_img.enumerate_pixels() {
//             input_array[[0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
//             input_array[[1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
//             input_array[[2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
//         }

//         let input_tensor = input_array.into_dyn();
//         Ok(input_tensor)
//     }

//     fn inference(&self, input_prev: &ArrayD<f32>, input_now: &ArrayD<f32>) -> Result<Vec<Tensor<f32>>> {
//         let start = Instant::now();
//         let input_tensor_prev = ort::value::Value::from_array(input_prev.clone().into_dyn())?;
//         let input_tensor_now = ort::value::Value::from_array(input_now.clone().into_dyn())?;

//         let outputs = self.session.run(inputs![input_tensor_prev, input_tensor_now])?;

//         let elapsed = start.elapsed();
//         println!("推理时间: {:.2} ms", elapsed.as_secs_f64() * 1000.0);

//         let mut result = Vec::new();
//         for output_name in &self.output_names {
//             let output = outputs[output_name].try_extract_tensor::<f32>()?;
//             result.push(output);
//         }

//         Ok(result)
//     }

//     fn process_output(&self, output: Tensor<f32>) -> ArrayD<f32> {
//         let (shape, data) = output;
//         let array = Array::from_shape_vec((shape[1], shape[2], shape[3]), data.to_vec()).unwrap();
//         let flow = array.into_dyn().into_shape((shape[1], shape[2], shape[3])).unwrap();
//         let resized_flow = flow.mapv(|val| val);
//         resized_flow
//     }
// }

// fn main() -> Result<()> {
//     // 模型文件路径
//     let model_path = "../../assets/ailia-models/neuflow_v2/neuflow_sintel.onnx";
//     println!("模型文件路径{}", model_path);

//     // 初始化模型
//     let estimator = NeuFlowV2::new(model_path)?;

//     // 加载第一张图片
//     let img1 = image::open("./assets/frame_0016.png")?.to_rgb8();
//     // 加载第二张图片
//     let img2 = image::open("./assets/frame_0025.png")?.to_rgb8();

//     // 估计光流
//     let flow = estimator.estimate_flow(&img1, &img2)?;

//     // 绘制光流图
//     let flow_img = flow_to_image(&flow);

//     // 创建结果文件夹
//     std::fs::create_dir_all(Path::new("./result"))?;

//     // 保存结果图片
//     flow_img.save("../result/ort_optics.png")?;
//     println!("光流预测结果已保存到 ./result/ort_optics.png");

//     Ok(())
// }