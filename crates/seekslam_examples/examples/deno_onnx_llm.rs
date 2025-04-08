#![allow(unused)]

//! 这个示例展示了如何在 Rust 中使用 Deno 运行 JavaScript 程序，并加载本地文件。

use deno_cli::deno_core;
use deno_cli::serde_json;
use deno_cli::deno_error;

use deno_cli::deno_core::*;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::v8;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};
use tempfile::NamedTempFile;

fn main() {
    // 创建一个 Deno 运行时
    let mut runtime = JsRuntime::new(RuntimeOptions::default());

    // 定义要运行的 JavaScript 代码
    let code = r#"
        // 加载本地文件 ./assets/onnxruntime-web/ort.all.mjs
        await import("./assets/onnxruntime-web/ort.all.mjs");
        // 设置 ONNX Runtime 的环境变量
        ort.env.wasm.wasmPaths = "./assets/onnxruntime-web/";
        ort.env.wasm.numThreads = 1;
        console.log("Loading model...");
        let executionProviders = ["wasm"];
        // 加载本地文件 ./assets/clip-image-vit-32-float32.onnx
        let onnxImageSession = await ort.InferenceSession.create("./assets/clip-image-vit-32-float32.onnx", { executionProviders });
        for(let i = 0; i < 5; i++) {
            let data = new Float32Array(3*224*224).map(n => Math.random()-0.5);
            const feeds = {'input': new ort.Tensor('float32', data, [1,3,224,224])};
            let t = performance.now();
            console.log("Starting inference...");
            const results = await onnxImageSession.run(feeds);
            console.log(`Finished inference in ${performance.now()-t}ms`);
        }
    "#;

    // 使用 tempfile 创建临时文件
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(code.as_bytes()).expect("Failed to write temp file");

    // 获取临时文件的路径
    let temp_file_path = temp_file.path().to_str().expect("Failed to get temp file path");

    // 运行 JavaScript 代码
    let output: Value = eval(&mut runtime, temp_file_path).expect("Eval failed");

    println!("Output: {output:?}");
}

fn eval(
    context: &mut JsRuntime,
    code_path: &str,
) -> Result<Value, String> {
    // 读取 JavaScript 文件内容
    let mut file = File::open(code_path).map_err(|e| format!("Failed to open file: {e:?}"))?;
    let mut code = String::new();
    file.read_to_string(&mut code).map_err(|e| format!("Failed to read file: {e:?}"))?;

    // 在 Deno 运行时中执行 JavaScript 代码
    let res = context.execute_script("<anon>", code);
    match res {
        Ok(global) => {
            let scope = &mut context.handle_scope();
            let local = v8::Local::new(scope, global);
            // 将 V8 对象反序列化为 Rust 类型
            let deserialized_value = serde_v8::from_v8::<Value>(scope, local);

            match deserialized_value {
                Ok(value) => Ok(value),
                Err(err) => Err(format!("Cannot deserialize value: {err:?}")),
            }
        }
        Err(err) => Err(format!("Evaling error: {err:?}")),
    }
}
