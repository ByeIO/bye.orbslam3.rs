# 测试gradio的深度图api并保存结果文件

#############################################
# gradio接口
from gradio_client import Client, handle_file
# 图像处理
from PIL import Image
# 内置库
import shutil
import os
#############################################

# 初始化Gradio客户端
client = Client("http://192.168.100.229:7863/")

# 调用API获取处理结果
result = client.predict(
    image=handle_file('../assets/scene1.png'),
    api_name="/on_submit"
)

# 收集所有文件路径（处理嵌套的列表结构）
file_paths = []
for item in result:
    if isinstance(item, list):
        file_paths.extend(item)  # 展开列表中的文件路径
    else:
        file_paths.append(item)  # 添加字符串类型的文件路径

# 创建输出目录（如果不存在）
output_dir = '../result'
os.makedirs(output_dir, exist_ok=True)

# 遍历处理每个文件路径
for idx, file_path in enumerate(file_paths):
    try:
        # 读取图像文件并转换为PNG格式
        img = Image.open(file_path)
        # 生成标准化的输出文件名（强制使用PNG格式）
        new_filename = f'gradio_api_depth_{idx}.png'
        new_path = os.path.join(output_dir, new_filename)
        # 保存转换后的文件
        img.save(new_path, 'PNG')
        print(f'文件已保存到：{new_path}')
    except Exception as e:
        print(f'处理文件 {file_path} 时出错：{str(e)}')

# 输出原始结果信息（调试用）
print("\n原始API返回路径：")
print(result)