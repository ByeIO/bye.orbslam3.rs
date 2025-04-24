#![allow(unused)]

//! 使用http_req库调用gradio库的ocr服务

// 网络请求库
use http_req::{
    request::RequestMessage,
    response::Response,
    stream::{self, Stream},
    uri::Uri,
};
use tokio::io::{self, AsyncWriteExt as _};

// 图片处理
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::{draw_polygon_mut, draw_text_mut};
use imageproc::point::Point;

// 字体渲染
use rusttype::{Font, Scale};
use ab_glyph::{FontArc, FontRef, PxScale};

// json解析
use serde_json::{Value, json};

// 错误处理
use anyhow::{ anyhow, Result };

// 标准库
use std::{
    convert::TryFrom,
    io::{BufReader, Read, Write},
    time::Duration,
    path::Path,
};

#[tokio::main]
async fn main() -> Result<()> {
    /* start 网络请求部分 */

    println!("准备连接服务器接口");

    // 准备请求数据
    let request_data = json!({
        "data": [
            {"path":"./assets/scene1.png","meta":{"_type":"gradio.FileData"}},
            "en"
        ]
    });

    // 创建URI
    let api_uri = Uri::try_from("http://192.168.31.20:7861/gradio_api/call/predict")?;
    let mut stream = Stream::connect(&api_uri, Some(Duration::from_secs(60)))?;

    // 准备请求消息
    let request_msg = RequestMessage::new(&api_uri)
        .header("Content-Type", "application/json")
        .header("Connection", "Close")
        .body(request_data.to_string().as_bytes())
        .parse();

    println!("发送识别请求");
    stream.write_all(&request_msg)?;

    // 读取响应
    let mut stream = BufReader::new(stream);
    let raw_head = stream::read_head(&mut stream);
    let mut body = Vec::new();
    stream.read_to_end(&mut body)?;

    // 解析响应
    let response = Response::from_head(&raw_head)?;
    if !response.status_code().is_success() {
        return Err(anyhow!("请求失败: {}", response.reason()));
    }

    let body_str = String::from_utf8(body)?;
    let event_id = body_str.split('"').nth(3).ok_or(anyhow!("解析EVENT_ID失败"))?;
    println!("获取到EVENT_ID: {}", event_id);

    // 获取结果
    let result_url = format!("http://192.168.31.20:7861/gradio_api/call/predict/{}", event_id);
    let result_uri = Uri::try_from(result_url.as_str())?;
    let mut result_stream = Stream::connect(&result_uri, Some(Duration::from_secs(60)))?;

    let result_request = RequestMessage::new(&result_uri)
        .header("Connection", "Close")
        .parse();

    result_stream.write_all(&result_request)?;

    let mut result_stream = BufReader::new(result_stream);
    let _ = stream::read_head(&mut result_stream);
    let mut result_body = Vec::new();
    result_stream.read_to_end(&mut result_body)?;

    let result_str = String::from_utf8(result_body)?;
    let output: Value = serde_json::from_str(&result_str)?;
    
    println!("提取API结果成功: {:?}", output);

    /* end 网络请求部分 */

    /* start 结果处理部分 */

    // 以下代码保持不变...
    // 提取识别结果数据
    let boxes = output["boxes"].as_array().ok_or("Invalid boxes");
    let texts = output["texts"].as_array().ok_or("Invalid texts");
    let scores = output["scores"].as_array().ok_or("Invalid scores");

    println!("提取数据成功");

    // 打开原始图片
    let mut original_image = image::open("./assets/scene1.png")?.to_rgba8();

    // 设置框和文字样式
    // 蓝色框
    let box_color = Rgba([0, 0, 255, 255]);    
    // 红色文字
    let text_color = Rgba([255, 0, 0, 255]);     
    let font_file = Vec::from(include_bytes!("../../../assets/SmileySans-Oblique.ttf") as &[u8]);
    let font = ab_glyph::FontArc::try_from_vec(font_file)?;
    // 字体大小  
    let scale = 20.0;        

    // 遍历所有识别结果
    for (i, ((box_, text), score)) in boxes.iter().zip(texts.iter()).zip(scores.iter()).enumerate() {
        println!("box:{:?}", box_);
        
        // let _box_ = box_.as_array().ok_or("Invalid box format");
        // let _text = text.as_str().ok_or("Invalid text format");
        // let _score = score.as_f64().ok_or("Invalid score format");

    //     // 转换坐标点
    //     let points: Vec<Point<i32>> = box_
    //         .iter()
    //         .map(|p| {
    //             let p = p.as_array().unwrap();
    //             Point::new(
    //                 p[0].as_f64().unwrap() as i32,
    //                 p[1].as_f64().unwrap() as i32
    //             )
    //         })
    //         .collect();

    //     // 绘制四边形框
    //     draw_polygon_mut(
    //         &mut original_image,
    //         &points,
    //         box_color,
    //     );

    //     // 在框的左上角添加识别文本和置信度
    //     let text_with_score = format!("{}({:.2})", text.unwrap(), score);
    //     draw_text_mut(
    //         &mut original_image,
    //         text_color,
    //         points[0].x as i32,
    //         points[0].y as i32 - 25,
    //         scale,
    //         &font,
    //         &text_with_score,
    //     );

    }// end for

    // println!("遍历结果成功");

    // // 创建结果目录（如果不存在）
    // std::fs::create_dir_all("./result")?;

    // // 保存处理后的图片
    // let output_path = "./result/gradio_ocr.png";
    // original_image.save(output_path)?;
    // println!("处理后的图片已保存到: {}", output_path);

    /* end 结果处理部分 */

    Ok(())
}