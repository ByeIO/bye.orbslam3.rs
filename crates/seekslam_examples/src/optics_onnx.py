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

# # 下载模型文件
# def download_model(url: str, path: str):
#     print(f"正在从 {url} 下载模型到 {path}")
#     r = requests.get(url, stream=True)
#     with open(path, 'wb') as f:
#         # 获取文件总大小
#         total_length = int(r.headers.get('content-length'))
#         # 分块下载并显示进度条
#         for chunk in tqdm(r.iter_content(chunk_size=1024 * 1024), total=total_length // (1024 * 1024),
#                           bar_format='{l_bar}{bar:10}'):
#             if chunk:
#                 f.write(chunk)
#                 f.flush()

# 检查模型文件是否存在，若不存在则下载
def check_model(model_path: str):
    if os.path.exists(model_path):
        print("模型文件存在!")
        return
    # 从路径中提取模型名称
    model_name = os.path.basename(model_path).split('.')[0]
    if model_name not in available_models:
        raise ValueError(f"无效的模型名称: {model_name}")
    # url = f"https://github.com/ibaiGorordo/ONNX-NeuFlowV2-Optical-Flow/releases/download/0.1.0/{model_name}.onnx"
    # download_model(url, model_path)
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


