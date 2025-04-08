# 优化onnx文件, 移除无用节点(例如-1大小的数组)

import onnx
from onnxsim import simplify

# 加载原模型
onnx_model = onnx.load("../../../assets/ailia-models/depth_anything/depth_anything_v2_vits.onnx")

# 验证模型
onnx.checker.check_model(onnx_model)

# 简化模型
model_simplified, check = simplify(onnx_model)

# 检查简化后的模型是否有效
assert check, "Simplified ONNX model could not be validated"

# 保存简化后的模型
onnx.save(model_simplified, '../../../assets/ailia-models/depth_anything/depth_anything_v2_vits_simplified.onnx')
