import os
import numpy as np
from PIL import Image
import onnxruntime as ort

def main():
    # 定义图片路径和模型路径
    img_path = "../assets/rgb1.png"
    model_path = "../../../assets/ailia-models/depth_anything/depth_anything_v2_vits.onnx"

    # 加载图片并转换为 RGB 格式
    image = Image.open(img_path).convert("RGB")
    orig_w, orig_h = image.size

    # 调整图片大小到模型输入尺寸
    resized = image.resize((518, 518), Image.Resampling.BICUBIC)

    # 将图片转换为张量
    image_array = np.array(resized, dtype=np.float32) / 255.0
    image_tensor = np.transpose(image_array, (2, 0, 1))  # 转换为通道优先
    image_tensor = np.expand_dims(image_tensor, axis=0)  # 添加批次维度

    # 加载 ONNX 模型
    session = ort.InferenceSession(model_path)
    print("模型加载成功")

    # 获取输入和输出名称
    input_name = session.get_inputs()[0].name
    output_name = session.get_outputs()[0].name

    # 运行模型
    result = session.run([output_name], {input_name: image_tensor})
    depth = result[0]

    # 将深度图调整回原始尺寸
    depth = depth.squeeze()  # 去掉批次维度
    depth = (depth - depth.min()) / (depth.max() - depth.min()) * 255  # 归一化并缩放
    depth = depth.astype(np.uint8)
    depth_resized = Image.fromarray(depth).resize((orig_w, orig_h), Image.Resampling.BICUBIC)

    # 保存结果图片
    os.makedirs("../result", exist_ok=True)
    depth_resized.save("../result/tract_depth_onnx.png")
    print("深度图已保存到 ../result/tract_depth_onnx.png")

if __name__ == "__main__":
    main()
