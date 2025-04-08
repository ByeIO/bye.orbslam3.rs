# 转换onnx 的 opset算子集
import onnx
from onnx import version_converter

# 加载原模型
onnx_model = onnx.load("../../../assets/ailia-models/depth_anything/depth_anything_v2_vits.onnx")

# 打印原始opset
for opset in onnx_model.opset_import:
    print(f"Domain: {opset.domain}, Version: {opset.version}")

# 转换为opset 11
converted_model = version_converter.convert_version(onnx_model, 15)

# 保存文件
onnx.save(converted_model, "../../../assets/ailia-models/depth_anything/depth_anything_v2_vits_opset15.onnx")
