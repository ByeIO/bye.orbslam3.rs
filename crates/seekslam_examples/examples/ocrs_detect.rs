#![allow(unused)]

//! 使用ocrs库检测并识别字母

// 标准库
use std::collections::VecDeque;
use std::error::Error;
use std::path::PathBuf;
use std::fs::create_dir_all;

// 文字检测(仅限ASCII字符)
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};

// rten(onnx)运行时
use rten::Model;
use rten_tensor::prelude::*;
use rten_imageproc::{ BoundingRect, RotatedRect };

// 图像处理
use image::{ImageBuffer, Rgb, RgbImage};

// 错误处理
use anyhow::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载模型
    let detection_model_path = PathBuf::from("./assets/text-detection.rten");
    let rec_model_path = PathBuf::from("./assets/text-recognition.rten");

    let detection_model = rten::Model::load_file(detection_model_path)?;
    let recognition_model = rten::Model::load_file(rec_model_path)?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    // 读取并处理图像
    let img = image::open("./assets/scene1.png").map(|image| image.into_rgb8())?;

    // 应用此库期望的标准图像预处理操作（转换为灰度图像，并将范围映射到 [-0.5, 0.5]）。
    let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
    let ocr_input = engine.prepare_input(img_source)?;

    // 检测并识别文本
    let word_rects = engine.detect_words(&ocr_input)?;
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);
    let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

    // 打印结果
    for line in line_texts
        .iter()
        .flatten()
        .filter(|l| l.to_string().len() > 1)
    {
        println!("{}", line);
    }

    // 保存带有文本框的图片
    let mut result_img: RgbImage = ImageBuffer::new(img.width(), img.height());
    for (x, y, pixel) in img.enumerate_pixels() {
        result_img.put_pixel(x, y, *pixel);
    }

    for rect in word_rects {
        let bounding_rect = rect.bounding_rect();
        let x = bounding_rect.left() as u32;
        let y = bounding_rect.top() as u32;
        let w = bounding_rect.width() as u32;
        let h = bounding_rect.height() as u32;
        for i in x..(x + w) {
            result_img.put_pixel(i, y, Rgb([255, 0, 0]));
            result_img.put_pixel(i, y + h - 1, Rgb([255, 0, 0]));
        }
        for i in y..(y + h) {
            result_img.put_pixel(x, i, Rgb([255, 0, 0]));
            result_img.put_pixel(x + w - 1, i, Rgb([255, 0, 0]));
        }
    }

    create_dir_all("./result")?;
    result_img.save("./result/ocrs_detect.png")?;

    Ok(())
}