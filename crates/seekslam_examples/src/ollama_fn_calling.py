# 测试ollama和deepseek-r1-tool实现函数调用

import ollama
from ollama import Client
# 解决python兼容性问题
from typing_extensions import MutableMapping 
from dronekit import connect, VehicleMode, LocationGlobalRelative
from pymavlink import mavutil
import time
import json

# 视觉大模型给出的场景描述
PROMPT1 = '''
在图片中，总共有三个大写字母可见：两个白色方块上分别写着"C"和"A"，一个蓝色方块上写着"B"。这些单词被放置在背景的网格屏障上。

相对于无人机，位置如下：
- 空中的白边框无人机位于左侧。
- 右侧有两个支架上的蓝色方块："A"的支架比"B"更接近无人机。它们都高于地板，并且距离左边的白边框无人机大约两米。
- "CAB"单词直接对应于这些字母的位置，分别与每个相应的标志对齐。

这个设置表明了用于测试或训练目的的目的地识别系统，可能使用无人机来执行这种功能。
'''

# 控制无人机上下左右前后运动
def move(duration, forward=0, backward=0, left=0, right=0, up=0, down=0):
    """
    控制无人机上下左右前后运动

    Args:
        duration (int): 运动持续时间（秒）
        forward (float, optional): 向前速度（0~1）。 Defaults to 0.
        backward (float, optional): 向后速度（0~1）。 Defaults to 0.
        left (float, optional): 向左速度（0~1）。 Defaults to 0.
        right (float, optional): 向右速度（0~1）。 Defaults to 0.
        up (float, optional): 向上速度（0~1）。 Defaults to 0.
        down (float, optional): 向下速度（0~1）。 Defaults to 0.
    """
    # 设置运动速度
    velocity_x = forward - backward
    velocity_y = right - left
    velocity_z = down - up
    print("检测到move函数调用")

# 建立连接
client = Client(host='http://localhost:11434')

# 定义 log 函数
def log(message: str) -> None:
    """
    打印日志信息

    Args:
        message (str): 要打印的日志内容
    """
    print(f"Log: {message}")

# 调用模型并传入 log 函数作为工具
response = client.chat(
    model='MFDoom/deepseek-r1-tool-calling:1.5b',  # 指定模型
    messages=[{'role': 'user', 'content': '请记录一条日志：模型开始运行'}],  # 用户消息
    tools=[log],  # 将 log 函数作为工具传入
    stream=True  # 开启流式输出
)

# 收集完整响应和处理工具调用
full_response = []
tool_calls = []

print("已提交请求, 等待响应")

for chunk in response:
    if 'message' in chunk:
        message = chunk['message']
        if 'content' in message and message['content']:
            content = message['content']
            print(content, end='', flush=True)
            full_response.append(content)
        
        if 'tool_calls' in message:
            tool_calls.extend(message['tool_calls'])

# 处理工具调用
if tool_calls:
    for tool_call in tool_calls:
        func_name = tool_call['function']['name']
        args = tool_call['function']['arguments']
        print("args:", args)
        # 直接使用args字典，不需要再次json.loads
        if func_name == "log":
            print("\n检测到log函数调用!\n")
            log(**args)  # 直接解包字典参数

# 将完整结果写入日志文件
with open('../result/ollama_vision.log', 'a', encoding='utf-8') as log_file:
    log_file.write(''.join(full_response))
    log_file.write('\n')