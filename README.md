# (WIP)基于MCP(Model Context Protocol)和VLM(Vision Language Models)的室内探索式建图框架

## 使用说明


## 模型说明
### 模型选用
**如下但单独列出各个文件的下载地址, 如果嫌麻烦可下载整合包.模型文件都很大, 因而整合包在百度网盘[]或魔搭社区[].**
```markdown
1. 图像分割模型
    * 原始→SAM2.1[@ref](https://github.com/facebookresearch/sam2)
    * 转换→ailia-models/segment-anything-2[@ref](https://github.com/axinc-ai/ailia-models/blob/master/image_segmentation/segment-anything-2)
    * 用途→图像分割, 图像进一步语义化.
    * 下载地址→
        - `image_encoder_hiera_t_2.1.onnx.prototxt` (https://storage.googleapis.com/ailia-models/segment-anything-2.1/image_encoder_hiera_t_2.1.onnx.prototxt)  
        - `mask_decoder_hiera_t_2.1.onnx.prototxt` (https://storage.googleapis.com/ailia-models/segment-anything-2.1/mask_decoder_hiera_t_2.1.onnx.prototxt)  
        - `prompt_encoder_hiera_t_2.1.onnx.prototxt` (https://storage.googleapis.com/ailia-models/segment-anything-2.1/prompt_encoder_hiera_t_2.1.onnx.prototxt)  
        - `memory_attention_hiera_t_2.1.opt.onnx.prototxt` (https://storage.googleapis.com/ailia-models/segment-anything-2.1/memory_attention_hiera_t_2.1.opt.onnx.prototxt)  
        - `memory_encoder_hiera_t_2.1.onnx.prototxt` (https://storage.googleapis.com/ailia-models/segment-anything-2.1/memory_encoder_hiera_t_2.1.onnx.prototxt)  
        - `mlp_hiera_t_2.1.onnx.prototxt` (https://storage.googleapis.com/ailia-models/segment-anything-2.1/mlp_hiera_t_2.1.onnx.prototxt)  
        - `obj_ptr_tpos_proj_hiera_t_2.1.onnx.prototxt` (https://storage.googleapis.com/ailia-models/segment-anything-2.1/obj_ptr_tpos_proj_hiera_t_2.1.onnx.prototxt)  

2. 光流估计模型
    * 原始→RAFT[@ref](https://github.com/princeton-vl/RAFT)
    * 转换→ailia-models/raft[@ref](https://github.com/axinc-ai/ailia-models/blob/master/optical_flow_estimation/raft)
    * 用途→估计相对位移.
    * 下载地址→
        [raft-things_fnet.onnx.prototxt](https://storage.googleapis.com/ailia-models/raft/raft-things_fnet.onnx.prototxt)<br/>
        [raft-things_cnet.onnx.prototxt](https://storage.googleapis.com/ailia-models/raft/raft-things_cnet.onnx.prototxt)<br/>
        [raft-things_update_block.onnx.prototxt](https://storage.googleapis.com/ailia-models/raft/raft-things_update_block.onnx.prototxt)<br/>
        [raft-small_fnet.onnx.prototxt](https://storage.googleapis.com/ailia-models/raft/raft-small_fnet.onnx.prototxt)<br/>
        [raft-small_cnet.onnx.prototxt](https://storage.googleapis.com/ailia-models/raft/raft-small_cnet.onnx.prototxt)<br/>
        [raft-small_update_block.onnx.prototxt](https://storage.googleapis.com/ailia-models/raft/raft-small_update_block.onnx.prototxt)<br/>

3. 文字识别模型
    * 原始→PaddleOCR[@ref](https://github.com/PaddlePaddle/PaddleOCR)
    * 转换→ailia-models/paddleocr[@ref](https://github.com/axinc-ai/ailia-models/blob/master/text_recognition/paddleocr)
    * 用途→识别目标字母, 例如`A`, `B`, `C`等.
    * 下载地址→
        [ch_ppocr_server_v2.0_det_train.onnx.prototxt](https://storage.googleapis.com/ailia-models/paddle_ocr/ch_ppocr_server_v2.0_det_train.onnx.prototxt)
        [ch_ppocr_mobile_v2.0_cls_train.onnx.prototxt](https://storage.googleapis.com/ailia-models/paddle_ocr/ch_ppocr_mobile_v2.0_cls_train.onnx.prototxt)
        [japan_mobile_v2.0_rec_infer.onnx.prototxt](https://storage.googleapis.com/ailia-models/paddle_ocr/japan_mobile_v2.0_rec_infer.onnx.prototxt)

4. 目标检测模型
    * 原始→CRAFT[@ref](https://github.com/clovaai/CRAFT-pytorch)
    * 转换→ailia-models/craft_pytorch[@ref](https://github.com/axinc-ai/ailia-models/blob/master/text_detection/craft_pytorch)
    * 用途→检测停机坪`H`标志.
    * 下载地址→
        [craft.onnx.prototxt](https://storage.googleapis.com/ailia-models/craft-pytorch/craft.onnx.prototxt)

5. 深度估计模型
    * 原始→DepthAnything[@ref](https://github.com/LiheYoung/Depth-Anything)
    * 转换→ailia-models/depth_anything[@ref](https://github.com/axinc-ai/ailia-models/blob/master/depth_estimation/depth_anything)
    * 用途→估计相对距离.
    * 下载地址→
        [depth_anything_v2_vits_indoor_dynamic.onnx](https://github.com/fabio-sim/Depth-Anything-ONNX/releases/download/v2.0.0/depth_anything_v2_vits_indoor_dynamic.onnx)

6. 旋转估计模型
    * 原始→rotnet[@ref](https://github.com/d4nst/RotNet)
    * 转换→ailia-models/rotnet[@ref](https://github.com/axinc-ai/ailia-models/blob/master/rotation_prediction/rotnet)
    * 用途→估计机身旋转, 补偿到tf坐标转换imu数据中.
    * 下载地址→
        [rotnet_gsv_2.onnx.prototxt](https://storage.googleapis.com/ailia-models/rotnet/rotnet_gsv_2.onnx.prototxt)

7. 三维检测模型
    * 原始→EgoNet[@ref](https://github.com/Nicholasli1995/EgoNet)
    * 转换→ailia-models/egonet[@ref](https://github.com/axinc-ai/ailia-models/blob/master/object_detection_3d/egonet)
    * 用途→主动避障物体碰撞箱检测
    * 下载地址→
        [HC.onnx.prototxt](https://storage.googleapis.com/ailia-models/egonet/HC.onnx.prototxt)  
        [L.onnx.prototxt](https://storage.googleapis.com/ailia-models/egonet/L.onnx.prototxt)

8. 低照度图像增强模型
    * 原始→DRBN SKF[@ref](https://github.com/langmanbusi/Semantic-Aware-Low-Light-Image-Enhancement/tree/main/DRBN_SKF)
    * 转换→ailia-models/drbn_skf[@ref](https://github.com/axinc-ai/ailia-models/blob/master/low_light_image_enhancement/drbn_skf)
    * 用途→增强低照度条件下的图像, 例如夜晚场景或者室内光照不足场景.
    * 下载地址→
        [drbn_skf_lol_v2.onnx.prototxt](https://storage.googleapis.com/ailia-models/drbn_skf/drbn_slf_lol_v2.onnx.prototxt)
        [model_lol.pt](https://drive.google.com/file/d/15djSbeDZd3NY5V-6XlRb-rk6_ljr-zLf/view)
        [model_lol_v2.pt](https://drive.google.com/file/d/1kO0Da29sCFF6vXwo7B_QuvrZvC31g0Ra/view?usp=sharing)
        
9. 图像分辨率增强模型
    * 原始→SPAN[@ref](https://github.com/hongyuanyu/SPAN)
    * 转换→ailia-models/span[@ref](https://github.com/axinc-ai/ailia-models/blob/master/super_resolution/span)
    * 用途→增强图像分辨率, 低成本摄像头也可以拥有高像素.
    * 下载地址→
        [spanx2_ch48.onnx.prototxt](https://storage.googleapis.com/ailia-models/span/spanx2_ch48.onnx.prototxt)

10. 物体线条检测模型
    * 原始→M-LSD[@ref](https://github.com/navervision/mlsd)
    * 转换→ailia-models/mlsd[@ref](https://github.com/axinc-ai/ailia-models/blob/master/line_segment_detection/mlsd)
    * 用途→检测物体边界线条, 用于主动避障.
    * 下载地址→
        [M-LSD_512_large.opt.onnx.prototxt](https://storage.googleapis.com/ailia-models/mlsd/M-LSD_512_large.opt.onnx.prototxt)

11. 图像关键点检测模型
    * 原始→
    * 转换→
    * 用途→匹配图像关键点, 用于建图时回环检测及估计相对位移.
    * 下载地址→

12. 思考大语言模型
    * 原始→deepseek-r1
    * 转换→
    * 特性→函数调用(Function Calling)
    * 用途→
    * 下载地址→

13. 视觉大语言模型
    * 原始→LLaVA-CoT
    * 转换→
    * 用途→
    * 下载地址→

```
- 推荐下载命令:
```sh
# 断点续传
aria2 -c "https://storage.googleapis.com/ailia-models/segment-anything-2.1/image_encoder_hiera_t_2.1.onnx.prototxt"
wget -c "https://storage.googleapis.com/ailia-models/segment-anything-2.1/image_encoder_hiera_t_2.1.onnx.prototxt"
```

### onnx算子支持列表
1. `wonnx`onnx运行时支持的算子[wonnx支持的算子op](docs/wonnx支持的算子op.md)
2. `onnx` Opset 21定义的算子[onnx_opset21_算子](docs/onnx_opset21_算子.md)

## 开发说明
### 顶层目录说明
```sh
- .cargo文件夹 : 编译器配置
- assets文件夹 : 资源文件
- docs文件夹 : 文档
- examples文件夹 : 示例代码
- files文件夹 : 其他资源文件
- result文件夹 : 代码运行结果
- src文件夹 : 源码目录
- static文件夹 : 封印为静态文件的库
- target文件夹 : 编译产物
- tests文件夹 : 单元测试
- vendor文件夹 : 所有依赖库
- crates文件夹 : 子项目
- `问题记录`文件夹 : 问题问答记录
* build.rs : 编译前预处理
* Cargo.toml : 项目配置
* clippy.toml : 代码风格规范
```

### 功能规划
* [ ]加载onnx.prototxt模型文件
* [ ]支持onnx Opset 18
* [ ]支持onnx Opset 13

### 通信模式
1. MCP协议
    >MCP 遵循客户端-服务器架构（client-server），其中包含以下几个核心概念：
    >MCP 主机（MCP Hosts）：发起请求的 LLM 应用程序（例如 Claude Desktop、IDE 或 AI 工具）。
    >MCP 客户端（MCP Clients）：在主机程序内部，与 MCP server 保持 1:1 的连接。
    >MCP 服务器（MCP Servers）：为 MCP client 提供上下文、工具和 prompt 信息。
    >本地资源（Local Resources）：本地计算机中可供 MCP server 安全访问的资源（例如文件、数据库）。
    >远程资源（Remote Resources）：MCP server 可以连接到的远程资源（例如通过 API）。

2. ROS2协议(ros2_humble)
    - **架构设计**：ROS2采用去中心化的架构，摒弃了ROS1中依赖单一主节点（ROS Master）的方式，不再需要中心节点来协调通信。
    - **通信机制**：
        - **DDS分布式通信**：
            - **核心概念**：基于数据分发服务（DDS）协议，DDS是一种面向分布式系统的协议，支持实时、可扩展和多平台的通信。其核心是一个以数据为中心的发布-订阅（Data-Centric Publish-Subscribe，DCPS）模型，该模型旨在为分布式异构平台上的进程间提供高效的数据传输。
            - **通信实体**：DCPS模型由DomainParticipant（域参与者）、Publisher（发布者）、Subscriber（订阅者）、DataWriter（数据写入器）、DataReader（数据读取器）和Topic（主题）组成。各进程之间的数据传输是根据服务质量（QoS）策略执行的。
            - **RTPS协议**：DataWriter和DataReader之间的数据传输通过RTPS协议（DDS标准协议）进行，该协议允许来自多个供应商的DDS实现通过抽象和优化传输（如TCP/UDP/IP）进行互操作。
            - **QoS配置**：支持QoS配置，开发者可根据需求调整传输策略，如保证数据必达、容忍延迟等。
        - **进程内通信**：在同一个进程内的节点之间，ROS2采用进程内通信机制，这种通信方式不依赖DDS，而是通过直接传递指针等方式实现更高效的数据传输。
    - **功能特性**：
        - **实时性支持**：支持实时性，适合实时系统。
        - **节点生命周期管理**：引入了节点生命周期管理。
        - **多机支持**：原生支持分布式、跨机器通信。
        - **消息序列化**：使用DDS高效的序列化方式。
        - **安全性增强**：更加注重安全性，使用DDS安全扩展进行加密、访问控制和身份验证。
    - **通信范式**：
        - **主题**：允许在发布-订阅模式下进行消息传递，一个节点将数据发布到一个命名的主题上，任意数量的节点可以订阅该主题以接收数据。通过QoS策略（可靠性、持久性、历史记录）增强，以满足定制的通信需求。
        - **服务**：允许进行请求-响应交互，如果节点A需要从节点B进行一次性计算或数据检索，它可以发送一个服务请求，节点B会进行回复。从DDS的改进中受益。
        - **操作**：引入用于基于目标的交互，允许发送、取消目标并获取反馈/结果，适用于长时间运行的任务，如导航。

### 源码说明
```sh

```

### 编译说明
```sh

```

## 主要参考文献
1. Model Context Protocol specification. (2025-03-26). https://spec.modelcontextprotocol.io/specification/2025-03-26/.
2. Zhang J, Huang J, Jin S, et al. Vision-language models for vision tasks: A survey[J]. IEEE Transactions on Pattern Analysis and Machine Intelligence, 2024.
