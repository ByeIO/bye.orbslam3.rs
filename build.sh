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
# 深度预测
RUST_BACKTRACE=1 cargo-zigbuild run --example ort_depth > result/ort_depth.log
# 图像分割
python3 segment_onnx.py > ../result/segment_onnx.log
docker run -it --rm -v /home/qsbye/Documents/ByeIO/工程文件/exp209-bye_deepseek_mcp_discovery_slam_rs:/home seekslam_mnist_build bash
cd /home/crates/seekslam_examples
docker commit 90e204338917 seekslam_build:v1
docker commit 90e204338917 seekslam_build:v2
RUST_BACKTRACE=1 cargo-zigbuild build --example ort_segment >> ./result/ort_segment.log
RUST_BACKTRACE=1 cargo-zigbuild run --example ort_segment >> ./result/ort_segment.log
RUST_BACKTRACE=1 cargo-zigbuild run --example ort_segment
# 转换模型
TEMP="../../../assets/perceiver-io-optical-flow" python3 torch2onnx.py $TEMP/pytorch_model.bin $TEMP/model.onnx
python3 torch2onnx.py ../../../assets/perceiver-io-optical-flow/pytorch_model.bin ../../../assets/perceiver-io-optical-flow/model.onnx
pip install -U tf2onnx
pip install -U keras2onnx
pip install -U torch-onnx
uv init
uv add tensorflow
uv add keras2onnx
python3 -m tf2onnx.convert --saved-model ../../../assets/perceiver-io-optical-flow --output ../../../assets/perceiver-io-optical-flow/model.onnx
python3 torch2onnx.py ../../../assets/perceiver-io-optical-flow ../../../assets/perceiver-io-optical-flow/model.onnx
conda create -n perceiver
conda activate perceiver
conda config --set show_channel_urls yes
# 换回默认源
conda config --remove-key channels
conda config --show channels
conda install python=3.10
conda install pip
pip install -U perceiver-io
# 安装导出器
pip install onnx
pip install onnxscript
# 导出
python3 pth2onnx.py --model ./raft-things.pth --output_path ./raft-things.onnx >> pth2onnx.log
python3 pth2onnx.py --model ./raft-chairs.pth --output_path ./raft-chairs.onnx >> pth2onnx.log
python3 pth2onnx.py --model ./raft-kitti.pth --output_path ./raft-kitti.onnx >> pth2onnx.log
python3 pth2onnx.py --model ./raft-sintel.pth --output_path ./raft-sintel.onnx >> pth2onnx.log
python3 pth2onnx.py --model ./raft-small.pth --output_path ./raft-small.onnx >> pth2onnx.log
# 万能克隆
cargo run --example clone_trait
# 光流推理
python3 optics_onnx.py >> ../result/optics_onnx.log
RUST_BACKTRACE=1 cargo run --example ort_optics >> ./result/ort_optics.log
# 文字检测&识别
RUST_BACKTRACE=1 cargo run --example ocrs_detect >> ./result/ocrs_detect.log
python3 ocr_onnx.py >> ../result/ocr_onnx.log
RUST_BACKTRACE=1 cargo run --example ddddocr_detect >> ./result/ddddocr_detect.log
cargo run --example ddddocr_detect
# 运行llamafile并推理
chmod +x assets/DeepSeek-R1-Distill-Qwen-1.5B-Q4_K_M.llamafile
cat ./crates/seekslam_examples/docs/prompt.txt | ./assets/DeepSeek-R1-Distill-Qwen-1.5B-Q4_K_M.llamafile >> result/llamafile_test.log
chmod +x assets/Qwen2.5-0.5B-Instruct-Q4_K_M.llamafile
cat ./crates/seekslam_examples/docs/prompt.txt | assets/Qwen2.5-0.5B-Instruct-Q4_K_M.llamafile >> result/llamafile_test.log
RUST_BACKTRACE=1 cargo run --example llamafile_role >> ./result/llamafile_role.log
assets/Qwen2.5-0.5B-Instruct-Q4_K_M.llamafile
RUST_BACKTRACE=1 cargo run --example llamafile_role
# 测试minicpm视觉模型
uv add ollama
uv run ollama_vision.py
uv run ollama_fn_calling.py > ../result/ollama_fn_calling.log
uv run ollama_decision.py >> ../result/ollama_decision.log
# 测试rospypi软件源
uv init 
cd ./crates/seekslam_examples/src/rospypi/wheels
python3 -m http.server 80
uv pip install -i http://127.0.0.1:80 rospy-all
uv add --index-url https://pypi.org/simple --default-index http://127.0.0.1:80 rospy-all --frozen
uv sync --index-strategy unsafe-best-match
# 测试ros通信
uv run rospy_echo.py
# 测试起降(键盘控制??)
python3 rospy_xtdrone_demo.py iris 1
# ①输入t可以让飞机解锁起飞。
# ②输入r可以让飞机回到起飞点。
# ③输入b进入姿态控制。
# ⑤输入l飞机自主降落。
# ⑦输入q退出程序。
# 部署ocr识别服务
scp qsbye@192.168.100.187:/lvm-group1/qsbye/docker_podman/biu_paddleocr-v2-amd64.tar /home/qsbye
podman load -i /home/qsbye/biu_paddleocr-v2-amd64.tar
podman run -p 7861:7860 -d localhost/biu_paddleocr:v2-amd64 python app_box.py
# 部署建图服务
scp qsbye@192.168.100.187:/lvm-group1/qsbye/docker_podman/biu_facebook_vggt-v2-amd64.tar /home/qsbye
podman load -i /home/qsbye/biu_facebook_vggt-v2-amd64.tar
podman run -p 7862:7860 -d biu_facebook_vggt:v2-amd64 python app_cpu.py
# 部署ollama服务
ollama pull minicpm-v:latest
ollama pull MFDoom/deepseek-r1-tool-calling:1.5b
sudo systemctl edit ollama.service
sudo systemctl daemon-reload
sudo systemctl restart ollama
sudo systemctl status ollama
OLLAMA_HOST=0.0.0.0
OLLAMA_ORIGINS=*
# 测试ocr服务
uv add gradio_client
uv run gradio_api_ocr.py >> ../result/gradio_api_ocr.log
# 测试建图服务
uv add selenium
uv run gradio_headless_mapping.py >> ../result/gradio_headless_mapping.log
podman exec c575ac745a80 /bin/bash -c "cat /home/user/app/app_cpu.py"
podman run -it -v .:/tmp/api -p 7862:7860 localhost/biu_facebook_vggt:v2-amd64 /bin/bash
uv run gradio_api_mapping.py >> ../result/gradio_api_mapping.log
# 测试gradio库
cargo run --example gradio_sd3 >> ./result/gradio_sd3.log
wget https://stabilityai-stable-diffusion-3-medium.hf.space/file=/tmp/gradio/06fcdf0e3455d2c8c709f52c7da593833f863a06/image.webp -O ./result/gradio_sd3.webp
cargo run --example gradio_bs >> ./result/gradio_bs.log
RUST_BACKTRACE=1 cargo run --example gradio_ocr >> ./result/gradio_ocr.log
# 同步文件
rsync -avz --partial --progress ./webots-rs qsbye@192.168.31.20:/home/qsbye
uv run jupyter notebook password
# qsbye
uv run jupyter notebook --ip="0.0.0.0"
uv run pytest dlqe.py -v
uv run pytest pid.py -v 