#![allow(unused)]
#![allow(deprecated)]

//! 测试 SAM2.1 图片分割
//! 编码模型: ../../../assets/ailia-models/segment-anything-2/sam2.1_base_plus.encoder.onnx
//! 输出:
//! high_res_feats_0
//! name: high_res_feats_0
//! tensor: float32[Reshapehigh_res_feats_0_dim_0,Reshapehigh_res_feats_0_dim_1,Reshapehigh_res_feats_0_dim_2,Reshapehigh_res_feats_0_dim_3]
//! high_res_feats_1
//! name: high_res_feats_1
//! tensor: float32[Reshapehigh_res_feats_1_dim_0,Reshapehigh_res_feats_1_dim_1,Reshapehigh_res_feats_1_dim_2,Reshapehigh_res_feats_1_dim_3]
//! image_embed
//! name: image_embed
//! tensor: float32[Reshapeimage_embed_dim_0,Reshapeimage_embed_dim_1,Reshapeimage_embed_dim_2,Reshapeimage_embed_dim_3]
//!
//! 解码模型: ../../../assets/ailia-models/segment-anything-2/sam2.1_base_plus.decoder.onnx
//! 输出:
//! masks
//! name: masks
//! tensor: float32[Resizemasks_dim_0,Resizemasks_dim_1,Resizemasks_dim_2,Resizemasks_dim_3]
//! iou_predictions
//! name: iou_predictions
//! tensor: float32[Resizemasks_dim_0,Whereiou_predictions_dim_1]
//!
//! 输入图片 ./assets/rgb1.png
//! 输出 ./result/segment_onnx.png

// 导入标准库路径处理模块
use std::path::Path;

// 导入 ORT 相关模块
use ort::{inputs, session::Session};
use ort::value::Tensor;

// 图像处理相关模块
use image::{GenericImageView, ImageBuffer, Rgb};

// 错误处理模块
use anyhow::Result;

// 线性代数库
use ndarray::{ 
    Array, ArrayD, ArrayViewD, 
    Dim, Ix1, Ix2, Ix3, Ix4, 
    IxDyn, stack, Axis,
    IxDynImpl, ArrayBase,
};
use ndarray::OwnedRepr;

// 随机数
use rand::rng;

// 随机分布
use rand_distr::{ Normal, Distribution};

// 生成截断正态分布的随机数
fn trunc_normal(size: &[usize], std: f32, a: f32, b: f32) -> ArrayD<f32> {
    // 创建正态分布（需处理可能的错误）
    let dist = Normal::new(0.0, std as f64).expect("Invalid normal distribution parameters");
    let mut rng = rand::rng();
    
    // 使用 mapv 进行向量化操作
    ArrayD::zeros(size).mapv(|_ : f32| {
        let v = dist.sample(&mut rng) as f32;
        v.clamp(a * std, b * std) // 等价于 max(a).min(b)
    })
}

// 转换坐标
fn transform_coords(coords: &mut Array<f32, Ix2>, orig_hw: (u32, u32)) {
    let (h, w) = (orig_hw.0 as f32, orig_hw.1 as f32);
    coords.index_axis_mut(ndarray::Axis(1), 0).mapv_inplace(|x| x / w);
    coords.index_axis_mut(ndarray::Axis(1), 1).mapv_inplace(|y| y / h);

    let resolution = 1024.0;
    coords.mapv_inplace(|x| x * resolution);
}

// 转换边框
fn transform_boxes(boxes: &mut Array<f32, Ix2>, orig_hw: (u32, u32)) {
    // 直接处理二维数组
    transform_coords(boxes, orig_hw); 
}

// 后处理掩码
fn postprocess_masks(masks: &Array<f32, Ix4>, orig_hw: (u32, u32)) -> Array<f32, Ix4> {

    // 创建一个用于存储插值后的掩码的向量
    let mut interpolated_masks = Vec::new();

    // 遍历输入的掩码数组
    for mask in masks.outer_iter() {

        // 将当前掩码转换为动态维度数组
        let mut _mask = mask.into_dyn();

        // 使用临时变量解决借用冲突
        let mut temp_mask: ArrayViewD<f32>;

        // 检查掩码的维度，如果为4，则提取第一个维度的数据
        if _mask.ndim() == 4 {
            // 转换为动态维度
            temp_mask = _mask.index_axis(ndarray::Axis(0), 0).into_dyn();
        } else {
            // 如果不是4维，直接赋值, 使用「视图」
            temp_mask = _mask.view(); 
        }

        // 如果掩码的维度不是3，则抛出异常
        if _mask.ndim() != 3 {
            panic!("Unexpected mask shape: {:?}", mask.shape());
        }

        // 翻转掩码的坐标轴
        let mut mask_ = temp_mask.reversed_axes();
        // 使用image库将掩码转换为图像，并进行缩放
        let resized_mask = image::imageops::resize(
            &ImageBuffer::from_fn(
                mask_.shape()[1] as u32,
                mask_.shape()[0] as u32,
                |x, y| Rgb([(mask_[[y as usize, x as usize, 0]] * 255.0) as u8; 3]),
            ),
            orig_hw.1,
            orig_hw.0,
            image::imageops::FilterType::Nearest,
        );

        // 将缩放后的图像转换为数组
        let mut resized_mask = Array::from_shape_fn(
            (resized_mask.height() as usize, resized_mask.width() as usize, 1),
            |(y, x, _)| resized_mask.get_pixel(x as u32, y as u32)[0] as f32 / 255.0,
        );

        // 再次翻转数组的坐标轴
        resized_mask = resized_mask.reversed_axes();

        // 将处理后的掩码添加到向量中
        interpolated_masks.push(resized_mask);
    }
    // 创建一个视图数组
    let views: Vec<_> = interpolated_masks.iter()
        .map(|arr| arr.view())
        .collect();
    
    // 将视图数组堆叠成一个新的数组
    ndarray::stack(ndarray::Axis(0), &views).unwrap()
}

struct SAM2ImagePredictor {
    encoder_session: Session,
    decoder_session: Session,
}

impl SAM2ImagePredictor {
    // 构造函数
    fn new(encoder_model_path: &str, decoder_model_path: &str) -> Result<Self> {
        let encoder_session = Session::builder()?.commit_from_file(encoder_model_path)?;
        let decoder_session = Session::builder()?.commit_from_file(decoder_model_path)?;
        Ok(SAM2ImagePredictor {
            encoder_session,
            decoder_session,
        })
    }

    // 修正返回类型，明确返回的三个数组的具体形状
    fn set_image(&mut self, image: &image::DynamicImage) -> Result<(Array<f32, Ix4>, Array<f32, Ix4>, Array<f32, Ix4>)> {
        // 调整图像大小为模型期望的尺寸
        let resized_img = image.resize_exact(1024, 1024, image::imageops::FilterType::Nearest);
        // 创建四维数组保存预处理数据 [batch=1, channels=3, height=1024, width=1024]
        let mut input_array = Array::zeros((1, 3, 1024, 1024));
        // 遍历所有像素进行归一化处理
        for (x, y, pixel) in resized_img.to_rgb8().enumerate_pixels() {
            // 归一化 R 通道
            input_array[[0, 0, y as usize, x as usize]] = pixel[0] as f32 / 255.0; 
            // 归一化 G 通道
            input_array[[0, 1, y as usize, x as usize]] = pixel[1] as f32 / 255.0; 
            // 归一化 B 通道
            input_array[[0, 2, y as usize, x as usize]] = pixel[2] as f32 / 255.0; 
        }
        // 将数组转换为 ORT 张量
        let input_tensor = ort::value::Value::from_array(input_array.clone())?;

        // 运行编码器模型推理
        let outputs = self.encoder_session.run(inputs![input_tensor])?;
        // 提取推理结果中的特征和嵌入向量
        let (high_res_feats_0_shape, high_res_feats_0) = outputs["high_res_feats_0"].try_extract_tensor::<f32>()?;
        let (high_res_feats_1_shape, high_res_feats_1) = outputs["high_res_feats_1"].try_extract_tensor::<f32>()?;
        let (image_embed_shape, image_embed) = outputs["image_embed"].try_extract_tensor::<f32>()?;

        // println!("high_res_feats_0的shape大小: {}", outputs["high_res_feats_0"].try_extract_tensor::<f32>()?.0);
        // println!("high_res_feats_1的shape大小: {}", outputs["high_res_feats_1"].try_extract_tensor::<f32>()?.0);
        // println!("image_embed的shape大小: {}", outputs["image_embed"].try_extract_tensor::<f32>()?.0);

        // 将特征和嵌入向量转换为 ndarray 数组
        Ok((
            // 高分辨率特征 0
            Array::from_shape_vec(
                Dim::<[usize; 4]>::new(
                    [high_res_feats_0_shape[0] as usize, high_res_feats_0_shape[1] as usize, 
                    high_res_feats_0_shape[2] as usize, high_res_feats_0_shape[3] as usize]
                ), 
                high_res_feats_0.to_vec()
            )?, 
            // 高分辨率特征 1
            Array::from_shape_vec(
                Dim::<[usize; 4]>::new(
                    [high_res_feats_1_shape[0] as usize, high_res_feats_1_shape[1] as usize, 
                    high_res_feats_1_shape[2] as usize, high_res_feats_1_shape[3] as usize]
                ), 
                high_res_feats_1.to_vec()
            )?, 
            // 图像嵌入向量
            Array::from_shape_vec(
                Dim::<[usize; 4]>::new(
                    [image_embed_shape[0] as usize, image_embed_shape[1] as usize,
                    image_embed_shape[2] as usize, image_embed_shape[3] as usize]
                ),
                image_embed.to_vec()
            )?, 
        ))
    }

    // 执行预测
    fn predict(
        &mut self,
        features: (Array<f32, Ix4>, Array<f32, Ix4>, Array<f32, Ix4>), 
        orig_hw: (u32, u32), 
        point_coords: Option<Array<f32, Ix2>>, 
        point_labels: Option<Array<f32, Ix1>>, 
        box_coords: Option<Array<f32, Ix2>>, 
        mask_input: Option<Array<f32, Ix4>>, 
    ) -> Result<(Array<f32, Ix4>, Array<f32, Ix2>, Array<f32, Ix4>)> {
        // 初始化未归一化的坐标和标签
        let mut unnorm_coords: Option<ArrayD<f32>> = None;
        let mut labels: Option<ArrayD<f32>> = None;
    
        // 如果有输入点坐标
        if let Some(mut coords) = point_coords {
            if coords.len() != 0 {
                // 转换坐标
                transform_coords(&mut coords, orig_hw); 
    
                // 插入轴
                let coords_3d = coords.insert_axis(Axis(0)).into_dyn();
                let lbls = point_labels.unwrap().insert_axis(Axis(0)).into_dyn();
    
                // 设置未归一化的坐标
                unnorm_coords = Some(coords_3d); 
                // 设置标签
                labels = Some(lbls); 
            }
        }
    
        // 初始化未归一化的边框
        let mut unnorm_box: Option<ArrayD<f32>> = None;
        if let Some(mut box_) = box_coords {
            // 调整边框形状
            let mut box_3d = box_
                .to_shape((1, 2, 2))
                .unwrap()
                .into_owned()
                .into_dyn();
    
            // 转换边框
            // 动态数组转为固定维度数组再调用处理函数
            // 转换为固定二维数组
            let mut fixed_box_3d: ArrayBase<_, Ix2> = match box_3d.view_mut().to_owned().into_dimensionality::<Ix2>() {
                Ok(array) => array,
                Err(_) => panic!("The array is not 2-dimensional!"),
            };
            transform_boxes(&mut fixed_box_3d.to_owned(), orig_hw); 
            unnorm_box = Some(box_3d);
        }
    
        // 处理输入掩码
        let mut mask_input = mask_input.map(|mut m| {
            if m.ndim() == 3 {
                // 插入轴
                // m = m.insert_axis(Axis(0)); 
            }
            m.into_dyn()
        });
        // 如果没有输入掩码，创建一个默认的掩码
        if mask_input.is_none() {
            mask_input = Some(Array::zeros((1, 1, 256, 256)).into_dyn());
        }
    
        // 如果没有点坐标和边框，创建默认的点坐标和标签
        if unnorm_coords.is_none() && unnorm_box.is_none() {
            unnorm_coords = Some(Array::from_shape_vec((1, 1, 2), vec![0.5, 0.5]).unwrap().into_dyn());
            labels = Some(Array::from_shape_vec((1, 1), vec![1.0]).unwrap().into_dyn());
        }
    
        // 合并点坐标和边框
        let mut concat_points: Option<(ArrayD<f32>, ArrayD<f32>)> = None;
        if let Some(coords) = unnorm_coords {
            concat_points = Some((coords, labels.unwrap()));
        }
    
        if let Some(mut box_coords) = unnorm_box {
            // 创建边框标签
            let box_labels = Array::from_shape_vec((1, 2), vec![2.0, 3.0]).unwrap().into_dyn();
            if let Some((mut concat_coords, mut concat_labels)) = concat_points {
                // 合并点坐标和边框坐标
                let concat_coords = ndarray::stack(Axis(1), &[box_coords.view(), concat_coords.view()]).unwrap().into_dyn();
                // 合并点标签和边框标签
                let concat_labels = ndarray::stack(Axis(1), &[box_labels.view(), concat_labels.view()]).unwrap().into_dyn();
                concat_points = Some((concat_coords, concat_labels));
            } else {
                concat_points = Some((box_coords, box_labels));
            }
        }
    
        // 获取掩码输入
        let mask_input_dummy = mask_input.unwrap();
        // 创建掩码启用标志
        let masks_enable = Array::from_shape_vec((1,), vec![if mask_input_dummy.len() > 0 { 1.0 } else { 0.0 }]).unwrap().into_dyn();
    
        // 解构输入特征
        let (high_res_feats_0, high_res_feats_1, image_embed) = features;
        // 创建原始图像大小数组
        let orig_im_size = Array::from_shape_vec((2,), vec![orig_hw.0 as i32, orig_hw.1 as i32]).unwrap().into_dyn();
    
        // 获取合并后的点坐标和标签
        let (concat_coords, concat_labels) = concat_points.ok_or_else(|| anyhow::anyhow!("concat_points must be exists"))?;
    
        // 在 predict 方法中
        let outputs = self.decoder_session.run(inputs![
            "point_coords" => ort::value::Value::from_array(concat_coords.mapv(|x| x as f32))?,
            "point_labels" => ort::value::Value::from_array(concat_labels.mapv(|x| x as f32))?,
            "mask_input" => ort::value::Value::from_array(mask_input_dummy.mapv(|x| x as f32))?,
            "has_mask_input" => ort::value::Value::from_array(masks_enable.mapv(|x| x as f32))?,
            "orig_im_size" => ort::value::Value::from_array(orig_im_size.mapv(|x| x as i32))?,
            "image_embed" => ort::value::Value::from_array(image_embed.mapv(|x| x as f32))?,
            "high_res_feats_0" => ort::value::Value::from_array(high_res_feats_0.mapv(|x| x as f32))?,
            "high_res_feats_1" => ort::value::Value::from_array(high_res_feats_1.mapv(|x| x as f32))?
        ])?;
    
        // 提取推理结果
        let (masks_shape, masks) = outputs["masks"].try_extract_tensor::<f32>()?;
        let (iou_pred_shape, iou_pred) = outputs["iou_predictions"].try_extract_tensor::<f32>()?;

        println!("masks_shape: {}", masks_shape);
        println!("iou_pred_shape: {}", iou_pred_shape);
    
        // 将推理结果转换为 ndarray 数组, 掩码
        let mut masks = Array::from_shape_vec((masks.len(),), masks.to_vec())?.into_dyn();
        // IOU 预测
        let iou_pred = Array::from_shape_vec((iou_pred.len(),), iou_pred.to_vec())?.into_dyn();
    
        // FIXME: 这两个返回值没有用到, 所以直接忽略
        // 提取低分辨率掩码
        // let low_res_masks = masks.slice(ndarray::s![.., 1.., .., ..]).to_owned();
        let low_res_masks: Array<f32, Ix4> = masks.clone().into_shape((1, 1, 480, 640))?;
        // 提取 IOU 预测
        // let iou_predictions = iou_pred.slice(ndarray::s![.., 1..]).to_owned();
        let iou_predictions: Array<f32, Ix2> = iou_pred.clone().into_shape((1, 1))?;
    
        // 后处理掩码
        // 动态数组转为固定维度数组再调用处理函数
        // 转换为固定四维数组

        let mut fixed_masks = low_res_masks.clone();
        // let mut fixed_masks: ArrayBase<_, Ix4> = match masks.view_mut().to_owned().into_dimensionality::<Ix4>() {
        //     Ok(array) => array,
        //     Err(_) => {
        //         // panic!("The array is not 4-dimensional!"),
        //     }
        // };


        let masks = postprocess_masks(&mut fixed_masks, orig_hw);
    
        // 返回结果
        Ok((masks, iou_predictions, low_res_masks))
        // Ok((masks, Array<f32, Ix2>::zeros(), Array<f32, Ix4>::zeros()))
    }

}

fn main() -> Result<()> {
    // 输入图片路径
    let input_image_path = "./assets/rgb1.png";
    // 输出图片路径
    let output_image_path = "./result/ort_segment.png";
    // 编码器模型路径
    let encoder_model_path = "../../assets/ailia-models/segment-anything-2/sam2.1_base_plus.encoder.onnx";
    // 解码器模型路径
    let decoder_model_path = "../../assets/ailia-models/segment-anything-2/sam2.1_base_plus.decoder.onnx";

    // 创建输出目录（如果不存在）
    std::fs::create_dir_all(Path::new(output_image_path).parent().unwrap())?;

    // 读取输入图片
    let image = image::open(input_image_path)?;
    let orig_hw = (image.height(), image.width());

    // 初始化 SAM2 图像分割预测器
    let mut predictor = SAM2ImagePredictor::new(encoder_model_path, decoder_model_path)?;

    // 设置输入图像并获取编码器的特征
    let features = predictor.set_image(&image)?;

    // 进行图像分割预测
    let (masks, iou_predictions, low_res_masks) = predictor.predict(features, orig_hw, None, None, None, None)?;

    // FIXME: 绘图错误
    // 将分割掩码转换为可视化图像 
    let mask = masks.slice(ndarray::s![0, 0, .., ..]).mapv(|x| (x * 255.0) as u8);
    let mut img_buffer = ImageBuffer::from_fn(orig_hw.1, orig_hw.0, |x, y| Rgb([mask[[x as usize, y as usize]]; 3]));

    // 将分割掩码叠加到原始图像上
    let orig_img = image.to_rgb8();
    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let orig_pixel = orig_img.get_pixel(x, y);
        *pixel = Rgb([
            ((orig_pixel[0] as f32 * 0.5 + pixel[0] as f32 * 0.5) as u8),
            ((orig_pixel[1] as f32 * 0.5 + pixel[1] as f32 * 0.5) as u8),
            ((orig_pixel[2] as f32 * 0.5 + pixel[2] as f32 * 0.5) as u8),
        ]);
    }

    // 保存输出图片
    img_buffer.save(output_image_path)?;

    Ok(())
}