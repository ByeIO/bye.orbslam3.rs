# 编译命令
cargo-zigbuild run --example mcp_server_handle_message
cargo-zigbuild run --example mcp_client_handle_message
# 测试
cargo run --example read_prototxt
cargo run --example mnist_onnx
# 安装cli工具
cargo install --git https://github.com/webonnx/wonnx.git wonnx-cli
nnx info ./assets/wonnx_data/models/opt-squeeze.onnx > result/nnx_cli.log
nnx infer ./assets/wonnx_data/models/opt-squeeze.onnx -i data=./assets/wonnx_data/images/pelican.jpeg --labels ./assets/wonnx_data/models/squeeze-labels.txt --top 3 >> result/nnx_cli.log
nnx info ./assets/mnist/mnist.onnx >> result/nnx_cli.log
# 测试
cargo run --example mnist_onnx > result/mnist_onnx.log
cargo run --example wasmtime_cli_version
# 下载onnxruntime_web
bun init
bun install -D onnxruntime-web
bun install -D onnxruntime-node
# 测试wasmtime运行onnxruntime
wasmtime run ./assets/microsoft_onnxruntime_wasi.wasm -- python3 -c "import onnxruntime;print('ok')"
wasmtime ./assets/c2w-net-proxy.wasm --invoke ./assets/microsoft_onnxruntime_wasi.wasm --net=socket python3
c2w-net --invoke ./assets/microsoft_onnxruntime_wasi.wasm --net=socket python3
wasmtime run ./assets/microsoft_onnxruntime_wasi.wasm -- python3 -c "import onnxruntime;print(onnxruntime.__version__)"
# 1.7.0
docker run -it --rm openvino/onnxruntime_ep_ubuntu20:2024.4.0
python3 -c "import onnxruntime;print(onnxruntime.__version__)"
# python3 -c "import cv2;import numpy;"
wasmtime ./assets/openvino_onnxruntime_wasi.wasm python3 -c "import onnxruntime;print(onnxruntime.__version__)"
# 测试onnx模型
cargo run --example tract_mobilenet_onnx > result/tract_mobilenet_onnx.log
cargo run --example tract_depth_onnx > result/tract_depth_onnx.log
# 优化模型
onnxsim ./assets/ailia-models/depth_anything/depth_anything_v2_vits.onnx ./assets/ailia-models/depth_anything/depth_anything_v2_vits_simplified.onnx
# 测试deno
cargo run --example deno_hello > result/deno_hello.log
