#![allow(unused)]

//! 测试ddddocr识别

// ocr识别
use ddddocr::MapJson;

// 图像处理
use image;
use imageproc;

// 错误处理
use anyhow::{ Result, anyhow };

// 标准库
use std::fs;

fn main(){
    // 加载图片
    let image_bytes = std::fs::read("./assets/scene1.png").unwrap();
    // 加载ocr模型
    let mut det = ddddocr::ddddocr_detection().unwrap();
    // 目标检测文字(标框)
    let res = det.detection(&image_bytes).unwrap();
    println!("检测框: {:?}", res);
    // 文字识别
    let mut ocr = ddddocr::ddddocr_classification().unwrap();

    // 自定义字符集
    // ocr.set_ranges("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    ocr.set_ranges("ABC");
    // 数字3对应枚举 CharsetRange::LowercaseUppercase，不用写枚举
    // ocr.set_ranges(3);

    let res2 = ocr.classification(&image_bytes, false).unwrap();
    println!("识别文字: {:?}", res2);

    let mut result = ocr.classification_probability(&image_bytes, false).unwrap();
    // 哦呀，看来数据有点儿太多了，小心卡死哦！
    println!("概率: {}", result.json());
    println!("识别结果: {:?}", result.get_text());

}
