#![allow(unused)]

//! 检测(seglink技术)

// 线性代数
use ndarray;

// 错误处理
use anyhow;

// 导入 ORT 相关模块
use ort::{inputs, session::Session};
use ort::value::Tensor;