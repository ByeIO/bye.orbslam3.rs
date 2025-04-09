import * as ort from "npm:onnxruntime-node";

// 本地 ONNX 模型文件路径
const MODEL_PATH = "./assets/clip-image-vit-32-float32.onnx";

console.log("正在加载本地模型...");
// 创建会话时直接使用本地文件路径
let onnxImageSession = await ort.InferenceSession.create(MODEL_PATH);

// 现在可以像之前一样运行推理
for (let i = 0; i < 5; i++) {
  let data = new Float32Array(3 * 224 * 224).map(() => Math.random() - 0.5);
  const feeds = { 'input': new ort.Tensor('float32', data, [1, 3, 224, 224]) };
  let t = performance.now();
  console.log("开始推理...");
  const results = await onnxImageSession.run(feeds);
  console.log(`推理完成，耗时 ${performance.now() - t}ms`);
}
