# NeoFlowV2光流相对位移估计数学原理

## 简介
实时高精度光流估计对于各种实际应用场景至关重要。尽管最近基于学习的光流方法已经实现了高精度，但它们通常伴随着巨大的计算成本。在本文中，我们提出了一种高效的光流方法，能够在降低计算需求的同时保持高精度。在此基础上，我们引入了新的组件，包括一个更轻量级的主干网络和一个快速细化模块。这两个模块在保持计算需求较低的同时，提供了接近最先进的精度。与其他最先进的方法相比，我们的模型在保持合成数据和真实数据的可比性能的同时，实现了10倍到70倍的速度提升。它能够在Jetson Orin Nano上以超过20 FPS的速度运行512x384分辨率的图像。完整的训练和评估代码可在[https://github.com/neufieldrobotics/NeuFlow_v2](https://github.com/neufieldrobotics/NeuFlow_v2)上找到。

Real-time high-accuracy optical flow estimation is crucial for various real-world applications. While recent learning-based optical flow methods have achieved high accu- racy, they often come with significant computational costs. In this paper, we propose a highly efficient optical flow method that balances high accuracy with reduced computational demands. Building upon NeuFlow v1, we introduce new components including a much more light-weight backbone and a fast refinement module. Both these modules help in keeping the computational demands light while providing close to state of the art accuracy. Compares to other state of the art methods, our model achieves a 10x-70x speedup while maintaining comparable performance on both synthetic and real-world data. It is capable of running at over 20 FPS on 512x384 resolution images on a Jetson Orin Nano. The full training and evaluation code is available at [https://github.com/neufieldrobotics/NeuFlow_v2].

## 数学原理
以下是对论文中NeuFlow v2数学原理的总结：

### 一、简单主干网络（Simple Backbone）
NeuFlow v2使用一个简单的CNN主干网络来提取多尺度图像的低级特征。主干网络从1/2、1/4和1/8尺度的图像中提取特征，使用由卷积层、归一化层和ReLU激活层组成的CNN块来提取特征，并将这些特征合并和调整大小到所需的输出尺度，即1/16尺度的特征和上下文，以及1/8尺度的特征和上下文。具体来说，对于输入图像\(I\)，主干网络的输出可以表示为：
$$
\begin{aligned}
F_{1/8}, F_{1/16}, C_{1/8}, C_{1/16} &= \text{Backbone}(I)
\end{aligned}
$$
其中，\(F_{1/8}\)和\(F_{1/16}\)是用于相关性计算的特征，\(C_{1/8}\)和\(C_{1/16}\)是用于光流细化的上下文。

### 二、交叉注意力和全局匹配（Cross-Attention and Global Matching）
交叉注意力用于在全局范围内交换图像之间的信息，增强匹配特征的区分度，减少未匹配特征的相似度。全局匹配则用于在全局范围内寻找对应特征，使模型能够处理大像素位移，例如在快速移动的相机情况下。交叉注意力和全局匹配的操作可以表示为：
$$
\begin{aligned}
F'_{1/16} &= \text{CrossAttention}(F_{1/16}, F_{1/16}) \\
F''_{1/16} &= \text{GlobalMatching}(F'_{1/16})
\end{aligned}
$$
其中，\(F'_{1/16}\)是经过交叉注意力增强后的特征，\(F''_{1/16}\)是经过全局匹配后的特征。

### 三、简单RNN细化模块（Simple RNN Refinement）
首先计算附近9×9邻域内的相关性，并使用估计的光流对相关性进行扭曲。然后将扭曲的相关性、上下文特征、估计的光流和前一隐藏状态连接起来，通过八层简单的3×3卷积层后接ReLU激活函数来输出细化后的光流和更新后的隐藏状态。简单RNN细化模块的操作可以表示为：
$$
\begin{aligned}
C &= \text{ComputeCorrelation}(F''_{1/16}, F''_{1/16}) \\
C_w &= \text{Warp}(C, F_{\text{flow}}) \\
h_{t+1}, F_{\text{flow}}^{t+1} &= \text{SimpleRNN}(C_w, C_{1/16}, F_{\text{flow}}^t, h_t)
\end{aligned}
$$
其中，\(C\)是计算的相关性，\(C_w\)是扭曲后的相关性，\(h_t\)是前一隐藏状态，\(F_{\text{flow}}^t\)是当前估计的光流，\(h_{t+1}\)和\(F_{\text{flow}}^{t+1}\)分别是更新后的隐藏状态和细化后的光流。

### 四、多尺度特征/上下文合并（Multi-Scale Feature/Context Merge）
为了将全局特征/上下文与局部特征/上下文合并，确保1/8尺度的特征/上下文包含全局和局部信息，使用简单的CNN块来合并1/16尺度的全局特征/上下文与1/8尺度的局部特征/上下文。合并操作可以表示为：
$$
\begin{aligned}
F_{1/8}^{\text{merged}}, C_{1/8}^{\text{merged}} &= \text{Merge}(F_{1/8}, F_{1/16}^{\text{interpolated}}, C_{1/8}, C_{1/16}^{\text{interpolated}})
\end{aligned}
$$
其中，\(F_{1/8}^{\text{merged}}\)和\(C_{1/8}^{\text{merged}}\)是合并后的1/8尺度的特征和上下文。

### 五、整体流程
NeuFlow v2的整体流程可以总结为：
$$
\begin{aligned}
F_{1/8}, F_{1/16}, C_{1/8}, C_{1/16} &= \text{Backbone}(I) \\
F'_{1/16} &= \text{CrossAttention}(F_{1/16}, F_{1/16}) \\
F''_{1/16} &= \text{GlobalMatching}(F'_{1/16}) \\
C &= \text{ComputeCorrelation}(F''_{1/16}, F''_{1/16}) \\
C_w &= \text{Warp}(C, F_{\text{flow}}^0) \\
h_1, F_{\text{flow}}^1 &= \text{SimpleRNN}(C_w, C_{1/16}, F_{\text{flow}}^0, h_0) \\
F_{1/8}^{\text{merged}}, C_{1/8}^{\text{merged}} &= \text{Merge}(F_{1/8}, F_{1/16}^{\text{interpolated}}, C_{1/8}, C_{1/16}^{\text{interpolated}}) \\
\text{Repeat SimpleRNN refinement for 8 iterations on } F_{1/8}^{\text{merged}} \text{ and } C_{1/8}^{\text{merged}}
\end{aligned}
$$
最终，通过凸上采样模块将细化后的1/8尺度光流上采样到全分辨率，得到最终的光流估计结果。

## 对应代码
```py
# 使用optics模型预测光流
# 模型: ../../../assets/neuflow_v2/neoflow_things.onnx
# 输入:input1
# name: input1
# tensor: float32[1,3,432,768]
# input2
# name: input2
# tensor: float32[1,3,432,768]
# 输出:output
# name: output
# tensor: float32[1,2,432,768]

import cv2
import numpy as np
import onnxruntime
import os
import requests
from tqdm import tqdm

# 可用的模型列表
available_models = ["neuflow_mixed", "neuflow_sintel", "neuflow_things"]

# 检查模型文件是否存在，若不存在则下载
def check_model(model_path: str):
    if os.path.exists(model_path):
        print("模型文件存在!")
        return
    # 从路径中提取模型名称
    model_name = os.path.basename(model_path).split('.')[0]
    if model_name not in available_models:
        raise ValueError(f"无效的模型名称: {model_name}")
        exit(0)

# 绘制光流图
def draw_flow(flow, image, boxes=None):
    # 将光流转换为彩色图像
    flow_img = flow_to_image(flow, 35)
    # 从RGB颜色空间转换为BGR颜色空间
    flow_img = cv2.cvtColor(flow_img, cv2.COLOR_RGB2BGR)
    # 合并原始图像和光流图像
    combined = cv2.addWeighted(image, 0.5, flow_img, 0.6, 0)
    if boxes is not None:
        # 创建白色背景图像
        white_background = np.ones((image.shape[0], image.shape[1], 3), dtype=np.uint8) * 255
        # 合并原始图像和白色背景
        new_image = cv2.addWeighted(image, 0.7, white_background, 0.4, 0)
        for box in boxes:
            x1, y1, x2, y2 = box.astype(int)
            # 将合并后的图像部分替换到新图像中
            new_image[y1:y2, x1:x2] = combined[y1:y2, x1:x2]
        combined = new_image
    return combined

# 生成颜色轮
def make_color_wheel():
    """
    根据Middlebury颜色编码生成颜色轮
    :return: 颜色轮
    """
    RY = 15
    YG = 6
    GC = 4
    CB = 11
    BM = 13
    MR = 6
    ncols = RY + YG + GC + CB + BM + MR
    colorwheel = np.zeros([ncols, 3])
    col = 0
    # RY段
    colorwheel[0:RY, 0] = 255
    colorwheel[0:RY, 1] = np.transpose(np.floor(255 * np.arange(0, RY) / RY))
    col += RY
    # YG段
    colorwheel[col:col + YG, 0] = 255 - np.transpose(np.floor(255 * np.arange(0, YG) / YG))
    colorwheel[col:col + YG, 1] = 255
    col += YG
    # GC段
    colorwheel[col:col + GC, 1] = 255
    colorwheel[col:col + GC, 2] = np.transpose(np.floor(255 * np.arange(0, GC) / GC))
    col += GC
    # CB段
    colorwheel[col:col + CB, 1] = 255 - np.transpose(np.floor(255 * np.arange(0, CB) / CB))
    colorwheel[col:col + CB, 2] = 255
    col += CB
    # BM段
    colorwheel[col:col + BM, 2] = 255
    colorwheel[col:col + BM, 0] = np.transpose(np.floor(255 * np.arange(0, BM) / BM))
    col += BM
    # MR段
    colorwheel[col:col + MR, 2] = 255 - np.transpose(np.floor(255 * np.arange(0, MR) / MR))
    colorwheel[col:col + MR, 0] = 255
    return colorwheel

# 计算光流颜色图
colorwheel = make_color_wheel()
def compute_color(u, v):
    """
    计算光流颜色图
    :param u: 光流水平分量图
    :param v: 光流垂直分量图
    :return: 颜色编码的光流图
    """
    [h, w] = u.shape
    img = np.zeros([h, w, 3])
    # 找出NaN值的索引
    nanIdx = np.isnan(u) | np.isnan(v)
    u[nanIdx] = 0
    v[nanIdx] = 0
    ncols = np.size(colorwheel, 0)
    # 计算光流的幅值
    rad = np.sqrt(u ** 2 + v ** 2)
    # 计算光流的角度
    a = np.arctan2(-v, -u) / np.pi
    fk = (a + 1) / 2 * (ncols - 1) + 1
    k0 = np.floor(fk).astype(int)
    k1 = k0 + 1
    k1[k1 == ncols + 1] = 1
    f = fk - k0
    for i in range(0, np.size(colorwheel, 1)):
        tmp = colorwheel[:, i]
        col0 = tmp[k0 - 1] / 255
        col1 = tmp[k1 - 1] / 255
        col = (1 - f) * col0 + f * col1
        idx = rad <= 1
        col[idx] = 1 - rad[idx] * (1 - col[idx])
        notidx = np.logical_not(idx)
        col[notidx] *= 0.75
        img[:, :, i] = np.uint8(np.floor(255 * col * (1 - nanIdx)))
    return img

# 将光流转换为Middlebury颜色编码图像
def flow_to_image(flow, maxrad=None):
    """
    将光流转换为Middlebury颜色编码图像
    :param flow: 光流图
    :return: Middlebury颜色编码的光流图像
    """
    u = flow[:, :, 0]
    v = flow[:, :, 1]
    rad = np.sqrt(u ** 2 + v ** 2)
    if maxrad is None:
        maxrad = max(-1, np.max(rad))
    eps = np.finfo(float).eps
    u = np.clip(u, -maxrad + 5, maxrad - 5)
    v = np.clip(v, -maxrad + 5, maxrad - 5)
    u = u / (maxrad + eps)
    v = v / (maxrad + eps)
    img = compute_color(u, v)
    return np.uint8(img)

# 光流估计类
class NeuFlowV2:
    def __init__(self, path: str):
        # 检查模型文件
        check_model(path)
        # 初始化ONNX运行时会话
        self.session = onnxruntime.InferenceSession(path, providers=onnxruntime.get_available_providers())
        # 获取输入信息
        self.get_input_details()
        # 获取输出信息
        self.get_output_details()

    def __call__(self, img_prev: np.ndarray, img_now: np.ndarray) -> np.ndarray:
        return self.estimate_flow(img_prev, img_now)

    # 估计光流
    def estimate_flow(self, img_prev: np.ndarray, img_now: np.ndarray) -> np.ndarray:
        # 准备输入张量
        input_tensors = self.prepare_inputs(img_prev, img_now)
        # 进行推理
        outputs = self.inference(input_tensors)
        return self.process_output(outputs[0])

    # 准备输入张量
    def prepare_inputs(self, img_prev: np.ndarray, img_now: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
        self.img_height, self.img_width = img_now.shape[:2]
        input_prev = self.prepare_input(img_prev)
        input_now = self.prepare_input(img_now)
        return input_prev, input_now

    # 准备单张输入图像
    def prepare_input(self, img: np.ndarray) -> np.ndarray:
        # 调整图像大小
        input_img = cv2.resize(img, (self.input_width, self.input_height))
        # 归一化
        input_img = input_img / 255.0
        input_img = input_img.transpose(2, 0, 1)
        input_tensor = input_img[np.newaxis, :, :, :].astype(np.float32)
        return input_tensor

    # 进行推理
    def inference(self, input_tensors: tuple[np.ndarray, np.ndarray]) -> np.ndarray:
        start = time.perf_counter()
        outputs = self.session.run(self.output_names, {self.input_names[0]: input_tensors[0],
                                                       self.input_names[1]: input_tensors[1]})
        print(f"推理时间: {(time.perf_counter() - start) * 1000:.2f} ms")
        return outputs

    # 处理输出结果
    def process_output(self, output) -> np.ndarray:
        flow = output.squeeze().transpose(1, 2, 0)
        return cv2.resize(flow, (self.img_width, self.img_height))

    # 获取输入信息
    def get_input_details(self):
        model_inputs = self.session.get_inputs()
        self.input_names = [model_inputs[i].name for i in range(len(model_inputs))]
        input_shape = model_inputs[0].shape
        self.input_height = input_shape[2]
        self.input_width = input_shape[3]

    # 获取输出信息
    def get_output_details(self):
        model_outputs = self.session.get_outputs()
        self.output_names = [model_outputs[i].name for i in range(len(model_outputs))]

if __name__ == '__main__':
    import time
    # 模型文件路径
    model_path = "../../../assets/ailia-models/neuflow_v2/neuflow_sintel.onnx"
    print("模型文件路径{}", model_path)
    # 初始化模型
    estimator = NeuFlowV2(model_path)
    # 加载第一张图片
    img1 = cv2.imread("../assets/frame_0016.png")
    # 加载第二张图片
    img2 = cv2.imread("../assets/frame_0025.png")
    # 估计光流
    flow = estimator(img1, img2)
    # 绘制光流图
    flow_img = draw_flow(flow, img1)
    # 创建结果文件夹
    os.makedirs("../result", exist_ok=True)
    # 保存结果图片
    cv2.imwrite("../result/optics_onnx.png", flow_img)
    print("光流预测结果已保存到 ../result/optics_onnx.png")   
```

