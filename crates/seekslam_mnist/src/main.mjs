import * as ort from 'onnxruntime-node';
import * as jpeg from 'https://deno.land/x/jpegts@1.1/mod.ts';

// 资源文件夹
const assetsFiles = Deno.readDirSync(import.meta.dirname + "/assets");

// 初始化运行时
// const wasmBase64 = await Deno.readTextFile(assetsFiles + '/ort-wasm-simd-threaded.wasm', { 
//     encoding: 'base64' 
//   });
  
//   ort.env.wasm.wasmPaths = {
//     'ort-wasm-simd-threaded.wasm': `data:application/wasm;base64,${wasmBase64}`
//   };

export default async function main() {
  const [modelBuffer, imageBuffer] = await Promise.all([
    Deno.readFile(import.meta.dirname + '/assets/mnist.onnx'),
    Deno.readFile(import.meta.dirname + '/assets/3.jpg')
  ]);

  const session = await ort.InferenceSession.create(modelBuffer);
  const tensor = processImage(imageBuffer);
  
  const results = await session.run({ input: tensor });
  console.log('推理结果:', Array.from(results.output.data));
}

// 图像处理实现（示例简化）
function processImage(buf) {
  const decoded = jpeg.decode(buf, { useTArray: true });
  return new ort.Tensor('float32', new Float32Array(decoded.data), [1, 1, 28, 28]);
}