import argparse
import torch
import os
from pathlib import Path
import json

def load_torch_model(model_dir: str) -> torch.nn.Module:
    """
    从目录加载Torch模型（需包含.bin和.json文件）
    :param model_dir: 模型目录路径，包含模型权重（.bin）和配置（.json）
    :return: 加载后的PyTorch模型
    """
    model_path = Path(model_dir)
    # 假设.bin文件是模型权重，.json是模型配置（需根据实际情况调整）
    bin_files = list(model_path.glob("*.bin"))
    if not bin_files:
        raise FileNotFoundError("未找到.bin模型权重文件")
    
    # 加载模型配置（可选，根据实际需求）
    config = {}
    json_files = list(model_path.glob("*.json"))
    if json_files:
        with open(json_files[0], "r") as f:
            config = json.load(f)
    
    # 根据配置构建实际模型（需替换为实际模型类和构建逻辑）
    # 示例中使用占位模型（实际需替换）
    # model = torch.nn.Linear(10, 2)  # 替换为实际模型
    model = PerceiverIOOpticalFlow(**config)  # 假设PerceiverIOOpticalFlow是实际模型类
    model.load_state_dict(torch.load(bin_files[0]))
    return model.eval()  # 设置为评估模式

def convert_to_onnx(model: torch.nn.Module, output_path: str, dynamic_axes: dict = None):
    """
    将PyTorch模型导出为ONNX格式
    :param model: PyTorch模型
    :param output_path: 输出ONNX文件路径
    :param dynamic_axes: 动态轴配置（如可变batch_size）
    """
    # 生成虚拟输入（需根据模型实际输入调整）
    dummy_input = torch.randn(1, 10)  # 替换为模型实际输入尺寸
    
    # 导出ONNX模型
    torch.onnx.export(
        model,
        dummy_input,
        output_path,
        export_params=True,  # 包含模型权重
        opset_version=13,    # ONNX算子集版本
        do_constant_folding=True,  # 启用常量折叠优化
        input_names=["input"],     # 输入节点名称
        output_names=["output"],   # 输出节点名称
        dynamic_axes=dynamic_axes  # 动态轴配置（可选）
    )
    print(f"模型已成功导出到: {output_path}")

def main():
    # 解析命令行参数
    parser = argparse.ArgumentParser(description="PyTorch模型转ONNX工具")
    parser.add_argument("model_dir", help="包含.bin/.json的Torch模型目录路径")
    parser.add_argument("output_path", help="输出ONNX文件路径")
    parser.add_argument("--dynamic", action="store_true", help="启用动态batch_size")
    args = parser.parse_args()

    # 动态轴配置（若启用）
    dynamic_axes = None
    if args.dynamic:
        dynamic_axes = {
            "input": {0: "batch_size"},  # 第0维（batch）动态
            "output": {0: "batch_size"}
        }

    # 加载模型并转换
    model = load_torch_model(args.model_dir)
    convert_to_onnx(model, args.output_path, dynamic_axes)

if __name__ == "__main__":
    main()