// webpack.config.mjs
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default {
  mode: 'production',
  entry: ['./src/index.mjs'],
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
    module: true,
  },
  experiments: {
    outputModule: true
  },
  module: {
    rules: [
      {
        test: /\.(onnx|jpg|wasm)$/,
        type: 'asset/inline',
        generator: {
          dataUrl: content => {
            // 保持MIME类型为application/wasm
            return `data:application/wasm;base64,${content.toString('base64')}`;
          }
        }
      }
    ]
  },
  resolve: {
    fallback: { 
      os: false,
      "node:os": false 
    }
  },
  externals: {
    'jpeg-js': 'jpeg-js'
  }
};