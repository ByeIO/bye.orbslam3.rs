# 获取onnx所有算子列表
import onnx

def get_operators_from_onnx(file_path):
    try:
        # 加载 ONNX 模型
        model = onnx.load(file_path)
        operators = set()
        # 遍历图中的所有节点
        for node in model.graph.node:
            # 获取节点的算子类型
            operators.add(node.op_type)
        return operators
    except FileNotFoundError:
        print("错误: 文件未找到!")
    except Exception as e:
        print(f"错误: 发生了一个未知错误: {e}")
    return set()

if __name__ == "__main__":
    file_path = '../assets/mnist.onnx'
    operators = get_operators_from_onnx(file_path)
    if operators:
        print("ONNX 文件中的所有唯一算子列表:")
        for op in operators:
            print(op)
    