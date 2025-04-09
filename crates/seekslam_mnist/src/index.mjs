// ESM语法
// import ort from '../assets/ort.all.mjs';
import ort from 'onnxruntime-web/all'
import jpeg from 'jpeg-js';
import modelData from '../assets/mnist.onnx?base64';
import imageData from '../assets/3.jpg';

// WASM路径设置, // 直接导入WASM二进制
// import wasmBase64 from '../assets/ort-wasm-simd-threaded.wasm?base64';

// 直接导入WASM文件（Webpack会将其转换为data URL）
import wasmUrl from '../assets/ort-wasm-simd-threaded.wasm';
// import ortBackendUrl from '../assets/ort-wasm-simd-threaded.mjs';

// 设置WASM路径
// ort.env.wasm.wasmPaths = {
//   'ort-wasm-simd-threaded.wasm': wasmUrl,
//   // 'ort-wasm-simd-threaded.mjs': ortBackendUrl
// };

// 初始化WASM运行时
// await ort.env.wasm.initWasm({
//   'ort-wasm-simd-threaded.mjs': ortBackendUrl,
//   // 'ort-wasm-simd-threaded.wasm': wasmUrl
// });

ort.env.wasm.numThreads = 1; // 禁用多线程
ort.env.wasm.simd = false;   // 禁用SIMD加速

export default async function main() {
  console.log("hello from mnist");
  const session = await ort.InferenceSession.create(modelData);
  
  // 图像处理逻辑
  const tensor = processImage(imageData);
  
  const results = await session.run({ input: tensor });
  console.log('推理结果:', Array.from(results.output.data));
}

function processImage(buf) {
  // 实现图像预处理逻辑
  return new ort.Tensor('float32', new Float32Array(784), [1,1,28,28]);
}

// 调用main
main().catch(e => console.error("全局错误:", e));