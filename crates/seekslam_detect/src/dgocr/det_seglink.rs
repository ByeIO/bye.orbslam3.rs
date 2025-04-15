#![allow(unused)]

//! 检测(seglink技术)

// 线性代数库
use ndarray::{
    Array, ArrayD, ArrayViewD, 
    Dim, Ix1, Ix2, Ix3, Ix4, 
    IxDyn, Axis, IxDynImpl, 
    ArrayBase, OwnedRepr, IntoDimension,
    ArrayView, Array1, Array2, Array3,
    Array4, CowArray, ArrayView1, ShapeError,
};
use ndarray::s;
use ndarray::concatenate;
use ndarray::stack;

// 错误处理
use anyhow::{anyhow, Result};

// 图像处理相关模块
use image::{GenericImageView, ImageBuffer, Rgb, RgbImage, DynamicImage};

// onnx运行时
use ort::environment::Environment;
use ort::session::Session;
use ort::value::{ TensorValueType, Value , Tensor };
use ort::session::builder::GraphOptimizationLevel;

// 标准库
use std::path::Path;

// 内部库
use super::utils_seglink::{
    decode_segments_links_python, combine_segments_python,
    cal_width, nms_python, rboxes_to_polygons, 
};

// 模型结构
pub struct SegLinkOCRDetection {
    pub model_path: String,
    pub cpu_thread_num: i32,
    pub sess: Session,
    pub output: std::collections::HashMap<String, ArrayD<f32>>,
}

impl SegLinkOCRDetection {
    // 构造函数
    pub fn new(model: &str, cpu_thread_num: i32) -> Result<Self> {
        let model_path = model.to_string();
        let cpu_thread_num = cpu_thread_num;
        // 使用ort库创建推理会话：通过SessionBuilder加载ONNX模型文件
        // ort库是ONNX Runtime的Rust封装，支持高性能推理
        let session = Session::builder()?.commit_from_file(&model_path)?;
        let output = std::collections::HashMap::new();
        Ok(Self {
            model_path,
            cpu_thread_num,
            sess : session,
            output,
        })
    }

    // 图片预处理
    pub fn preprocess(&self, input: &str) -> Result<ArrayD<f32>> {
        // 读取图片
        let img = image::open(input)?;
        let (w, h) = img.dimensions();
        let img = img.to_rgb8();
    
        // 将图像从 RGB 转换为 BGR，并存储为 Vec<f32>
        let mut img_vec: Vec<f32> = img
            .pixels()
            .flat_map(|p| {
                let b = p[0] as f32;
                let g = p[1] as f32;
                let r = p[2] as f32;
                vec![b, g, r]
            })
            .collect();
    
        // 填充图片
        let max_side = w.max(h);
        let padding = vec![0.0; (max_side * max_side * 3 - w * h * 3) as usize];
        img_vec.extend(padding);
    
        // 缩放图片
        let resize_size = 1024;
        let img_pad_resize: ImageBuffer<Rgb<f32>, Vec<f32>> = image::imageops::resize(
            &ImageBuffer::from_raw(max_side, max_side, img_vec).unwrap(),
            resize_size,
            resize_size,
            image::imageops::FilterType::Triangle,
        );
    
        // 将缩放后的图片数据转换为 Vec<f32>
        let mut img_pad_resize_vec: Vec<f32> = img_pad_resize
            .pixels()
            .flat_map(|p| vec![p[0], p[1], p[2]])
            .collect();
    
        // 归一化
        let mean = vec![123.68, 116.78, 103.94];
        for i in 0..img_pad_resize_vec.len() {
            img_pad_resize_vec[i] -= mean[i % 3];
        }
    
        // 转换为 Array 类型
        let img_array = Array::from_shape_vec(
            (Dim::<[usize; 3]>::new([resize_size.try_into().unwrap(), resize_size.try_into().unwrap(), 3])),
            img_pad_resize_vec,
        )?;
        Ok(img_array.insert_axis(Axis(0)).into_dyn())
    }

    // 前向传播
    pub fn forward(&self, input: ArrayD<f32>) -> Result<std::collections::HashMap<String, ArrayD<f32>>> {
        // 获取输入和输出名称
        // 输入: name: input_images, tensor: float32[1,3,1024,1024]
        let input_name = "input_images";
        // 将输入的 ArrayD<f32> 转换为 Value<f32>
        let input_value = input.into_raw_vec_and_offset().0;
        // 输出名称
        let output_names = vec![
            "dete_0/conv_cls/BiasAdd:0",
            "dete_0/conv_lnk/BiasAdd:0",
            "dete_0/conv_reg/BiasAdd:0",
            "dete_1/conv_cls/BiasAdd:0",
            "dete_1/conv_lnk/BiasAdd:0",
            "dete_1/conv_reg/BiasAdd:0",
            "dete_2/conv_cls/BiasAdd:0",
            "dete_2/conv_lnk/BiasAdd:0",
            "dete_2/conv_reg/BiasAdd:0",
            "dete_3/conv_cls/BiasAdd:0",
            "dete_3/conv_lnk/BiasAdd:0",
            "dete_3/conv_reg/BiasAdd:0",
            "dete_4/conv_cls/BiasAdd:0",
            "dete_4/conv_lnk/BiasAdd:0",
            "dete_4/conv_reg/BiasAdd:0",
            "dete_5/conv_cls/BiasAdd:0",
            "dete_5/conv_lnk/BiasAdd:0",
            "dete_5/conv_reg/BiasAdd:0",
        ];

        // 运行模型
        let mut inputs = std::collections::HashMap::new();
        inputs.insert(input_name, input_value);
        let outputs: Vec<OrtOwnedTensor<f32, _>> = self.sess.run(inputs)?;
        let mut all_maps = Vec::new();
        for i in (0..outputs.len()).step_by(3) {
            let cls_maps = outputs[i].view().to_owned();
            let lnk_maps = outputs[i + 1].view().to_owned();
            let reg_maps = outputs[i + 2].view().to_owned();
            all_maps.push((cls_maps, lnk_maps, reg_maps));
        }

        // unimplemented!()

        // 解码
        let image_size = Array::from_shape_vec((2,), vec![1024, 1024])?;
        let Ok((segments, group_indices, segment_counts, _)) = decode_segments_links_python(
            &image_size,
            &all_maps,
            &[6., 11.84210526, 23.68421053, 45., 90., 150.],
        );
        let Ok((combined_rboxes, combined_counts)) = combine_segments_python(&segments, &group_indices, &segment_counts);
        let mut output = std::collections::HashMap::new();
        output.insert("combined_rboxes".to_string(), combined_rboxes);
        output.insert("combined_counts".to_string(), combined_counts as f32);
        Ok(output.into())
    }

//     // 图片后处理
//     pub fn postprocess(&self, inputs: std::collections::HashMap<String, ArrayD<f32>>) -> Result<ArrayD<f32>> {
//         let rboxes = inputs.get("combined_rboxes").unwrap();
//         let count = inputs.get("combined_counts").unwrap();
//         if count[0] == 0 || count[0] < rboxes.shape()[0] as f32 {
//             return Ok(ArrayD::<f32>::zeros((0, 8)));
//         }
//         let rboxes = rboxes.slice(s![..count[0] as usize, ..]).to_owned();
//         let polygons = rboxes_to_polygons(&rboxes);
//         let orig_h = 1024;
//         let orig_w = 1024;
//         let resize_h = 1024;
//         let resize_w = 1024;
//         let scale_y = orig_h as f32 / resize_h as f32;
//         let scale_x = orig_w as f32 / resize_w as f32;
//         let polygons = polygons.map_axis(Axis(1), |row| {
//             let mut row = row.to_vec();
//             for i in 0..row.len() {
//                 if i % 2 == 0 {
//                     row[i] = row[i] * scale_x;
//                 } else {
//                     row[i] = row[i] * scale_y;
//                 }
//             }
//             Array::from(row)
//         });
//         let polygons = polygons.mapv(|x| x.round() as i32);
//         let dt_n9 = polygons
//             .outer_iter()
//             .map(|row| {
//                 let mut row = row.to_vec();
//                 row.push(cal_width(&row));
//                 row
//             })
//             .collect::<Vec<_>>();
//         let dt_nms = nms_python(&dt_n9);
//         let dt_polygons = dt_nms
//             .iter()
//             .map(|row| row[..8].to_vec())
//             .collect::<Vec<_>>();
//         Ok(Array::from_shape_vec((dt_polygons.len(), 8), dt_polygons.concat())?)
//     }

//     // 运行
//     pub fn run(&self, input: &str) -> Result<ArrayD<f32>> {
//         let input = self.preprocess(input)?;
//         let output = self.forward(input)?;
//         let result = self.postprocess(output)?;
//         Ok(result)
//     }

}