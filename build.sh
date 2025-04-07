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
