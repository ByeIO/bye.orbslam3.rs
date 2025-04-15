#![allow(unused)]

//! 读光OCR_v2 onnx(ort)

// 1. 工具函数(seglink方法)
pub mod utils_seglink;

// 2. 工具函数
pub mod utils;

// 3. 识别
pub mod rec;

// 4. 可视化
pub mod visual;

// 5. 读光ocr主程序
pub mod dgocr;

// 6. 检测
pub mod det;

// 7. 检测(seglink方法)
pub mod det_seglink;