#![allow(unused)]

//! onnx数据格式及算子数据格式定义文件
//! proto2/3数据交换格式编译为rust代码

// 1. onnx文件格式定义(onnx opset 22)
// 源文件: onnx.in.proto, 使用prost转换
pub mod onnx;

// // 2. 机器学习子模块
// pub mod onnx_ml;

// // 3. 数据子模块
// pub mod onnx_data;

// // 4. 机器学习算子集
// pub mod onnx_operators_ml;

// // 5. 通用算子集
// pub mod onnx_operators;
