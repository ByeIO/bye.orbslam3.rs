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
cargo run --example deno_onnx_llm > result/deno_onnx_llm.log
deno ./src/deno_onnx_node.js > result/deno_onnx_node.log
deno ./assets/onnxruntime-web/deno_onnx_llm.js > result/deno_onnx_llm.log
node ./assets/onnxruntime-web/deno_onnx_llm.js > result/deno_onnx_llm.log
# 获取算子列表
python get_onnx_ops.py > ../result/get_onnx_ops.log
cargo test
# 安装转换器
sudo apt install -y protobuf-compiler
# cargo install protoc_bin_vendored
cargo install protoc-gen-prost
# Ensure compiler version is 3+
protoc --version  
protoc --proto_path=assets/onnx_opset22 \
       --rust_out=result/ \
       --experimental_allow_proto3_optional \
       onnx.in.proto
echo 'export PATH=$PATH:/home/qsbye/.cargo/bin' >> ~/.bashrc
source ~/.bashrc
protoc --prost_out=result --proto_path=assets/onnx_opset22 onnx.in.proto
# 安装编译依赖
sudo apt install -y sqlite3 libsqlite3-dev
cargo install cargo-zigbuild
rustup target add x86_64-unknown-linux-musl
sudo apt install -y clang
sudo systemctl daemon-reload
conda install -c conda-forge gcc
g++ --version
docker pull messense/cargo-zigbuild:sha-58f09d7
# 重新编译
docker run -it --rm -v /home/qsbye/Documents/ByeIO/工程文件/exp209-bye_deepseek_mcp_discovery_slam_rs:/home messense/cargo-zigbuild:sha-58f09d7
apt update && apt install -y cmake clang
docker commit 5096c723e7fc seekslam_mnist_build
docker run -it --rm -v /home/qsbye/Documents/ByeIO/工程文件/exp209-bye_deepseek_mcp_discovery_slam_rs:/home seekslam_mnist_build
cd /home/crates/seekslam_mnist
# cargo-zigbuild test --target x86_64-unknown-linux-musl
RUST_BACKTRACE=1 cargo-zigbuild zigbuild 
cargo-zigbuild check
# 打包js文件
bun install webpack-cli -D
bun --bunx webpack init
bun install -D file-loader url-loader
bun build index.ts --outdir .
bun --bunx webpack build
deno compile --include assets --include assets/3.jpg index.ts
docker run -it --rm -v /home/qsbye/Documents/ByeIO/工程文件/exp209-bye_deepseek_mcp_discovery_slam_rs:/home seekslam_mnist_build bash -c "cd /home/crates/seekslam_mnist && cargo-zigbuild run"
curl -fsSL https://deno.land/install.sh | sh
source ~/.bashrc
# deno 2.2.8
deno compile --include assets --allow-all index.ts
deno compile --include assets --allow-all --target aarch64-unknown-linux-gnu --output seekslam_mnist.linux.aarch64 index.ts
deno compile--include assets --allow-all --target aarch64-apple-darwin index.ts
# 使用webpack的url-loader方式打包
npm install onnxruntime-web jpeg-js webpack webpack-cli url-loader wasm-loader babel-loader @babel/core @babel/preset-env --save
npx webpack
node dist/bundle.js
# 使用deno静态打包
deno compile --allow-all --include=./assets --output mnist_app src/main.mjs
# 测试
alias car='docker run -it --rm -v /home/qsbye/Documents/ByeIO/工程文件/exp209-bye_deepseek_mcp_discovery_slam_rs:/home seekslam_mnist_build bash -c "cd /home/crates/seekslam_examples && $@"'
docker run -it --rm -v /home/qsbye/Documents/ByeIO/工程文件/exp209-bye_deepseek_mcp_discovery_slam_rs:/home seekslam_mnist_build bash
cargo run --example ort_onnx_info > result/ort_onnx_info.log
# 转换算子集
python3 src/convert_mnist_onnx_opset.py
RUST_BACKTRACE=1 cargo-zigbuild run --example ort_mnist > result/ort_mnist.log