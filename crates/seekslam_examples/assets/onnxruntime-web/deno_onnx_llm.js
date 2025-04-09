// 加载本地文件 ./assets/onnxruntime-web/ort.all.mjs
// await import("./assets/onnxruntime-web/ort.all.mjs");

import * as ort from "./ort.all.mjs";

// import * as ort from "npm:onnxruntime-node";


// 设置 ONNX Runtime 的环境变量
ort.env.wasm.wasmPaths = "./";
ort.env.wasm.numThreads = 1;
console.log("Loading model...");
let executionProviders = ["wasm"];
// 加载本地文件 ./assets/clip-image-vit-32-float32.onnx
// let onnxImageSession = await ort.InferenceSession.create("../assets/clip-image-vit-32-float32.onnx", { executionProviders });
let onnxImageSession = await ort.InferenceSession.create("file:/Users/workspace/Desktop/projects/毕业设计/工程文件/exp209-bye_deepseek_mcp_discovery_slam_rs/crates/seekslam_examples/assets/clip-image-vit-32-float32.onnx");

for(let i = 0; i < 5; i++) {
    let data = new Float32Array(3*224*224).map(n => Math.random()-0.5);
    const feeds = {'input': new ort.Tensor('float32', data, [1,3,224,224])};
    let t = performance.now();
    console.log("Starting inference...");
    const results = await onnxImageSession.run(feeds);
    console.log(`Finished inference in ${performance.now()-t}ms`);
}
