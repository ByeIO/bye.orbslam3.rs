# 问题
```py
# 获取camera数据并进行处理
# 输入: webots机器人实例上下文
# 输出: webots机器人的camera画面的base64编码数据
#        并且另外保存图片到目录

#############################################
# 第三方库
from gradio_client import Client, handle_file
from PIL import Image, ImageDraw
import cv2
import asyncio
# 内置库
import base64
import io
import os
import json
import requests
import logging
#############################################

# 配置日志
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class _Camera:
    def __init__(robot):
        self.robot = robot
    
    # 捕获图片并返回为base64数据
    def capture_image_and_return(self):
        _capture_image_and_save()
        # 返回base64数据
        # TODO
        img_base64 = ''
        return img_base64

    # 1Hz捕获图片并保存到目录
    def _capture_image_and_save(self, save_folder, image_counter):
        """
        捕获并保存当前相机图像
        :param save_folder: 图像保存目录
        :param image_counter: 图像计数器
        :return: 新的图像计数器
        """
        # 获取图像数据
        image_data = self.camera.getImage()
        if image_data:
            # 转换为OpenCV格式
            width = self.camera.getWidth()
            height = self.camera.getHeight()
            image = np.frombuffer(image_data, dtype=np.uint8).reshape((height, width, 4))
            image_bgr = cv2.cvtColor(image, cv2.COLOR_BGRA2BGR)
            
            # 保存图像
            filename = os.path.join(save_folder, f"img_{image_counter:04d}.jpg")
            cv2.imwrite(filename, image_bgr)
            image_counter += 1
        
        return image_counter
```
修正并完善代码, 添加中文注释;

# 问题
```py
def _update_history(self, question, answer):
        """更新对话历史"""
        self.chat_history.append({"q": question, "a": answer})
```
更新完成后打印chat_history的最后一条消息;

## 回答
要在更新对话历史后打印最后一条消息，可以修改代码如下：

```py
def _update_history(self, question, answer):
    """更新对话历史"""
    self.chat_history.append({"q": question, "a": answer})
    # 打印最后一条消息
    print(f"最新对话记录 - Q: {question}, A: {answer}")
```

或者如果你想打印整个最后一条记录对象：

```py
def _update_history(self, question, answer):
    """更新对话历史"""
    self.chat_history.append({"q": question, "a": answer})
    # 打印最后一条记录
    print("最新对话记录:", self.chat_history[-1])
```

两种方式都可以，第一种更直观地显示了问题和答案，第二种则直接打印出整个字典对象。

# 问题
```py
# 阶段2(在两个房间交界处悬停决策)提示词
_PROMPT2 = "你来到了两个房间的交界处, 悬停在1.5米高度, 先侦察一下第二个房间的状况. 刚刚使用深度估计算法获取了朝向面的深度图(通过颜色区分深度), 你需要判断深度信息以判断应该如何前进, 注意避开障碍物.刚刚使用OCR算法识别目标, 结果为{_RESULT_}, 你需要调用函数移动到目标或者前进以继续搜索目标."

# 对外接口, 预处理prompt
def preprecess_prompt(self, *args, **kwargs):
    '''
    接受任意类型和任意数量的参数, 转换为str并格式化到prompt中
    
    参数:
        *args: 位置参数列表
        **kwargs: 关键字参数字典
        
    返回:
        str: 格式化后的prompt字符串
    '''
    prompt_parts = []
    
    # 处理位置参数
    for i, arg in enumerate(args):
        if isinstance(arg, (list, dict)):
            # 对列表和字典使用json格式化
            prompt_parts.append(f"参数{i+1}:\n{json.dumps(arg, indent=2, ensure_ascii=False)}")
        else:
            # 其他类型直接转字符串
            prompt_parts.append(f"参数{i+1}: {str(arg)}")
    
    # 处理关键字参数
    for key, value in kwargs.items():
        if isinstance(value, (list, dict)):
            # 对列表和字典使用json格式化
            prompt_parts.append(f"{key}:\n{json.dumps(value, indent=2, ensure_ascii=False)}")
        else:
            # 其他类型直接转字符串
            prompt_parts.append(f"{key}: {str(value)}")
    
    # 将所有部分用换行符连接
    formatted_prompt = "\n".join(prompt_parts)
    
    # 更新当前prompt
    if hasattr(self, 'prompt'):
        self.prompt.append(formatted_prompt)
    else:
        self.prompt = [formatted_prompt]
        
    return formatted_prompt
```
修正代码, 以能够将输入参数格式化到`{_RESULT_}`位置形成新的prompt;

# 问题
示例代码:
```py
# 无人机视觉识别与自主决策控制系统
# 功能：通过视觉大模型识别场景，然后使用函数调用模型进行决策和控制

import ollama
from ollama import Client
from typing_extensions import MutableMapping 
import time
import json
import requests  # 用于直接调用API接口

# 系统常量定义
PROMPT_VISION = '图片里面有几个字母？描述字母相对无人机的位置关系, 无人机更靠近哪个字母?'  # 视觉识别提示词
PROMPT_DECISION = '攻击目标为B, 根据相对关系移动无人机到B前方'  # 决策提示词
MODEL_VISION = 'minicpm-v:latest'  # 视觉大模型
MODEL_TOOL = 'MFDoom/deepseek-r1-tool-calling:1.5b'  # 函数调用模型

# 初始化Ollama客户端
client = Client(host='http://localhost:11434')

def preload_models():
    """
    预加载模型到内存中并保持常驻
    """
    print("\n[系统] 开始预加载模型并保持常驻内存...")
    
    try:
        # 预加载视觉模型并保持常驻
        print("[系统] 正在预加载视觉模型并保持常驻...")
        response = requests.post(
            'http://localhost:11434/api/generate',
            json={
                'model': MODEL_VISION,
                'prompt': ' ',  # 空提示
                'keep_alive': -1  # 保持模型常驻内存
            }
        )
        response.raise_for_status()
        
        # 预加载函数调用模型并保持常驻
        print("[系统] 正在预加载函数调用模型并保持常驻...")
        response = requests.post(
            'http://localhost:11434/api/generate',
            json={
                'model': MODEL_TOOL,
                'prompt': ' ',  # 空提示
                'keep_alive': -1  # 保持模型常驻内存
            }
        )
        response.raise_for_status()
        
        print("[系统] 模型预加载完成并保持常驻内存")
    except Exception as e:
        print(f"[错误] 模型预加载失败: {str(e)}")
        raise

def check_model_status():
    """
    检查模型是否已加载
    """
    try:
        response = requests.get('http://localhost:11434/api/tags')
        response.raise_for_status()
        models = [model['name'] for model in response.json()['models']]
        
        print("\n[系统] 当前已加载模型:")
        for model in models:
            print(f" - {model}")
            
        return MODEL_VISION in models and MODEL_TOOL in models
    except Exception as e:
        print(f"[错误] 检查模型状态失败: {str(e)}")
        return False

def vision_recognition(image_path):
    """
    视觉识别处理函数
    参数:
        image_path: 图片文件路径
    返回:
        str: 视觉模型返回的场景描述
    """
    print("\n[系统] 开始视觉识别处理...")
    
    try:
        # 读取图片文件
        with open(image_path, 'rb') as f:
            image_data = f.read()
            
            # 调用视觉模型进行识别，保持模型常驻
            stream = client.generate(
                model=MODEL_VISION,
                prompt=PROMPT_VISION,
                images=[image_data],
                stream=True,
                keep_alive=-1  # 保持模型常驻内存
            )
            
            # 收集和处理响应
            full_response = []
            print("[系统] 已提交视觉识别请求，等待响应...")
            
            for chunk in stream:
                print(chunk['response'], end='', flush=True)
                full_response.append(chunk['response'])
            
            # 返回完整的场景描述
            return ''.join(full_response)
    except Exception as e:
        print(f"[错误] 视觉识别失败: {str(e)}")
        raise

def decision_making(scene_description):
    """
    决策和函数调用处理
    参数:
        scene_description: 视觉模型返回的场景描述
    """
    print("\n[系统] 开始决策和函数调用处理...")
    
    try:
        # 准备对话消息
        messages = [
            {'role': 'system', 'content': '你是一个无人机控制系统，需要根据视觉描述做出决策'},
            {'role': 'user', 'content': scene_description},
            {'role': 'user', 'content': PROMPT_DECISION}
        ]
        
        # 调用函数模型，保持模型常驻
        response = client.chat(
            model=MODEL_TOOL,
            messages=messages,
            tools=[move],  # 注册可调用的函数
            stream=True,
            keep_alive=-1  # 保持模型常驻内存
        )
        
        # 处理响应
        full_response = []
        tool_calls = []
        print("[系统] 已提交决策请求，等待响应...")
        
        for chunk in response:
            if 'message' in chunk:
                message = chunk['message']
                
                # 收集文本响应
                if 'content' in message and message['content']:
                    content = message['content']
                    print(content, end='', flush=True)
                    full_response.append(content)
                
                # 收集函数调用请求
                if 'tool_calls' in message:
                    tool_calls.extend(message['tool_calls'])
        
        # 处理函数调用
        if tool_calls:
            print("\n[系统] 检测到函数调用请求")
            for tool_call in tool_calls:
                func_name = tool_call['function']['name']
                args = tool_call['function']['arguments']
                
                print(f"[系统] 调用函数: {func_name}, 参数: {args}")

                # 执行对应的函数
                if func_name == "move":
                    try:
                        # 参数已经是字典格式，直接处理
                        if isinstance(args, str):
                            # 如果是字符串，先转换为字典
                            args = json.loads(args.replace("'", '"'))
                        
                        # 确保所有值为数值类型
                        numeric_args = {
                            'duration': float(args.get('duration', 10)),
                            'forward': float(args.get('forward', 0)),
                            'backward': float(args.get('backward', 0)),
                            'left': float(args.get('left', 0)),
                            'right': float(args.get('right', 0)),
                            'up': float(args.get('up', 0)),
                            'down': float(args.get('down', 0))
                        }
                        move(**numeric_args)
                    except Exception as e:
                        print(f"[错误] 函数调用参数处理失败: {str(e)}")
                        raise
        
        # 记录完整日志
        with open('../result/ollama_decision.log', 'a', encoding='utf-8') as log_file:
            log_file.write(''.join(full_response))
            log_file.write('\n')
    except Exception as e:
        print(f"[错误] 决策处理失败: {str(e)}")
        raise

def move(duration=10.0, forward=0.0, backward=0.0, left=0.0, right=0.0, up=0.0, down=0.0):
    """
    无人机运动控制函数
    参数:
        duration: 运动持续时间(秒) 默认10秒
        forward: 向前速度(0~1) 默认0
        backward: 向后速度(0~1) 默认0
        left: 向左速度(0~1) 默认0
        right: 向右速度(0~1) 默认0
        up: 向上速度(0~1) 默认0
        down: 向下速度(0~1) 默认0
    """
    try:
        # 验证参数范围
        duration = max(0.1, min(float(duration), 60))  # 限制在0.1-60秒之间
        forward = max(0, min(float(forward), 1))
        backward = max(0, min(float(backward), 1))
        left = max(0, min(float(left), 1))
        right = max(0, min(float(right), 1))
        up = max(0, min(float(up), 1))
        down = max(0, min(float(down), 1))
        
        # 计算各轴速度
        velocity_x = forward - backward
        velocity_y = right - left
        velocity_z = down - up
        
        # 打印运动指令
        print(f"\n[控制] 执行运动指令:")
        print(f"时长: {duration}秒")
        print(f"X轴速度: {velocity_x:.2f} (前+/后-)")
        print(f"Y轴速度: {velocity_y:.2f} (右+/左-)")
        print(f"Z轴速度: {velocity_z:.2f} (下+/上-)")
        
    except Exception as e:
        print(f"[错误] 运动控制参数无效: {str(e)}")
        raise

# 主程序
if __name__ == "__main__":
    try:
        # 0. 预加载模型并保持常驻
        preload_models()
        
        # 检查模型状态
        if not check_model_status():
            print("[警告] 部分模型未正确加载，尝试重新加载...")
            preload_models()
        
        # 1. 视觉识别阶段
        image_path = '../assets/scene1.png'  # 图片路径
        scene_description = vision_recognition(image_path)
        
        # 2. 决策和控制阶段
        decision_making(scene_description)
        
        print("\n[系统] 任务执行完成")
    except Exception as e:
        print(f"[系统] 任务执行失败: {str(e)}")
```
待完善代码:
```py
# 1. 调用视觉模型分析图像
prompt_vision = "分析图片深度信息"
vision_result = ""
# depth_imgs_base64数据转换以输入模型

# TODO
```
根据示例代码完善代码块;

## 回答
根据示例代码的结构和风格，我来完善这个调用视觉模型分析图像深度信息的代码块。以下是完善后的代码：

```py
# 1. 调用视觉模型分析图像
prompt_vision = "分析图片深度信息"
vision_result = ""

try:
    print("\n[系统] 开始视觉深度分析处理...")
    
    # 假设depth_imgs_base64是已经准备好的base64编码图像数据列表
    # 将base64字符串转换为字节数据
    image_data = [base64.b64decode(img) for img in depth_imgs_base64]
    
    # 调用视觉模型进行深度分析，保持模型常驻
    stream = client.generate(
        model=MODEL_VISION,
        prompt=prompt_vision,
        images=image_data,
        stream=True,
        keep_alive=-1  # 保持模型常驻内存
    )
    
    # 收集和处理响应
    full_response = []
    print("[系统] 已提交视觉深度分析请求，等待响应...")
    
    for chunk in stream:
        print(chunk['response'], end='', flush=True)
        full_response.append(chunk['response'])
    
    # 获取完整的深度分析结果
    vision_result = ''.join(full_response)
    
    # 记录分析结果到日志
    with open('../result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
        log_file.write(vision_result)
        log_file.write('\n')
        
except Exception as e:
    print(f"[错误] 视觉深度分析失败: {str(e)}")
    raise
```

注意事项：
1. 需要确保已经导入base64模块（`import base64`）
2. `depth_imgs_base64`应该是预先准备好的base64编码图像数据列表
3. 这段代码保持了与示例代码一致的错误处理、日志记录和模型常驻机制
4. 输出结果会同时打印到控制台和保存到日志文件
5. 使用了与示例相同的Ollama客户端调用方式

如果需要处理单张图像而不是列表，可以修改为：
```py
image_data = base64.b64decode(depth_img_base64)  # 单张图像
# 然后在client.generate调用中使用images=[image_data]
```

# 问题
```sh
curl -X POST http://192.168.100.229:7861/gradio_api/call/predict -s -H "Content-Type: application/json" -d '{
  "data": [
							{"path":"/Users/workspace/Desktop/projects/毕业设计/工程文件/exp256-webots-seekslam/tests/controllers/mission6_supervisor/stage_images/1.png","meta":{"_type":"gradio.FileData"}},
							"ch"
]}' \
  | awk -F'"' '{ print $4}'  \
  | read EVENT_ID; curl -N http://192.168.100.229:7861/gradio_api/call/predict/$EVENT_ID
```
修正命令为一行;

## 回答
Here's the corrected command in a single line:

```sh
curl -X POST http://192.168.100.229:7861/gradio_api/call/predict -s -H "Content-Type: application/json" -d '{"data": [{"path":"/Users/workspace/Desktop/projects/毕业设计/工程文件/exp256-webots-seekslam/tests/controllers/mission6_supervisor/stage_images/1.png","meta":{"_type":"gradio.FileData"}},"ch"]}' | awk -F'"' '{ print $4}' | read EVENT_ID && curl -N http://192.168.100.229:7861/gradio_api/call/predict/$EVENT_ID
```

Key changes:
1. Combined the two curl commands with `&&` instead of separate lines
2. Removed unnecessary newlines and whitespace in the JSON payload
3. Used `read EVENT_ID` in the same pipeline with `&&` to ensure the variable is available for the second curl command

# 问题
```txt
{
"boxes"
:
[
"0"
:
[
"0"
:
[
"0"
:
764
,
"1"
:
8
]
,
"1"
:
[
"0"
:
787
,
"1"
:
8
]
,
"2"
:
[
"0"
:
787
,
"1"
:
33
]
,
"3"
:
[
"0"
:
764
,
"1"
:
33
]
]
]
,
"texts"
:
[
"0"
:
"X"
]
,
"scores"
:
[
"0"
:
0.8657664656639099
]
}
```
将如上json合并到一行;

## 回答
Here's the JSON data merged into a single line:

```json
{"boxes":[[[[764,8],[787,8],[787,33],[764,33]]]],"texts":["X"],"scores":[0.8657664656639099]}
```

This maintains all the original structure and data while removing all unnecessary whitespace and newlines. The content is exactly the same, just compressed into a single line format.

# 问题
提示:
```py
# 无人机视觉识别与自主决策控制系统
# 功能：通过视觉大模型识别场景，然后使用函数调用模型进行决策和控制

import ollama
from ollama import Client
from typing_extensions import MutableMapping 
import time
import json
import requests  # 用于直接调用API接口

# 系统常量定义
PROMPT_VISION = '图片里面有几个字母？描述字母相对无人机的位置关系, 无人机更靠近哪个字母?'  # 视觉识别提示词
PROMPT_DECISION = '攻击目标为B, 根据相对关系移动无人机到B前方'  # 决策提示词
MODEL_VISION = 'minicpm-v:latest'  # 视觉大模型
MODEL_TOOL = 'MFDoom/deepseek-r1-tool-calling:1.5b'  # 函数调用模型

# 初始化Ollama客户端
client = Client(host='http://localhost:11434')

def preload_models():
    """
    预加载模型到内存中并保持常驻
    """
    print("\n[系统] 开始预加载模型并保持常驻内存...")
    
    try:
        # 预加载视觉模型并保持常驻
        print("[系统] 正在预加载视觉模型并保持常驻...")
        response = requests.post(
            'http://localhost:11434/api/generate',
            json={
                'model': MODEL_VISION,
                'prompt': ' ',  # 空提示
                'keep_alive': -1  # 保持模型常驻内存
            }
        )
        response.raise_for_status()
        
        # 预加载函数调用模型并保持常驻
        print("[系统] 正在预加载函数调用模型并保持常驻...")
        response = requests.post(
            'http://localhost:11434/api/generate',
            json={
                'model': MODEL_TOOL,
                'prompt': ' ',  # 空提示
                'keep_alive': -1  # 保持模型常驻内存
            }
        )
        response.raise_for_status()
        
        print("[系统] 模型预加载完成并保持常驻内存")
    except Exception as e:
        print(f"[错误] 模型预加载失败: {str(e)}")
        raise

def check_model_status():
    """
    检查模型是否已加载
    """
    try:
        response = requests.get('http://localhost:11434/api/tags')
        response.raise_for_status()
        models = [model['name'] for model in response.json()['models']]
        
        print("\n[系统] 当前已加载模型:")
        for model in models:
            print(f" - {model}")
            
        return MODEL_VISION in models and MODEL_TOOL in models
    except Exception as e:
        print(f"[错误] 检查模型状态失败: {str(e)}")
        return False

def vision_recognition(image_path):
    """
    视觉识别处理函数
    参数:
        image_path: 图片文件路径
    返回:
        str: 视觉模型返回的场景描述
    """
    print("\n[系统] 开始视觉识别处理...")
    
    try:
        # 读取图片文件
        with open(image_path, 'rb') as f:
            image_data = f.read()
            
            # 调用视觉模型进行识别，保持模型常驻
            stream = client.generate(
                model=MODEL_VISION,
                prompt=PROMPT_VISION,
                images=[image_data],
                stream=True,
                keep_alive=-1  # 保持模型常驻内存
            )
            
            # 收集和处理响应
            full_response = []
            print("[系统] 已提交视觉识别请求，等待响应...")
            
            for chunk in stream:
                print(chunk['response'], end='', flush=True)
                full_response.append(chunk['response'])
            
            # 返回完整的场景描述
            return ''.join(full_response)
    except Exception as e:
        print(f"[错误] 视觉识别失败: {str(e)}")
        raise

def decision_making(scene_description):
    """
    决策和函数调用处理
    参数:
        scene_description: 视觉模型返回的场景描述
    """
    print("\n[系统] 开始决策和函数调用处理...")
    
    try:
        # 准备对话消息
        messages = [
            {'role': 'system', 'content': '你是一个无人机控制系统，需要根据视觉描述做出决策'},
            {'role': 'user', 'content': scene_description},
            {'role': 'user', 'content': PROMPT_DECISION}
        ]
        
        # 调用函数模型，保持模型常驻
        response = client.chat(
            model=MODEL_TOOL,
            messages=messages,
            tools=[move],  # 注册可调用的函数
            stream=True,
            keep_alive=-1  # 保持模型常驻内存
        )
        
        # 处理响应
        full_response = []
        tool_calls = []
        print("[系统] 已提交决策请求，等待响应...")
        
        for chunk in response:
            if 'message' in chunk:
                message = chunk['message']
                
                # 收集文本响应
                if 'content' in message and message['content']:
                    content = message['content']
                    print(content, end='', flush=True)
                    full_response.append(content)
                
                # 收集函数调用请求
                if 'tool_calls' in message:
                    tool_calls.extend(message['tool_calls'])
        
        # 处理函数调用
        if tool_calls:
            print("\n[系统] 检测到函数调用请求")
            for tool_call in tool_calls:
                func_name = tool_call['function']['name']
                args = tool_call['function']['arguments']
                
                print(f"[系统] 调用函数: {func_name}, 参数: {args}")

                # 执行对应的函数
                if func_name == "move":
                    try:
                        # 参数已经是字典格式，直接处理
                        if isinstance(args, str):
                            # 如果是字符串，先转换为字典
                            args = json.loads(args.replace("'", '"'))
                        
                        # 确保所有值为数值类型
                        numeric_args = {
                            'duration': float(args.get('duration', 10)),
                            'forward': float(args.get('forward', 0)),
                            'backward': float(args.get('backward', 0)),
                            'left': float(args.get('left', 0)),
                            'right': float(args.get('right', 0)),
                            'up': float(args.get('up', 0)),
                            'down': float(args.get('down', 0))
                        }
                        move(**numeric_args)
                    except Exception as e:
                        print(f"[错误] 函数调用参数处理失败: {str(e)}")
                        raise
        
        # 记录完整日志
        with open('../result/ollama_decision.log', 'a', encoding='utf-8') as log_file:
            log_file.write(''.join(full_response))
            log_file.write('\n')
    except Exception as e:
        print(f"[错误] 决策处理失败: {str(e)}")
        raise

def move(duration=10.0, forward=0.0, backward=0.0, left=0.0, right=0.0, up=0.0, down=0.0):
    """
    无人机运动控制函数
    参数:
        duration: 运动持续时间(秒) 默认10秒
        forward: 向前速度(0~1) 默认0
        backward: 向后速度(0~1) 默认0
        left: 向左速度(0~1) 默认0
        right: 向右速度(0~1) 默认0
        up: 向上速度(0~1) 默认0
        down: 向下速度(0~1) 默认0
    """
    try:
        # 验证参数范围
        duration = max(0.1, min(float(duration), 60))  # 限制在0.1-60秒之间
        forward = max(0, min(float(forward), 1))
        backward = max(0, min(float(backward), 1))
        left = max(0, min(float(left), 1))
        right = max(0, min(float(right), 1))
        up = max(0, min(float(up), 1))
        down = max(0, min(float(down), 1))
        
        # 计算各轴速度
        velocity_x = forward - backward
        velocity_y = right - left
        velocity_z = down - up
        
        # 打印运动指令
        print(f"\n[控制] 执行运动指令:")
        print(f"时长: {duration}秒")
        print(f"X轴速度: {velocity_x:.2f} (前+/后-)")
        print(f"Y轴速度: {velocity_y:.2f} (右+/左-)")
        print(f"Z轴速度: {velocity_z:.2f} (下+/上-)")
        
    except Exception as e:
        print(f"[错误] 运动控制参数无效: {str(e)}")
        raise

# 主程序
if __name__ == "__main__":
    try:
        # 0. 预加载模型并保持常驻
        preload_models()
        
        # 检查模型状态
        if not check_model_status():
            print("[警告] 部分模型未正确加载，尝试重新加载...")
            preload_models()
        
        # 1. 视觉识别阶段
        image_path = '../assets/scene1.png'  # 图片路径
        scene_description = vision_recognition(image_path)
        
        # 2. 决策和控制阶段
        decision_making(scene_description)
        
        print("\n[系统] 任务执行完成")
    except Exception as e:
        print(f"[系统] 任务执行失败: {str(e)}")
```
代码:
```py
# 智慧无人机决策控制系统
# 特点: 接入ollama大模型能力, 实现智能决策
# 功能: 通过视觉大模型识别场景，使用函数调用模型进行多阶段决策和控制

# 攻击目标(与tasks中一致)
_TARGET = "B"
# 系统提示词
_SYSTEM_PROMPT = f"你是一架智慧无人机, 你的任务是找到{_TARGET}并进行攻击, 然后返回到起飞点.你可以通过深度图、OCR识别图、三维重建图进行决策.你可以使用up, down, left, right, forward, backward, rotate, attack函数.你需要不断地进行思考以应对各种情况."
# 阶段1(起飞后在第一个房间建图后决策)提示词
_PROMPT1 = "现在你在第一个房间起飞了, 悬停在1.5米高度. 刚刚使用三维重建算法对房间进行了建图, 你需要找到通往第二个房间的方向并调用函数移动."
# 阶段2(在两个房间交界处悬停决策)提示词
_PROMPT2 = "你来到了两个房间的交界处, 悬停在1.5米高度, 先侦察一下第二个房间的状况. 刚刚使用深度估计算法获取了朝向面的深度图(通过颜色区分深度), 你需要判断深度信息{_DEPTH_INFO}以判断应该如何前进, 注意避开障碍物.刚刚使用OCR算法识别目标, 结果为{_OCR_RESULT}, 你需要调用函数移动到目标或者前进以继续搜索目标."
# 阶段3(在第二个房间建图后决策)提示词
_PROMPT3 = "刚刚对第二个房间建图成功, 发现目标方位{_OCR_RESULT}, 应该马上调用函数移动到目标进行攻击.移动时注意深度信息进行避障."
# 阶段4(在目标前判断是否攻击)提示词
_PROMPT4 = f"你来到了目标{_TARGET}前面, 在有效攻击范围内, 使用函数对其进行攻击, 然后原路撤离, 回到起飞点降落."
# 对话记录
_CHAT_HISTORY = [{"q":"", "a":""},]
# 视觉大模型
MODEL_VISION = 'minicpm-v:latest'  
# 函数调用模型
MODEL_TOOL = 'MFDoom/deepseek-r1-tool-calling:1.5b'  
#####################################
# 大模型接口
from ollama import Client
# python版本兼容
from typing_extensions import MutableMapping 
# 单元测试
import pytest
# 图像处理
from PIL import Image
# 内置库
import time
import json
import requests
import base64
import io
import os
import sys
#####################################

# 输入: 原图, 深度图, ocr结果
# 输出: 控制指令 up, down, forward, backward, left, right, rotate, attack

class _Ollama():
    def __init__(self, robot, controller):
        # 初始化ollama客户端
        self.client = Client(host='http://10.8.8.130:11434')
        # 用于函数调用的精简函数集合
        self.fn = _Move(robot, controller)
        # 添加完成数据的prompt
        self.prompt = ["", ]
        # 当前决策阶段
        self.current_stage = 1
        # 对话历史记录
        self.chat_history = []
        # 起飞前预加载模型
        self._preload_models()

    def _preload_models(self):
        """预加载并保持模型常驻内存"""
        try:
            print("[系统] 开始预加载AI模型...")
            # 预加载视觉模型
            requests.post(
                'http://10.8.8.130:11434/api/generate',
                json={'model': MODEL_VISION, 'prompt': ' ', 'keep_alive': -1}
            )
            # 预加载函数调用模型
            requests.post(
                'http://10.8.8.130:11434/api/generate', 
                json={'model': MODEL_TOOL, 'prompt': ' ', 'keep_alive': -1}
            )
            print("[系统] 模型预加载完成")
        except Exception as e:
            print(f"[错误] 模型预加载失败: {str(e)}")
            raise

    # 对外接口, 预处理prompt
    def preprecess_prompt(self, *args, **kwargs):
        '''
        接受任意类型和任意数量的参数, 转换为str并格式化到prompt中
        
        参数:
            *args: 位置参数列表
            **kwargs: 关键字参数字典
            
        返回:
            str: 格式化后的prompt字符串
        '''
        prompt_parts = []
        
        # 处理位置参数
        for i, arg in enumerate(args):
            if isinstance(arg, (list, dict)):
                # 对列表和字典使用json格式化
                prompt_parts.append(f"参数{i+1}:\n{json.dumps(arg, indent=2, ensure_ascii=False)}")
            else:
                # 其他类型直接转字符串
                prompt_parts.append(f"参数{i+1}: {str(arg)}")
        
        # 处理关键字参数
        for key, value in kwargs.items():
            if isinstance(value, (list, dict)):
                # 对列表和字典使用json格式化
                prompt_parts.append(f"{key}:\n{json.dumps(value, indent=2, ensure_ascii=False)}")
            else:
                # 其他类型直接转字符串
                prompt_parts.append(f"{key}: {str(value)}")
        
        # 将所有部分用换行符连接
        formatted_prompt = "\n".join(prompt_parts)
        
        # 更新当前prompt
        if hasattr(self, 'prompt'):
            self.prompt.append(formatted_prompt)
        else:
            self.prompt = [formatted_prompt]
            
        return formatted_prompt
    
    # 对外接口, 每个阶段明确思考内容
    def process_stage(self, stage=self.current_stage, image_base64=None, depth_imgs_base64=None, ocr_result=None):
        """
        处理当前任务阶段
        参数:
            image_base64: 原始视觉图像(图像base64数据)
            depth_imgs_base64: 深度估计结果png_base64
            ocr_result: OCR识别结果json
        返回:
            dict: 控制指令
        """
        try:
            # 根据阶段处理提示词
            if stage == 1:    
                prompt = _PROMPT1
            elif stage == 2:
                # 将OCR结果格式化进入
                prompt = _PROMPT2.format(_OCR_RESULT=_OCR_RESULT)
            elif stage == 3:
                prompt = _PROMPT3.format(_OCR_RESULT=_OCR_RESULT)
            elif stage == 4:
                prompt = _PROMPT4
            else: 
                prompt = f"你是自主侦察无人机, 你的任务是在室内搜索{_TARGET}并攻击."

            # 1. 调用视觉模型分析图像
            prompt_vision = "分析图片深度信息"
            vision_result = ""
            # 图像列表
            image_base64_list = []
            if image_base64 != None:
                image_base64_list.append(image_base64)
            if depth_imgs_base64 != None:
                image_base64_list.append(depth_imgs_base64)
            image_bytes_list = []

            # 转换为字节数组
            image_bytes_list = [base64.b64decode(img) for img in image_base64_list]
            
            try:
                print("\n[系统] 开始视觉深度分析处理...")
                
                # 假设depth_imgs_base64是已经准备好的base64编码图像数据列表
                # 将base64字符串转换为字节数据
                image_data = [base64.b64decode(img) for img in depth_imgs_base64]
                
                # 调用视觉模型进行深度分析，保持模型常驻
                stream = client.generate(
                    model=MODEL_VISION,
                    prompt=prompt_vision,
                    images=image_bytes_list,
                    stream=True,
                    keep_alive=-1  # 保持模型常驻内存
                )
                
                # 收集和处理响应
                full_response = []
                print("[系统] 已提交视觉深度分析请求，等待响应...")
                
                for chunk in stream:
                    print(chunk['response'], end='', flush=True)
                    full_response.append(chunk['response'])
                
                # 获取完整的深度分析结果
                vision_result = ''.join(full_response)
                
                # 记录分析结果到日志
                with open('./result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
                    log_file.write(vision_result)
                    log_file.write('\n')
                    
            except Exception as e:
                print(f"[错误] 视觉深度分析失败: {str(e)}")
                raise

            # 将视觉大模型返回的结果添加到prompt中
            this_prompt = ''
            try:
                this_prompt = prompt.format(_DEPTH_INFO=vision_result)
            except Exception as e:
                print("可能不需要深度信息")

            # 构建对话消息
            messages = [
                # 角色提示词
                {"role": "system", "content": _SYSTEM_PROMPT},
                # 对话历史
                *self._format_history(),
                # 阶段提示词
                {"role": "user", "content": this_prompt}
            ]
            
            # 2. 调用函数调用模型进行决策
            response = self.client.chat(
                model=MODEL_TOOL,
                messages=messages,
                tools=[self.fn.up, self.fn.down, self.fn.left, 
                      self.fn.right, self.fn.forward, self.fn.backward,
                      self.fn.rotate, self.fn.attack],
                stream=True
            )
            
            # 处理响应
            tool_calls = []
            full_response = []
            for chunk in response:
                if 'message' in chunk:
                    msg = chunk['message']
                    # 收集文本响应
                    if 'content' in msg:
                        print(msg['content'], end='', flush=True)
                        full_response.append(msg['content'])
                    # 收集函数调用
                    if 'tool_calls' in msg:
                        tool_calls.extend(msg['tool_calls'])
            
            # 执行函数调用
            if tool_calls:
                print("\n[系统] 执行函数调用...")
                for call in tool_calls:
                    self._execute_function_call(call)
            
            # 更新对话历史
            self._update_history(prompt, ''.join(full_response))
            
            # 检查阶段是否完成
            self._check_stage_completion()
            
            return {"status": "success", "stage": self.current_stage}
            
        except Exception as e:
            print(f"[错误] 决策处理失败: {str(e)}")
            return {"status": "error", "message": str(e)}
    
    def _execute_function_call(self, call):
        """执行函数调用"""
        func_name = call['function']['name']
        args = call['function']['arguments']
        
        try:
            # 转换参数格式
            if isinstance(args, str):
                args = json.loads(args.replace("'", '"'))
                
            print(f"[执行] {func_name}({args})")
            
            # 调用对应函数
            func = getattr(self.fn, func_name)
            if func_name == "attack":
                func()  # 无参数
            else:
                # 运动函数参数处理
                arg_name = 'cm' if func_name != 'rotate' else 'degrees'
                value = args.get(arg_name, 0)
                
                # 处理字符串值中的单位
                if isinstance(value, str):
                    if 'degrees' in value:
                        value = value.replace('degrees', '').strip()
                    elif 'cm' in value:
                        value = value.replace('cm', '').strip()
                
                # 转换为float
                value = float(value)
                
                func(value)
                
        except Exception as e:
            print(f"[错误] 函数执行失败: {str(e)}")
            raise
    
    def _format_history(self):
        """格式化对话历史"""
        return [
            dict_obj
            for item in self.chat_history[-20:]  # 保留最近20条
            for dict_obj in (
                {"role": "user", "content": item["q"]},
                {"role": "assistant", "content": item["a"]}
            )
        ]
    
    def _update_history(self, question, answer):
        """更新对话历史"""
        self.chat_history.append({"q": question, "a": answer})
        # 打印最后一条记录
        print("最新对话记录:", self.chat_history[-1])
        
    def _check_stage_completion(self):
        """检查并更新任务阶段"""
        # 简单决策树：根据当前状态和响应判断是否进入下一阶段
        if self.current_stage == 1 and "move" in str(self.chat_history[-1]["a"]):
            pass
            # self.current_stage = 2
        elif self.current_stage == 2 and _TARGET in str(self.chat_history[-1]["a"]):
            pass
            # self.current_stage = 3
        elif self.current_stage == 3 and "attack" in str(self.chat_history[-1]["a"]):
            pass
            # self.current_stage = 4
        elif self.current_stage == 4 and "move" in str(self.chat_history[-1]["a"]):
            pass
            print("[系统] 任务完成！")
            # self.current_stage = 1  # 重置

#####################################
class _Move():
    '''给大模型函数调用(Function Call)的精简函数'''
    def __init__(self, robot, controller):
        self.controller = controller
        self.robot = robot
        self.drone = self.robot
    def up(self, cm):
        '''上升'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_up(cm)
            print(f"函数up:{cm}厘米")
    def down(self, cm):
        '''下降'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_down(cm)
            print(f"函数down:{cm}厘米")
    def left(self, cm):
        '''左移'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_left(cm)
            print(f"函数left:{cm}厘米")
    def right(self, cm):
        '''右移'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_right(cm)
            print(f"函数right:{cm}厘米")
    def forward(self, cm):
        '''前进'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_forward(cm)
            print(f"函数forward:{cm}厘米")
    def backward(self, cm):
        '''后退'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_backward(cm)
            print(f"函数backward:{cm}厘米")
    def rotate(self, degrees):
        '''旋转'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.rotate(degrees)
            print(f"函数rotate:{degrees}度")
    def attack(self):
        '''攻击'''
        # 打开激光
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.attack(True)
        # 等待一会
        time.sleep(5)
        # 关闭激光
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.attack(False)
        print("函数attack完成")
#####################################
# 单元测试

# 测试例程
if __name__ == "__main__":
    # 阶段1的图片
    stage1_original = "./stage_images/1.png"
    stage1_depth = "./stage_images/1_depth.webp"
    stage1_ocr = """{"boxes":[[[[764,8],[787,8],[787,33],[764,33]]]],"texts":["X"],"scores":[0.8657664656639099]}"""
    
    # 阶段2的图片
    stage2_original = "./stage_images/2.png"
    stage2_depth = "./stage_images/2_depth.webp"
    stage2_ocr = """{"boxes":[[[[763,3],[790,3],[790,31],[763,31]]],[[[242,229],[330,231],[327,343],[239,340]]]],"texts":["X","C"],"scores":[0.7861287593841553,0.5938434600830078]}"""
    
    # 阶段3的图片
    stage3_original = "./stage_images/3.png"
    stage3_depth = "./stage_images/3_depth.webp"
    stage3_ocr = """{"boxes":[[[[345,184],[485,180],[490,382],[350,386]]]],"texts":["B"],"scores":[0.9931768178939819]}"""

    # 阶段4的图片
    stage4_original = "./stage_images/4.png"
    stage4_depth = "./stage_images/4_depth.webp"
    stage4_ocr = "错误"

    # 测试决策
    # 模拟测试
    class MockController:
        def move_up(self, cm): pass
        def move_down(self, cm): pass
        def move_left(self, cm): pass
        def move_right(self, cm): pass
        def move_forward(self, cm): pass
        def move_backward(self, cm): pass
        def rotate(self, degrees): pass
        def laser_on(self): print("激光开启")
        def laser_off(self): print("激光关闭")
    
    _agent = _Ollama(None, MockController())

    # 辅助函数：将任意格式图像转换为PNG字节流
    def convert_to_png_bytes(image_path):
        """将任意格式图像转换为PNG格式的字节流"""
        try:
            with Image.open(image_path) as img:
                # 转换为RGB模式（兼容不支持透明通道的模型）
                if img.mode != 'RGB':
                    img = img.convert('RGB')
                
                # 转换为字节流
                img_byte_arr = io.BytesIO()
                img.save(img_byte_arr, format='PNG')
                return img_byte_arr.getvalue()
        except Exception as e:
            print(f"[错误] 图像转换失败({image_path}): {str(e)}")
            return None
        
    # 辅助函数：将图片转换为base64
    def image_to_base64(image_path):
        with open(image_path, "rb") as image_file:
            return base64.b64encode(image_file.read()).decode('utf-8')
    
    # 辅助函数: 将任意格式图片转为png格式base64
    def any_image_to_base64(image_path):
        """将任意格式图像转换为PNG格式的base64编码字符串
        
        Args:
            image_path (str): 图像文件路径
            
        Returns:
            str: base64编码的PNG图像字符串，失败时返回None
        """
        try:
            with Image.open(image_path) as img:
                # 转换为RGB模式（兼容不支持透明通道的模型）
                if img.mode != 'RGB':
                    img = img.convert('RGB')
                
                # 转换为字节流
                img_byte_arr = io.BytesIO()
                img.save(img_byte_arr, format='PNG')
                img_bytes = img_byte_arr.getvalue()
                
                # 转换为base64编码
                base64_str = base64.b64encode(img_bytes).decode('utf-8')
                return base64_str
                
        except Exception as e:
            print(f"[错误] 图像转换失败({image_path}): {str(e)}", file=sys.stderr)
            return None
    
    # 模拟阶段1处理
    print("\n=== 阶段1测试 ===")
    # 准备图片数据
    stage1_images = [
        {"role": "user", "content": "原始图像", "images": [any_image_to_base64(stage1_original)]},
        {"role": "user", "content": "深度图像", "images": [any_image_to_base64(stage1_depth)]}
    ]
    # 调用视觉模型处理图片
    vision_response = _agent.client.chat(
        model=MODEL_VISION,
        messages=stage1_images
    )
    print("[视觉模型响应]")
    for chunk in vision_response:
        # 修正这里：先检查chunk是否是字典，如果是元组则转换为字典处理
        if isinstance(chunk, dict):
            if 'message' in chunk and 'content' in chunk['message']:
                print(chunk['message']['content'], end='', flush=True)
        elif isinstance(chunk, tuple):
            # 假设元组格式为 (status, data)
            _, data = chunk
            if 'message' in data and 'content' in data['message']:
                print(data['message']['content'], end='', flush=True)
    print("\n")

    # 调用决策模型
    _agent.process_stage(images=stage1_images)

    # 模拟阶段2处理（假设识别到目标B）
    print("\n=== 阶段2测试 ===")
    # 准备图片数据
    stage2_images = [
        {"role": "user", "content": "原始图像", "images": [any_image_to_base64(stage2_original)]},
        {"role": "user", "content": "深度图像", "images": [any_image_to_base64(stage2_depth)]}
    ]
    # 调用视觉模型处理图片
    vision_response = drone.client.chat(
        model=MODEL_VISION,
        messages=stage2_images
    )
    print("[视觉模型响应]")
    for chunk in vision_response:
        if isinstance(chunk, dict):
            if 'message' in chunk and 'content' in chunk['message']:
                print(chunk['message']['content'], end='', flush=True)
        elif isinstance(chunk, tuple):
            _, data = chunk
            if 'message' in data and 'content' in data['message']:
                print(data['message']['content'], end='', flush=True)
    print("\n")

    # 调用决策模型
    _agent.process_stage(images=stage2_images, depth_imgs_base64="前方2米处有障碍物", ocr_result="识别到字母B")

    # 模拟阶段3处理
    print("\n=== 阶段3测试 ===")
    # 准备图片数据
    stage3_images = [
        {"role": "user", "content": "原始图像", "images": [any_image_to_base64(stage3_original)]},
        {"role": "user", "content": "深度图像", "images": [any_image_to_base64(stage3_depth)]}
    ]
    # 调用视觉模型处理图片
    vision_response = drone.client.chat(
        model=MODEL_VISION,
        messages=stage3_images
    )
    print("[视觉模型响应]")
    for chunk in vision_response:
        if isinstance(chunk, dict):
            if 'message' in chunk and 'content' in chunk['message']:
                print(chunk['message']['content'], end='', flush=True)
        elif isinstance(chunk, tuple):
            _, data = chunk
            if 'message' in data and 'content' in data['message']:
                print(data['message']['content'], end='', flush=True)
    print("\n")

    # 调用决策模型
    _agent.process_stage(images=stage3_images)

    # 模拟阶段4处理
    print("\n=== 阶段4测试 ===")
    # 准备图片数据
    stage4_images = [
        {"role": "user", "content": "原始图像", "images": [any_image_to_base64(stage4_original)]},
        {"role": "user", "content": "深度图像", "images": [any_image_to_base64(stage4_depth)]}
    ]
    # 调用视觉模型处理图片
    vision_response = drone.client.chat(
        model=MODEL_VISION,
        messages=stage4_images
    )
    print("[视觉模型响应]")
    for chunk in vision_response:
        if isinstance(chunk, dict):
            if 'message' in chunk and 'content' in chunk['message']:
                print(chunk['message']['content'], end='', flush=True)
        elif isinstance(chunk, tuple):
            _, data = chunk
            if 'message' in data and 'content' in data['message']:
                print(data['message']['content'], end='', flush=True)
    print("\n")

    # 调用决策模型
    _agent.process_stage(images=stage4_images)
```
根据提示和代码修正测试例程;

## 回答
```py
# 智慧无人机决策控制系统
# 特点: 接入ollama大模型能力, 实现智能决策
# 功能: 通过视觉大模型识别场景，使用函数调用模型进行多阶段决策和控制

# 攻击目标(与tasks中一致)
_TARGET = "B"
# 系统提示词
_SYSTEM_PROMPT = f"你是一架智慧无人机, 你的任务是找到{_TARGET}并进行攻击, 然后返回到起飞点.你可以通过深度图、OCR识别图、三维重建图进行决策.你可以使用up, down, left, right, forward, backward, rotate, attack函数.你需要不断地进行思考以应对各种情况."
# 阶段1(起飞后在第一个房间建图后决策)提示词
_PROMPT1 = "现在你在第一个房间起飞了, 悬停在1.5米高度. 刚刚使用三维重建算法对房间进行了建图, 你需要找到通往第二个房间的方向并调用函数移动."
# 阶段2(在两个房间交界处悬停决策)提示词
_PROMPT2 = "你来到了两个房间的交界处, 悬停在1.5米高度, 先侦察一下第二个房间的状况. 刚刚使用深度估计算法获取了朝向面的深度图(通过颜色区分深度), 你需要判断深度信息{_DEPTH_INFO}以判断应该如何前进, 注意避开障碍物.刚刚使用OCR算法识别目标, 结果为{_OCR_RESULT}, 你需要调用函数移动到目标或者前进以继续搜索目标."
# 阶段3(在第二个房间建图后决策)提示词
_PROMPT3 = "刚刚对第二个房间建图成功, 发现目标方位{_OCR_RESULT}, 应该马上调用函数移动到目标进行攻击.移动时注意深度信息进行避障."
# 阶段4(在目标前判断是否攻击)提示词
_PROMPT4 = f"你来到了目标{_TARGET}前面, 在有效攻击范围内, 使用函数对其进行攻击, 然后原路撤离, 回到起飞点降落."
# 对话记录
_CHAT_HISTORY = [{"q": "", "a": ""}, ]
# 视觉大模型
MODEL_VISION = 'minicpm-v:latest'
# 函数调用模型
MODEL_TOOL = 'MFDoom/deepseek-r1-tool-calling:1.5b'
#####################################
# 大模型接口
from ollama import Client
# python版本兼容
from typing_extensions import MutableMapping
# 单元测试
import pytest
# 图像处理
from PIL import Image
# 内置库
import time
import json
import requests
import base64
import io
import os
import sys
#####################################

# 输入: 原图, 深度图, ocr结果
# 输出: 控制指令 up, down, forward, backward, left, right, rotate, attack

class _Ollama():
    def __init__(self, robot, controller):
        # 初始化ollama客户端
        self.client = Client(host='http://10.8.8.130:11434')
        # 用于函数调用的精简函数集合
        self.fn = _Move(robot, controller)
        # 添加完成数据的prompt
        self.prompt = ["", ]
        # 当前决策阶段
        self.current_stage = 1
        # 对话历史记录
        self.chat_history = []
        # 起飞前预加载模型
        self._preload_models()

    def _preload_models(self):
        """预加载并保持模型常驻内存"""
        try:
            print("[系统] 开始预加载AI模型...")
            # 预加载视觉模型
            requests.post(
                'http://10.8.8.130:11434/api/generate',
                json={'model': MODEL_VISION, 'prompt': ' ', 'keep_alive': -1}
            )
            # 预加载函数调用模型
            requests.post(
                'http://10.8.8.130:11434/api/generate',
                json={'model': MODEL_TOOL, 'prompt': ' ', 'keep_alive': -1}
            )
            print("[系统] 模型预加载完成")
        except Exception as e:
            print(f"[错误] 模型预加载失败: {str(e)}")
            raise

    # 对外接口, 预处理prompt
    def preprecess_prompt(self, *args, **kwargs):
        '''
        接受任意类型和任意数量的参数, 转换为str并格式化到prompt中

        参数:
            *args: 位置参数列表
            **kwargs: 关键字参数字典

        返回:
            str: 格式化后的prompt字符串
        '''
        prompt_parts = []

        # 处理位置参数
        for i, arg in enumerate(args):
            if isinstance(arg, (list, dict)):
                # 对列表和字典使用json格式化
                prompt_parts.append(f"参数{i + 1}:\n{json.dumps(arg, indent=2, ensure_ascii=False)}")
            else:
                # 其他类型直接转字符串
                prompt_parts.append(f"参数{i + 1}: {str(arg)}")

        # 处理关键字参数
        for key, value in kwargs.items():
            if isinstance(value, (list, dict)):
                # 对列表和字典使用json格式化
                prompt_parts.append(f"{key}:\n{json.dumps(value, indent=2, ensure_ascii=False)}")
            else:
                # 其他类型直接转字符串
                prompt_parts.append(f"{key}: {str(value)}")

        # 将所有部分用换行符连接
        formatted_prompt = "\n".join(prompt_parts)

        # 更新当前prompt
        if hasattr(self, 'prompt'):
            self.prompt.append(formatted_prompt)
        else:
            self.prompt = [formatted_prompt]

        return formatted_prompt

    # 对外接口, 每个阶段明确思考内容
    def process_stage(self, stage=None, image_base64=None, depth_imgs_base64=None, ocr_result=None):
        if stage is None:
            stage = self.current_stage
        """
        处理当前任务阶段
        参数:
            image_base64: 原始视觉图像(图像base64数据)
            depth_imgs_base64: 深度估计结果png_base64
            ocr_result: OCR识别结果json
        返回:
            dict: 控制指令
        """
        try:
            # 根据阶段处理提示词
            if stage == 1:
                prompt = _PROMPT1
            elif stage == 2:
                # 将OCR结果格式化进入
                prompt = _PROMPT2.format(_OCR_RESULT=ocr_result)
            elif stage == 3:
                prompt = _PROMPT3.format(_OCR_RESULT=ocr_result)
            elif stage == 4:
                prompt = _PROMPT4
            else:
                prompt = f"你是自主侦察无人机, 你的任务是在室内搜索{_TARGET}并攻击."

            # 1. 调用视觉模型分析图像
            prompt_vision = "分析图片深度信息"
            vision_result = ""
            # 图像列表
            image_base64_list = []
            if image_base64 is not None:
                image_base64_list.append(image_base64)
            if depth_imgs_base64 is not None:
                image_base64_list.append(depth_imgs_base64)
            image_bytes_list = []

            # 转换为字节数组
            image_bytes_list = [base64.b64decode(img) for img in image_base64_list]

            try:
                print("\n[系统] 开始视觉深度分析处理...")

                # 假设depth_imgs_base64是已经准备好的base64编码图像数据列表
                # 将base64字符串转换为字节数据
                image_data = [base64.b64decode(img) for img in depth_imgs_base64]

                # 调用视觉模型进行深度分析，保持模型常驻
                stream = self.client.generate(
                    model=MODEL_VISION,
                    prompt=prompt_vision,
                    images=image_bytes_list,
                    stream=True,
                    keep_alive=-1  # 保持模型常驻内存
                )

                # 收集和处理响应
                full_response = []
                print("[系统] 已提交视觉深度分析请求，等待响应...")

                for chunk in stream:
                    print(chunk['response'], end='', flush=True)
                    full_response.append(chunk['response'])

                # 获取完整的深度分析结果
                vision_result = ''.join(full_response)

                # 记录分析结果到日志
                with open('./result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
                    log_file.write(vision_result)
                    log_file.write('\n')

            except Exception as e:
                print(f"[错误] 视觉深度分析失败: {str(e)}")
                raise

            # 将视觉大模型返回的结果添加到prompt中
            this_prompt = ''
            try:
                this_prompt = prompt.format(_DEPTH_INFO=vision_result)
            except Exception as e:
                print("可能不需要深度信息")

            # 构建对话消息
            messages = [
                # 角色提示词
                {"role": "system", "content": _SYSTEM_PROMPT},
                # 对话历史
                *self._format_history(),
                # 阶段提示词
                {"role": "user", "content": this_prompt}
            ]

            # 2. 调用函数调用模型进行决策
            response = self.client.chat(
                model=MODEL_TOOL,
                messages=messages,
                tools=[self.fn.up, self.fn.down, self.fn.left,
                       self.fn.right, self.fn.forward, self.fn.backward,
                       self.fn.rotate, self.fn.attack],
                stream=True
            )

            # 处理响应
            tool_calls = []
            full_response = []
            for chunk in response:
                if 'message' in chunk:
                    msg = chunk['message']
                    # 收集文本响应
                    if 'content' in msg:
                        print(msg['content'], end='', flush=True)
                        full_response.append(msg['content'])
                    # 收集函数调用
                    if 'tool_calls' in msg:
                        tool_calls.extend(msg['tool_calls'])

            # 执行函数调用
            if tool_calls:
                print("\n[系统] 执行函数调用...")
                for call in tool_calls:
                    self._execute_function_call(call)

            # 更新对话历史
            self._update_history(prompt, ''.join(full_response))

            # 检查阶段是否完成
            self._check_stage_completion()

            return {"status": "success", "stage": self.current_stage}

        except Exception as e:
            print(f"[错误] 决策处理失败: {str(e)}")
            return {"status": "error", "message": str(e)}

    def _execute_function_call(self, call):
        """执行函数调用"""
        func_name = call['function']['name']
        args = call['function']['arguments']

        try:
            # 转换参数格式
            if isinstance(args, str):
                args = json.loads(args.replace("'", '"'))

            print(f"[执行] {func_name}({args})")

            # 调用对应函数
            func = getattr(self.fn, func_name)
            if func_name == "attack":
                func()  # 无参数
            else:
                # 运动函数参数处理
                arg_name = 'cm' if func_name != 'rotate' else 'degrees'
                value = args.get(arg_name, 0)

                # 处理字符串值中的单位
                if isinstance(value, str):
                    if 'degrees' in value:
                        value = value.replace('degrees', '').strip()
                    elif 'cm' in value:
                        value = value.replace('cm', '').strip()

                # 转换为float
                value = float(value)

                func(value)

        except Exception as e:
            print(f"[错误] 函数执行失败: {str(e)}")
            raise

    def _format_history(self):
        """格式化对话历史"""
        return [
            dict_obj
            for item in self.chat_history[-20:]  # 保留最近20条
            for dict_obj in (
                {"role": "user", "content": item["q"]},
                {"role": "assistant", "content": item["a"]}
            )
        ]

    def _update_history(self, question, answer):
        """更新对话历史"""
        self.chat_history.append({"q": question, "a": answer})
        # 打印最后一条记录
        print("最新对话记录:", self.chat_history[-1])

    def _check_stage_completion(self):
        """检查并更新任务阶段"""
        # 简单决策树：根据当前状态和响应判断是否进入下一阶段
        if self.current_stage == 1 and "move" in str(self.chat_history[-1]["a"]):
            self.current_stage = 2
        elif self.current_stage == 2 and _TARGET in str(self.chat_history[-1]["a"]):
            self.current_stage = 3
        elif self.current_stage == 3 and "attack" in str(self.chat_history[-1]["a"]):
            self.current_stage = 4
        elif self.current_stage == 4 and "move" in str(self.chat_history[-1]["a"]):
            print("[系统] 任务完成！")
            self.current_stage = 1  # 重置


#####################################
class _Move():
    '''给大模型函数调用(Function Call)的精简函数'''

    def __init__(self, robot, controller):
        self.controller = controller
        self.robot = robot
        self.drone = self.robot

    def up(self, cm):
        '''上升'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_up(cm)
            print(f"函数up:{cm}厘米")

    def down(self, cm):
        '''下降'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_down(cm)
            print(f"函数down:{cm}厘米")

    def left(self, cm):
        '''左移'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_left(cm)
            print(f"函数left:{cm}厘米")

    def right(self, cm):
        '''右移'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_right(cm)
            print(f"函数right:{cm}厘米")

    def forward(self, cm):
        '''前进'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_forward(cm)
            print(f"函数forward:{cm}厘米")

    def backward(self, cm):
        '''后退'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_backward(cm)
            print(f"函数backward:{cm}厘米")

    def rotate(self, degrees):
        '''旋转'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.rotate(degrees)
            print(f"函数rotate:{degrees}度")

    def attack(self):
        '''攻击'''
        # 打开激光
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.attack(True)
        # 等待一会
        time.sleep(5)
        # 关闭激光
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.attack(False)
        print("函数attack完成")


#####################################
# 单元测试

# 测试例程
if __name__ == "__main__":
    # 阶段1的图片
    stage1_original = "./stage_images/1.png"
    stage1_depth = "./stage_images/1_depth.webp"
    stage1_ocr = """{"boxes":[[[[764,8],[787,8],[787,33],[764,33]]]],"texts":["X"],"scores":[0.8657664656639099]}"""

    # 阶段2的图片
    stage2_original = "./stage_images/2.png"
    stage2_depth = "./stage_images/2_depth.webp"
    stage2_ocr = """{"boxes":[[[[763,3],[790,3],[790,31],[763,31]]],[[[242,229],[330,231],[327,343],[239,340]]]],"texts":["X","C"],"scores":[0.7861287593841553,0.5938434600830078]}"""

    # 阶段3的图片
    stage3_original = "./stage_images/3.png"
    stage3_depth = "./stage_images/3_depth.webp"
    stage3_ocr = """{"boxes":[[[[345,184],[485,180],[490,382],[350,386]]]],"texts":["B"],"scores":[0.9931768178939819]}"""

    # 阶段4的图片
    stage4_original = "./stage_images/4.png"
    stage4_depth = "./stage_images/4_depth.webp"
    stage4_ocr = "错误"

    # 测试决策
    # 模拟测试
    class MockController:
        def move_up(self, cm):
            pass

        def move_down(self, cm):
            pass

        def move_left(self, cm):
            pass

        def move_right(self, cm):
            pass

        def move_forward(self, cm):
            pass

        def move_backward(self, cm):
            pass

        def rotate(self, degrees):
            pass

        def attack(self, on):
            if on:
                print("激光开启")
            else:
                print("激光关闭")

    _agent = _Ollama(None, MockController())

    # 辅助函数：将任意格式图像转换为PNG字节流
    def convert_to_png_bytes(image_path):
        """将任意格式图像转换为PNG格式的字节流"""
        try:
            with Image.open(image_path) as img:
                # 转换为RGB模式（兼容不支持透明通道的模型）
                if img.mode != 'RGB':
                    img = img.convert('RGB')

                # 转换为字节流
                img_byte_arr = io.BytesIO()
                img.save(img_byte_arr, format='PNG')
                return img_byte_arr.getvalue()
        except Exception as e:
            print(f"[错误] 图像转换失败({image_path}): {str(e)}")
            return None

    # 辅助函数：将图片转换为base64
    def image_to_base64(image_path):
        with open(image_path, "rb") as image_file:
            return base64.b64encode(image_file.read()).decode('utf-8')

    # 辅助函数: 将任意格式图片转为png格式base64
    def any_image_to_base64(image_path):
        """将任意格式图像转换为PNG格式的base64编码字符串

        Args:
            image_path (str): 图像文件路径

        Returns:
            str: base64编码的PNG图像字符串，失败时返回None
        """
        try:
            with Image.open(image_path) as img:
                # 转换为RGB模式（兼容不支持透明通道的模型）
                if img.mode != 'RGB':
                    img = img.convert('RGB')

                # 转换为字节流
                img_byte_arr = io.BytesIO()
                img.save(img_byte_arr, format='PNG')
                img_bytes = img_byte_arr.getvalue()

                # 转换为base64编码
                base64_str = base64.b64encode(img_bytes).decode('utf-8')
                return base64_str

        except Exception as e:
            print(f"[错误] 图像转换失败({image_path}): {str(e)}", file=sys.stderr)
            return None

    # 模拟阶段1处理
    print("\n=== 阶段1测试 ===")
    # 准备图片数据
    stage1_images = [
        {"role": "user", "content": "原始图像", "images": [any_image_to_base64(stage1_original)]},
        {"role": "user", "content": "深度图像", "images": [any_image_to_base64(stage1_depth)]}
    ]
    # 调用视觉模型处理图片
    vision_response = _agent.client.chat(
        model=MODEL_VISION,
        messages=stage1_images
    )
    print("[视觉模型响应]")
    for chunk in vision_response:
        if isinstance(chunk, dict):
            if 'message' in chunk and 'content' in chunk['message']:
                print(chunk['message']['content'], end='', flush=True)
        elif isinstance(chunk, tuple):
            _, data = chunk
            if 'message' in data and 'content' in data['message']:
                print(data['message']['content'], end='', flush=True)
    print("\n")

    # 调用决策模型
    _agent.process_stage(stage=1, image_base64=any_image_to_base64(stage1_original),
                         depth_imgs_base64=any_image_to_base64(stage1_depth), ocr_result=stage1_ocr)

    # 模拟阶段2处理（假设识别到目标B）
    print("\n=== 阶段2测试 ===")
    # 准备图片数据
    stage2_images = [
        {"role": "user", "content": "原始图像", "images": [any_image_to_base64(stage2_original)]},
        {"role": "user", "content": "深度图像", "images": [any_image_to_base64(stage2_depth)]}
    ]
    # 调用视觉模型处理图片
    vision_response = _agent.client.chat(
        model=MODEL_VISION,
        messages=stage2_images
    )
    print("[视觉模型响应]")
    for chunk in vision_response:
        if isinstance(chunk, dict):
            if 'message' in chunk and 'content' in chunk['message']:
                print(chunk['message']['content'], end='', flush=True)
        elif isinstance(chunk, tuple):
            _, data = chunk
            if 'message' in data and 'content' in data['message']:
                print(data['message']['content'], end='', flush=True)
    print("\n")

    # 调用决策模型
    _agent.process_stage(stage=2, image_base64=any_image_to_base64(stage2_original),
                         depth_imgs_base64=any_image_to_base64(stage2_depth), ocr_result=stage2_ocr)

    # 模拟阶段3处理
    print("\n=== 阶段3测试 ===")
    # 准备图片数据
    stage3_images = [
        {"role": "user", "content": "原始图像", "images": [any_image_to_base64(stage3_original)]},
        {"role": "user", "content": "深度图像", "images": [any_image_to_base64(stage3_depth)]}
    ]
    # 调用视觉模型处理图片
    vision_response = _agent.client.chat(
        model=MODEL_VISION,
        messages=stage3_images
    )
    print("[视觉模型响应]")
    for chunk in vision_response:
        if isinstance(chunk, dict):
            if 'message' in chunk and 'content' in chunk['message']:
                print(chunk['message']['content'], end='', flush=True)
        elif isinstance(chunk, tuple):
            _, data = chunk
            if 'message' in data and 'content' in data['message']:
                print(data['message']['content'], end='', flush=True)
    print("\n")

    # 调用决策模型
    _agent.process_stage(stage=3, image_base64=any_image_to_base64(stage3_original),
                         depth_imgs_base64=any_image_to_base64(stage3_depth), ocr_result=stage3_ocr)

    # 模拟阶段4处理
    print("\n=== 阶段4测试 ===")
    # 准备图片数据
    stage4_images = [
        {"role": "user", "content": "原始图像", "images": [any_image_to_base64(stage4_original)]},
        {"role": "user", "content": "深度图像", "images": [any_image_to_base64(stage4_depth)]}
    ]
    # 调用视觉模型处理图片
    vision_response = _agent.client.chat(
        model=MODEL_VISION,
        messages=stage4_images
    )
    print("[视觉模型响应]")
    for chunk in vision_response:
        if isinstance(chunk, dict):
            if 'message' in chunk and 'content' in chunk['message']:
                print(chunk['message']['content'], end='', flush=True)
        elif isinstance(chunk, tuple):
            _, data = chunk
            if 'message' in data and 'content' in data['message']:
                print(data['message']['content'], end='', flush=True)
    print("\n")

    # 调用决策模型
    _agent.process_stage(stage=4, image_base64=any_image_to_base64(stage4_original),
                         depth_imgs_base64=any_image_to_base64(stage4_depth), ocr_result=stage4_ocr)
```

# 问题
代码:
```py
# 智慧无人机决策控制系统
# 特点: 接入ollama大模型能力, 实现智能决策
# 功能: 通过视觉大模型识别场景，使用函数调用模型进行多阶段决策和控制

# 攻击目标(与tasks中一致)
_TARGET = "B"
# 系统提示词
_SYSTEM_PROMPT = f"你是一架智慧无人机, 你的任务是找到{_TARGET}并进行攻击, 然后返回到起飞点.你可以通过深度图、OCR识别图、三维重建图进行决策.你可以使用up, down, left, right, forward, backward, rotate, attack函数.你需要不断地进行思考以应对各种情况."
# 阶段1(起飞后在第一个房间建图后决策)提示词
_PROMPT1 = "现在你在第一个房间起飞了, 悬停在1.5米高度. 刚刚使用三维重建算法对房间进行了建图, 你需要找到通往第二个房间的方向并调用函数移动."
# 阶段2(在两个房间交界处悬停决策)提示词
_PROMPT2 = "你来到了两个房间的交界处, 悬停在1.5米高度, 先侦察一下第二个房间的状况. 刚刚使用深度估计算法获取了朝向面的深度图(通过颜色区分深度), 你需要判断深度信息{_DEPTH_INFO}以判断应该如何前进, 注意避开障碍物.刚刚使用OCR算法识别目标, 结果为{_OCR_RESULT}, 你需要调用函数移动到目标或者前进以继续搜索目标."
# 阶段3(在第二个房间建图后决策)提示词
_PROMPT3 = "刚刚对第二个房间建图成功, 发现目标方位{_OCR_RESULT}, 应该马上调用函数移动到目标进行攻击.移动时注意深度信息进行避障."
# 阶段4(在目标前判断是否攻击)提示词
_PROMPT4 = f"你来到了目标{_TARGET}前面, 在有效攻击范围内, 使用函数对其进行攻击, 然后原路撤离, 回到起飞点降落."
# 对话记录
_CHAT_HISTORY = [{"q":"", "a":""},]
# 视觉大模型
MODEL_VISION = 'minicpm-v:latest'  
# 函数调用模型
MODEL_TOOL = 'MFDoom/deepseek-r1-tool-calling:1.5b'  
#####################################
# 大模型接口
from ollama import Client
# python版本兼容
from typing_extensions import MutableMapping 
# 单元测试
import pytest
# 图像处理
from PIL import Image
# 内置库
import time
import json
import requests
import base64
import io
import os
import sys
#####################################

# 输入: 原图, 深度图, ocr结果
# 输出: 控制指令 up, down, forward, backward, left, right, rotate, attack

class _Ollama():
    def __init__(self, robot, controller):
        # 初始化ollama客户端
        self.client = Client(host='http://10.8.8.130:11434')
        # 用于函数调用的精简函数集合
        self.fn = _Move(robot, controller)
        # 添加完成数据的prompt
        self.prompt = ["", ]
        # 当前决策阶段
        self.current_stage = 1
        # 对话历史记录
        self.chat_history = []
        # 起飞前预加载模型
        self._preload_models()

    def _preload_models(self):
        """预加载并保持模型常驻内存"""
        try:
            print("[系统] 开始预加载AI模型...")
            # 预加载视觉模型
            requests.post(
                'http://10.8.8.130:11434/api/generate',
                json={'model': MODEL_VISION, 'prompt': ' ', 'keep_alive': -1}
            )
            # 预加载函数调用模型
            requests.post(
                'http://10.8.8.130:11434/api/generate', 
                json={'model': MODEL_TOOL, 'prompt': ' ', 'keep_alive': -1}
            )
            print("[系统] 模型预加载完成")
        except Exception as e:
            print(f"[错误] 模型预加载失败: {str(e)}")
            raise

    # 对外接口, 预处理prompt
    def preprecess_prompt(self, *args, **kwargs):
        '''
        接受任意类型和任意数量的参数, 转换为str并格式化到prompt中
        
        参数:
            *args: 位置参数列表
            **kwargs: 关键字参数字典
            
        返回:
            str: 格式化后的prompt字符串
        '''
        prompt_parts = []
        
        # 处理位置参数
        for i, arg in enumerate(args):
            if isinstance(arg, (list, dict)):
                # 对列表和字典使用json格式化
                prompt_parts.append(f"参数{i+1}:\n{json.dumps(arg, indent=2, ensure_ascii=False)}")
            else:
                # 其他类型直接转字符串
                prompt_parts.append(f"参数{i+1}: {str(arg)}")
        
        # 处理关键字参数
        for key, value in kwargs.items():
            if isinstance(value, (list, dict)):
                # 对列表和字典使用json格式化
                prompt_parts.append(f"{key}:\n{json.dumps(value, indent=2, ensure_ascii=False)}")
            else:
                # 其他类型直接转字符串
                prompt_parts.append(f"{key}: {str(value)}")
        
        # 将所有部分用换行符连接
        formatted_prompt = "\n".join(prompt_parts)
        
        # 更新当前prompt
        if hasattr(self, 'prompt'):
            self.prompt.append(formatted_prompt)
        else:
            self.prompt = [formatted_prompt]
            
        return formatted_prompt
    
    # 对外接口, 每个阶段明确思考内容
    def process_stage(self, stage=0, image_base64=None, depth_imgs_base64=None, ocr_result=None):
        """
        处理当前任务阶段
        参数:
            image_base64: 原始视觉图像(图像base64数据)
            depth_imgs_base64: 深度估计结果png_base64
            ocr_result: OCR识别结果json
        返回:
            dict: 控制指令
        """
        try:
            # 根据阶段处理提示词
            if stage == 1:    
                prompt = _PROMPT1
            elif stage == 2:
                # 将OCR结果格式化进入
                prompt = _PROMPT2.format(_OCR_RESULT=_OCR_RESULT)
            elif stage == 3:
                prompt = _PROMPT3.format(_OCR_RESULT=_OCR_RESULT)
            elif stage == 4:
                prompt = _PROMPT4
            else: 
                prompt = f"你是自主侦察无人机, 你的任务是在室内搜索{_TARGET}并攻击."

            # 1. 调用视觉模型分析图像
            prompt_vision = "分析图片深度信息"
            vision_result = ""
            # 图像列表
            image_base64_list = []
            if image_base64 != None:
                image_base64_list.append(image_base64)
            if depth_imgs_base64 != None:
                image_base64_list.append(depth_imgs_base64)
            image_bytes_list = []

            # 转换为字节数组
            image_bytes_list = [base64.b64decode(img) for img in image_base64_list]
            
            try:
                print("\n[系统] 开始视觉深度分析处理...")
                
                # 假设depth_imgs_base64是已经准备好的base64编码图像数据列表
                # 将base64字符串转换为字节数据
                image_data = [base64.b64decode(img) for img in depth_imgs_base64]
                
                # 调用视觉模型进行深度分析，保持模型常驻
                stream = client.generate(
                    model=MODEL_VISION,
                    prompt=prompt_vision,
                    images=image_bytes_list,
                    stream=True,
                    keep_alive=-1  # 保持模型常驻内存
                )
                
                # 收集和处理响应
                full_response = []
                print("[系统] 已提交视觉深度分析请求，等待响应...")
                
                for chunk in stream:
                    print(chunk['response'], end='', flush=True)
                    full_response.append(chunk['response'])
                
                # 获取完整的深度分析结果
                vision_result = ''.join(full_response)
                
                # 记录分析结果到日志
                with open('./result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
                    log_file.write(vision_result)
                    log_file.write('\n')
                    
            except Exception as e:
                print(f"[错误] 视觉深度分析失败: {str(e)}")
                raise

            # 将视觉大模型返回的结果添加到prompt中
            this_prompt = ''
            try:
                this_prompt = prompt.format(_DEPTH_INFO=vision_result)
            except Exception as e:
                print("可能不需要深度信息")

            # 构建对话消息
            messages = [
                # 角色提示词
                {"role": "system", "content": _SYSTEM_PROMPT},
                # 对话历史
                *self._format_history(),
                # 阶段提示词
                {"role": "user", "content": this_prompt}
            ]
            
            # 2. 调用函数调用模型进行决策
            response = self.client.chat(
                model=MODEL_TOOL,
                messages=messages,
                tools=[self.fn.up, self.fn.down, self.fn.left, 
                      self.fn.right, self.fn.forward, self.fn.backward,
                      self.fn.rotate, self.fn.attack],
                stream=True
            )
            
            # 处理响应
            tool_calls = []
            full_response = []
            for chunk in response:
                if 'message' in chunk:
                    msg = chunk['message']
                    # 收集文本响应
                    if 'content' in msg:
                        print(msg['content'], end='', flush=True)
                        full_response.append(msg['content'])
                    # 收集函数调用
                    if 'tool_calls' in msg:
                        tool_calls.extend(msg['tool_calls'])
            
            # 执行函数调用
            if tool_calls:
                print("\n[系统] 执行函数调用...")
                for call in tool_calls:
                    self._execute_function_call(call)
            
            # 更新对话历史
            self._update_history(prompt, ''.join(full_response))
            
            # 检查阶段是否完成
            self._check_stage_completion()
            
            return {"status": "success", "stage": self.current_stage}
            
        except Exception as e:
            print(f"[错误] 决策处理失败: {str(e)}")
            return {"status": "error", "message": str(e)}
    
    def _execute_function_call(self, call):
        """执行函数调用"""
        func_name = call['function']['name']
        args = call['function']['arguments']
        
        try:
            # 转换参数格式
            if isinstance(args, str):
                args = json.loads(args.replace("'", '"'))
                
            print(f"[执行] {func_name}({args})")
            
            # 调用对应函数
            func = getattr(self.fn, func_name)
            if func_name == "attack":
                func()  # 无参数
            else:
                # 运动函数参数处理
                arg_name = 'cm' if func_name != 'rotate' else 'degrees'
                value = args.get(arg_name, 0)
                
                # 处理字符串值中的单位
                if isinstance(value, str):
                    if 'degrees' in value:
                        value = value.replace('degrees', '').strip()
                    elif 'cm' in value:
                        value = value.replace('cm', '').strip()
                
                # 转换为float
                value = float(value)
                
                func(value)
                
        except Exception as e:
            print(f"[错误] 函数执行失败: {str(e)}")
            raise
    
    def _format_history(self):
        """格式化对话历史"""
        return [
            dict_obj
            for item in self.chat_history[-20:]  # 保留最近20条
            for dict_obj in (
                {"role": "user", "content": item["q"]},
                {"role": "assistant", "content": item["a"]}
            )
        ]
    
    def _update_history(self, question, answer):
        """更新对话历史"""
        self.chat_history.append({"q": question, "a": answer})
        # 打印最后一条记录
        print("最新对话记录:", self.chat_history[-1])
        
    def _check_stage_completion(self):
        """检查并更新任务阶段"""
        # 简单决策树：根据当前状态和响应判断是否进入下一阶段
        if self.current_stage == 1 and "move" in str(self.chat_history[-1]["a"]):
            pass
            # self.current_stage = 2
        elif self.current_stage == 2 and _TARGET in str(self.chat_history[-1]["a"]):
            pass
            # self.current_stage = 3
        elif self.current_stage == 3 and "attack" in str(self.chat_history[-1]["a"]):
            pass
            # self.current_stage = 4
        elif self.current_stage == 4 and "move" in str(self.chat_history[-1]["a"]):
            pass
            print("[系统] 任务完成！")
            # self.current_stage = 1  # 重置

#####################################
class _Move():
    '''给大模型函数调用(Function Call)的精简函数'''
    def __init__(self, robot, controller):
        self.controller = controller
        self.robot = robot
        self.drone = self.robot
    def up(self, cm):
        '''上升'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_up(cm)
            print(f"函数up:{cm}厘米")
    def down(self, cm):
        '''下降'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_down(cm)
            print(f"函数down:{cm}厘米")
    def left(self, cm):
        '''左移'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_left(cm)
            print(f"函数left:{cm}厘米")
    def right(self, cm):
        '''右移'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_right(cm)
            print(f"函数right:{cm}厘米")
    def forward(self, cm):
        '''前进'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_forward(cm)
            print(f"函数forward:{cm}厘米")
    def backward(self, cm):
        '''后退'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_backward(cm)
            print(f"函数backward:{cm}厘米")
    def rotate(self, degrees):
        '''旋转'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.rotate(degrees)
            print(f"函数rotate:{degrees}度")
    def attack(self):
        '''攻击'''
        # 打开激光
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.attack(True)
        # 等待一会
        time.sleep(5)
        # 关闭激光
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.attack(False)
        print("函数attack完成")
#####################################
# 单元测试

# 测试例程
if __name__ == "__main__":
    # 阶段1的图片
    stage1_original = "./stage_images/1.png"
    stage1_depth = "./stage_images/1_depth.webp"
    stage1_ocr = """{"boxes":[[[[764,8],[787,8],[787,33],[764,33]]]],"texts":["X"],"scores":[0.8657664656639099]}"""
    
    # 阶段2的图片
    stage2_original = "./stage_images/2.png"
    stage2_depth = "./stage_images/2_depth.webp"
    stage2_ocr = """{"boxes":[[[[763,3],[790,3],[790,31],[763,31]]],[[[242,229],[330,231],[327,343],[239,340]]]],"texts":["X","C"],"scores":[0.7861287593841553,0.5938434600830078]}"""
    
    # 阶段3的图片
    stage3_original = "./stage_images/3.png"
    stage3_depth = "./stage_images/3_depth.webp"
    stage3_ocr = """{"boxes":[[[[345,184],[485,180],[490,382],[350,386]]]],"texts":["B"],"scores":[0.9931768178939819]}"""

    # 阶段4的图片
    stage4_original = "./stage_images/4.png"
    stage4_depth = "./stage_images/4_depth.webp"
    stage4_ocr = "错误"

    # 测试决策
    # 模拟测试
    class MockController:
        def move_up(self, cm): pass
        def move_down(self, cm): pass
        def move_left(self, cm): pass
        def move_right(self, cm): pass
        def move_forward(self, cm): pass
        def move_backward(self, cm): pass
        def rotate(self, degrees): pass
        def laser_on(self): print("激光开启")
        def laser_off(self): print("激光关闭")
    
    _agent = _Ollama(None, MockController())

    # 辅助函数：将任意格式图像转换为PNG字节流
    def convert_to_png_bytes(image_path):
        """将任意格式图像转换为PNG格式的字节流"""
        try:
            with Image.open(image_path) as img:
                # 转换为RGB模式（兼容不支持透明通道的模型）
                if img.mode != 'RGB':
                    img = img.convert('RGB')
                
                # 转换为字节流
                img_byte_arr = io.BytesIO()
                img.save(img_byte_arr, format='PNG')
                return img_byte_arr.getvalue()
        except Exception as e:
            print(f"[错误] 图像转换失败({image_path}): {str(e)}")
            return None
        
    # 辅助函数：将图片转换为base64
    def image_to_base64(image_path):
        with open(image_path, "rb") as image_file:
            return base64.b64encode(image_file.read()).decode('utf-8')
    
    # 辅助函数: 将任意格式图片转为png格式base64
    def any_image_to_base64(image_path):
        """将任意格式图像转换为PNG格式的base64编码字符串
        
        Args:
            image_path (str): 图像文件路径
            
        Returns:
            str: base64编码的PNG图像字符串，失败时返回None
        """
        try:
            with Image.open(image_path) as img:
                # 转换为RGB模式（兼容不支持透明通道的模型）
                if img.mode != 'RGB':
                    img = img.convert('RGB')
                
                # 转换为字节流
                img_byte_arr = io.BytesIO()
                img.save(img_byte_arr, format='PNG')
                img_bytes = img_byte_arr.getvalue()
                
                # 转换为base64编码
                base64_str = base64.b64encode(img_bytes).decode('utf-8')
                return base64_str
                
        except Exception as e:
            print(f"[错误] 图像转换失败({image_path}): {str(e)}", file=sys.stderr)
            return None
    
    # 模拟阶段1处理
    print("\n=== 阶段1测试 ===")

    # 调用决策模型
    _agent.process_stage(any_image_to_base64(stage1_original), any_image_to_base64(stage1_depth), stage1_ocr)

    # 模拟阶段2处理（假设识别到目标B）
    print("\n=== 阶段2测试 ===")

    # 调用决策模型
    _agent.process_stage(any_image_to_base64(stage2_original), any_image_to_base64(stage2_depth), stage2_ocr)

    # 模拟阶段3处理
    print("\n=== 阶段3测试 ===")

    # 调用决策模型
    _agent.process_stage(any_image_to_base64(stage3_original), any_image_to_base64(stage3_depth), stage3_ocr)

    # 模拟阶段4处理
    print("\n=== 阶段4测试 ===")

    # 调用决策模型
    _agent.process_stage(any_image_to_base64(stage4_original), any_image_to_base64(stage4_depth), stage4_ocr)
```
报错:
```sh
[系统] 开始预加载AI模型...
[系统] 模型预加载完成

=== 阶段1测试 ===

[系统] 开始视觉深度分析处理...
[错误] 视觉深度分析失败: Invalid base64-encoded string: number of data characters (1) cannot be 1 more than a multiple of 4
[错误] 决策处理失败: Invalid base64-encoded string: number of data characters (1) cannot be 1 more than a multiple of 4

=== 阶段2测试 ===
[错误] 决策处理失败: Incorrect padding

=== 阶段3测试 ===
[错误] 决策处理失败: Incorrect padding

=== 阶段4测试 ===
[错误] 决策处理失败: string argument should contain only ASCII characters
```
根据报错信息修正代码;

## 回答
根据报错信息，问题主要出在base64编码处理和字符串编码上。以下是修正后的代码：

```python
# 智慧无人机决策控制系统
# 特点: 接入ollama大模型能力, 实现智能决策
# 功能: 通过视觉大模型识别场景，使用函数调用模型进行多阶段决策和控制

# 攻击目标(与tasks中一致)
_TARGET = "B"
# 系统提示词
_SYSTEM_PROMPT = f"你是一架智慧无人机, 你的任务是找到{_TARGET}并进行攻击, 然后返回到起飞点.你可以通过深度图、OCR识别图、三维重建图进行决策.你可以使用up, down, left, right, forward, backward, rotate, attack函数.你需要不断地进行思考以应对各种情况."
# 阶段1(起飞后在第一个房间建图后决策)提示词
_PROMPT1 = "现在你在第一个房间起飞了, 悬停在1.5米高度. 刚刚使用三维重建算法对房间进行了建图, 你需要找到通往第二个房间的方向并调用函数移动."
# 阶段2(在两个房间交界处悬停决策)提示词
_PROMPT2 = "你来到了两个房间的交界处, 悬停在1.5米高度, 先侦察一下第二个房间的状况. 刚刚使用深度估计算法获取了朝向面的深度图(通过颜色区分深度), 你需要判断深度信息{_DEPTH_INFO}以判断应该如何前进, 注意避开障碍物.刚刚使用OCR算法识别目标, 结果为{_OCR_RESULT}, 你需要调用函数移动到目标或者前进以继续搜索目标."
# 阶段3(在第二个房间建图后决策)提示词
_PROMPT3 = "刚刚对第二个房间建图成功, 发现目标方位{_OCR_RESULT}, 应该马上调用函数移动到目标进行攻击.移动时注意深度信息进行避障."
# 阶段4(在目标前判断是否攻击)提示词
_PROMPT4 = f"你来到了目标{_TARGET}前面, 在有效攻击范围内, 使用函数对其进行攻击, 然后原路撤离, 回到起飞点降落."
# 对话记录
_CHAT_HISTORY = [{"q":"", "a":""},]
# 视觉大模型
MODEL_VISION = 'minicpm-v:latest'  
# 函数调用模型
MODEL_TOOL = 'MFDoom/deepseek-r1-tool-calling:1.5b'  
#####################################
# 大模型接口
from ollama import Client
# python版本兼容
from typing_extensions import MutableMapping 
# 单元测试
import pytest
# 图像处理
from PIL import Image
# 内置库
import time
import json
import requests
import base64
import io
import os
import sys
#####################################

class _Ollama():
    def __init__(self, robot, controller):
        # 初始化ollama客户端
        self.client = Client(host='http://10.8.8.130:11434')
        # 用于函数调用的精简函数集合
        self.fn = _Move(robot, controller)
        # 添加完成数据的prompt
        self.prompt = ["", ]
        # 当前决策阶段
        self.current_stage = 1
        # 对话历史记录
        self.chat_history = []
        # 起飞前预加载模型
        self._preload_models()

    def _preload_models(self):
        """预加载并保持模型常驻内存"""
        try:
            print("[系统] 开始预加载AI模型...")
            # 预加载视觉模型
            requests.post(
                'http://10.8.8.130:11434/api/generate',
                json={'model': MODEL_VISION, 'prompt': ' ', 'keep_alive': -1}
            )
            # 预加载函数调用模型
            requests.post(
                'http://10.8.8.130:11434/api/generate', 
                json={'model': MODEL_TOOL, 'prompt': ' ', 'keep_alive': -1}
            )
            print("[系统] 模型预加载完成")
        except Exception as e:
            print(f"[错误] 模型预加载失败: {str(e)}")
            raise

    def preprecess_prompt(self, *args, **kwargs):
        '''
        接受任意类型和任意数量的参数, 转换为str并格式化到prompt中
        '''
        prompt_parts = []
        
        # 处理位置参数
        for i, arg in enumerate(args):
            if isinstance(arg, (list, dict)):
                prompt_parts.append(f"参数{i+1}:\n{json.dumps(arg, indent=2, ensure_ascii=False)}")
            else:
                prompt_parts.append(f"参数{i+1}: {str(arg)}")
        
        # 处理关键字参数
        for key, value in kwargs.items():
            if isinstance(value, (list, dict)):
                prompt_parts.append(f"{key}:\n{json.dumps(value, indent=2, ensure_ascii=False)}")
            else:
                prompt_parts.append(f"{key}: {str(value)}")
        
        formatted_prompt = "\n".join(prompt_parts)
        
        if hasattr(self, 'prompt'):
            self.prompt.append(formatted_prompt)
        else:
            self.prompt = [formatted_prompt]
            
        return formatted_prompt
    
    def process_stage(self, stage=0, image_base64=None, depth_imgs_base64=None, ocr_result=None):
        """
        处理当前任务阶段
        """
        try:
            # 根据阶段处理提示词
            if stage == 1:    
                prompt = _PROMPT1
            elif stage == 2:
                prompt = _PROMPT2.format(_OCR_RESULT=ocr_result)
            elif stage == 3:
                prompt = _PROMPT3.format(_OCR_RESULT=ocr_result)
            elif stage == 4:
                prompt = _PROMPT4
            else: 
                prompt = f"你是自主侦察无人机, 你的任务是在室内搜索{_TARGET}并攻击."

            # 1. 调用视觉模型分析图像
            prompt_vision = "分析图片深度信息"
            vision_result = ""
            
            # 准备图像数据
            image_bytes_list = []
            if image_base64:
                try:
                    image_bytes_list.append(base64.b64decode(image_base64.encode('ascii')))
                except Exception as e:
                    print(f"[警告] 原始图像解码失败: {str(e)}")
            
            if depth_imgs_base64:
                try:
                    if isinstance(depth_imgs_base64, list):
                        for img in depth_imgs_base64:
                            image_bytes_list.append(base64.b64decode(img.encode('ascii')))
                    else:
                        image_bytes_list.append(base64.b64decode(depth_imgs_base64.encode('ascii')))
                except Exception as e:
                    print(f"[警告] 深度图像解码失败: {str(e)}")

            # 调用视觉模型进行深度分析
            if image_bytes_list:
                try:
                    print("\n[系统] 开始视觉深度分析处理...")
                    
                    stream = self.client.generate(
                        model=MODEL_VISION,
                        prompt=prompt_vision,
                        images=image_bytes_list,
                        stream=True,
                        keep_alive=-1
                    )
                    
                    full_response = []
                    print("[系统] 已提交视觉深度分析请求，等待响应...")
                    
                    for chunk in stream:
                        if 'response' in chunk:
                            print(chunk['response'], end='', flush=True)
                            full_response.append(chunk['response'])
                    
                    vision_result = ''.join(full_response)
                    
                    with open('./result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
                        log_file.write(vision_result)
                        log_file.write('\n')
                        
                except Exception as e:
                    print(f"[错误] 视觉深度分析失败: {str(e)}")
                    raise

            # 将视觉大模型返回的结果添加到prompt中
            this_prompt = ''
            try:
                this_prompt = prompt.format(_DEPTH_INFO=vision_result)
            except:
                this_prompt = prompt

            # 构建对话消息
            messages = [
                {"role": "system", "content": _SYSTEM_PROMPT},
                *self._format_history(),
                {"role": "user", "content": this_prompt}
            ]
            
            # 2. 调用函数调用模型进行决策
            response = self.client.chat(
                model=MODEL_TOOL,
                messages=messages,
                tools=[self.fn.up, self.fn.down, self.fn.left, 
                      self.fn.right, self.fn.forward, self.fn.backward,
                      self.fn.rotate, self.fn.attack],
                stream=True
            )
            
            # 处理响应
            tool_calls = []
            full_response = []
            for chunk in response:
                if 'message' in chunk:
                    msg = chunk['message']
                    if 'content' in msg:
                        print(msg['content'], end='', flush=True)
                        full_response.append(msg['content'])
                    if 'tool_calls' in msg:
                        tool_calls.extend(msg['tool_calls'])
            
            # 执行函数调用
            if tool_calls:
                print("\n[系统] 执行函数调用...")
                for call in tool_calls:
                    self._execute_function_call(call)
            
            # 更新对话历史
            self._update_history(prompt, ''.join(full_response))
            
            # 检查阶段是否完成
            self._check_stage_completion()
            
            return {"status": "success", "stage": self.current_stage}
            
        except Exception as e:
            print(f"[错误] 决策处理失败: {str(e)}")
            return {"status": "error", "message": str(e)}
    
    def _execute_function_call(self, call):
        """执行函数调用"""
        func_name = call['function']['name']
        args = call['function']['arguments']
        
        try:
            # 转换参数格式
            if isinstance(args, str):
                args = json.loads(args.replace("'", '"'))
                
            print(f"[执行] {func_name}({args})")
            
            # 调用对应函数
            func = getattr(self.fn, func_name)
            if func_name == "attack":
                func()  # 无参数
            else:
                # 运动函数参数处理
                arg_name = 'cm' if func_name != 'rotate' else 'degrees'
                value = args.get(arg_name, 0)
                
                # 处理字符串值中的单位
                if isinstance(value, str):
                    if 'degrees' in value:
                        value = value.replace('degrees', '').strip()
                    elif 'cm' in value:
                        value = value.replace('cm', '').strip()
                
                # 转换为float
                value = float(value)
                
                func(value)
                
        except Exception as e:
            print(f"[错误] 函数执行失败: {str(e)}")
            raise
    
    def _format_history(self):
        """格式化对话历史"""
        return [
            dict_obj
            for item in self.chat_history[-20:]  # 保留最近20条
            for dict_obj in (
                {"role": "user", "content": item["q"]},
                {"role": "assistant", "content": item["a"]}
            )
        ]
    
    def _update_history(self, question, answer):
        """更新对话历史"""
        self.chat_history.append({"q": question, "a": answer})
        print("最新对话记录:", self.chat_history[-1])
        
    def _check_stage_completion(self):
        """检查并更新任务阶段"""
        if self.current_stage == 1 and "move" in str(self.chat_history[-1]["a"]):
            pass
        elif self.current_stage == 2 and _TARGET in str(self.chat_history[-1]["a"]):
            pass
        elif self.current_stage == 3 and "attack" in str(self.chat_history[-1]["a"]):
            pass
        elif self.current_stage == 4 and "move" in str(self.chat_history[-1]["a"]):
            print("[系统] 任务完成！")

#####################################
class _Move():
    '''给大模型函数调用(Function Call)的精简函数'''
    def __init__(self, robot, controller):
        self.controller = controller
        self.robot = robot
        self.drone = self.robot
        
    def up(self, cm):
        '''上升'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_up(cm)
            print(f"函数up:{cm}厘米")
            
    def down(self, cm):
        '''下降'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_down(cm)
            print(f"函数down:{cm}厘米")
            
    def left(self, cm):
        '''左移'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_left(cm)
            print(f"函数left:{cm}厘米")
            
    def right(self, cm):
        '''右移'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_right(cm)
            print(f"函数right:{cm}厘米")
            
    def forward(self, cm):
        '''前进'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_forward(cm)
            print(f"函数forward:{cm}厘米")
            
    def backward(self, cm):
        '''后退'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_backward(cm)
            print(f"函数backward:{cm}厘米")
            
    def rotate(self, degrees):
        '''旋转'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.rotate(degrees)
            print(f"函数rotate:{degrees}度")
            
    def attack(self):
        '''攻击'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.attack(True)
        time.sleep(5)
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.attack(False)
        print("函数attack完成")

#####################################
# 单元测试

if __name__ == "__main__":
    # 阶段1的图片
    stage1_original = "./stage_images/1.png"
    stage1_depth = "./stage_images/1_depth.webp"
    stage1_ocr = """{"boxes":[[[[764,8],[787,8],[787,33],[764,33]]]],"texts":["X"],"scores":[0.8657664656639099]}"""
    
    # 阶段2的图片
    stage2_original = "./stage_images/2.png"
    stage2_depth = "./stage_images/2_depth.webp"
    stage2_ocr = """{"boxes":[[[[763,3],[790,3],[790,31],[763,31]]],[[[242,229],[330,231],[327,343],[239,340]]]],"texts":["X","C"],"scores":[0.7861287593841553,0.5938434600830078]}"""
    
    # 阶段3的图片
    stage3_original = "./stage_images/3.png"
    stage3_depth = "./stage_images/3_depth.webp"
    stage3_ocr = """{"boxes":[[[[345,184],[485,180],[490,382],[350,386]]]],"texts":["B"],"scores":[0.9931768178939819]}"""

    # 阶段4的图片
    stage4_original = "./stage_images/4.png"
    stage4_depth = "./stage_images/4_depth.webp"
    stage4_ocr = "错误"

    # 测试决策
    class MockController:
        def move_up(self, cm): pass
        def move_down(self, cm): pass
        def move_left(self, cm): pass
        def move_right(self, cm): pass
        def move_forward(self, cm): pass
        def move_backward(self, cm): pass
        def rotate(self, degrees): pass
        def attack(self, state): 
            print("激光开启" if state else "激光关闭")
    
    _agent = _Ollama(None, MockController())

    # 辅助函数：将图片转换为base64编码的ASCII字符串
    def image_to_base64(image_path):
        try:
            with open(image_path, "rb") as image_file:
                # 读取图像数据并编码为base64 ASCII字符串
                return base64.b64encode(image_file.read()).decode('ascii')
        except Exception as e:
            print(f"[错误] 图像转换失败({image_path}): {str(e)}")
            return None

    # 模拟阶段1处理
    print("\n=== 阶段1测试 ===")
    _agent.process_stage(
        stage=1,
        image_base64=image_to_base64(stage1_original),
        depth_imgs_base64=image_to_base64(stage1_depth),
        ocr_result=stage1_ocr
    )

    # 模拟阶段2处理
    print("\n=== 阶段2测试 ===")
    _agent.process_stage(
        stage=2,
        image_base64=image_to_base64(stage2_original),
        depth_imgs_base64=image_to_base64(stage2_depth),
        ocr_result=stage2_ocr
    )

    # 模拟阶段3处理
    print("\n=== 阶段3测试 ===")
    _agent.process_stage(
        stage=3,
        image_base64=image_to_base64(stage3_original),
        depth_imgs_base64=image_to_base64(stage3_depth),
        ocr_result=stage3_ocr
    )

    # 模拟阶段4处理
    print("\n=== 阶段4测试 ===")
    _agent.process_stage(
        stage=4,
        image_base64=image_to_base64(stage4_original),
        depth_imgs_base64=image_to_base64(stage4_depth),
        ocr_result=stage4_ocr
    )
```

主要修改点：

1. 修正了base64编码处理，确保所有base64字符串都使用ASCII编码：
   - 在`image_to_base64`函数中添加了`.decode('ascii')`
   - 在图像解码时添加了`.encode('ascii')`

2. 改进了图像处理逻辑：
   - 添加了对图像数据的有效性检查
   - 处理了单张图像和多张图像的不同情况

3. 修正了OCR结果格式化：
   - 直接在提示词格式化中使用传入的ocr_result参数

4. 简化了测试代码：
   - 使用统一的`image_to_base64`函数
   - 明确指定每个测试的阶段参数

5. 改进了错误处理：
   - 添加了更详细的错误日志
   - 对可能失败的操作添加了try-catch块

这些修改应该能解决原来的base64编码问题和字符串编码问题。

# 问题
提示:
```py
@overload
  def generate(
    self,
    model: str = '',
    prompt: str = '',
    suffix: str = '',
    *,
    system: str = '',
    template: str = '',
    context: Optional[Sequence[int]] = None,
    stream: Literal[False] = False,
    raw: bool = False,
    format: Optional[Union[Literal['', 'json'], JsonSchemaValue]] = None,
    images: Optional[Sequence[Union[str, bytes, Image]]] = None,
    options: Optional[Union[Mapping[str, Any], Options]] = None,
    keep_alive: Optional[Union[float, str]] = None,
  ) -> GenerateResponse: ...
```
代码:
```py
# 1. 调用视觉模型分析图像
prompt_vision = "分析图片深度信息"
vision_result = ""
# 图像列表
image_base64_list = []
if image_base64 != None:
    image_base64_list.append(image_base64)
if depth_imgs_base64 != None:
    image_base64_list.append(depth_imgs_base64)
image_bytes_list = []

# 转换为字节数组
image_bytes_list = [base64.b64decode(img) for img in image_base64_list]

try:
    print("\n[系统] 开始视觉深度分析处理...")
    
    # 假设depth_imgs_base64是已经准备好的base64编码图像数据列表
    # 将base64字符串转换为字节数据

    # 调用视觉模型进行深度分析，保持模型常驻
    stream = self.client.generate(
        model=MODEL_VISION,
        prompt=prompt_vision,
        images=image_bytes_list,
        stream=True,
        keep_alive=-1  # 保持模型常驻内存
    )
    
    # 收集和处理响应
    full_response = []
    print("[系统] 已提交视觉深度分析请求，等待响应...")
    
    for chunk in stream:
        print(chunk['response'], end='', flush=True)
        full_response.append(chunk['response'])
    
    # 获取完整的深度分析结果
    vision_result = ''.join(full_response)
    
    # 记录分析结果到日志
    with open('./result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
        log_file.write(vision_result)
        log_file.write('\n')
        
except Exception as e:
    print(f"[错误] 视觉深度分析失败: {str(e)}")
    raise
```
报错:
```sh
[系统] 开始预加载AI模型...
[系统] 模型预加载完成

=== 阶段1测试 ===

[系统] 开始视觉深度分析处理...
[系统] 已提交视觉深度分析请求，等待响应...
[错误] 视觉深度分析失败: Failed to create new sequence: failed to process inputs: unable to make llava embedding from image
 (status code: -1)
[错误] 决策处理失败: Failed to create new sequence: failed to process inputs: unable to make llava embedding from image
 (status code: -1)
```
根据提示和报错信息修正代码;

## 回答
根据报错信息，问题出在图像处理环节，具体是"unable to make llava embedding from image"。这通常意味着图像数据格式不符合模型要求或者图像处理失败。以下是修正后的代码：

```py
# 1. 调用视觉模型分析图像
prompt_vision = "分析图片深度信息"
vision_result = ""
# 图像列表
image_list = []  # 改为更通用的image_list

if image_base64 is not None:
    try:
        img_data = base64.b64decode(image_base64)
        image_list.append(img_data)
    except Exception as e:
        print(f"[警告] 主图像base64解码失败: {str(e)}")

if depth_imgs_base64 is not None:
    try:
        if isinstance(depth_imgs_base64, (list, tuple)):
            for img in depth_imgs_base64:
                img_data = base64.b64decode(img)
                image_list.append(img_data)
        else:
            img_data = base64.b64decode(depth_imgs_base64)
            image_list.append(img_data)
    except Exception as e:
        print(f"[警告] 深度图像base64解码失败: {str(e)}")

try:
    print("\n[系统] 开始视觉深度分析处理...")
    
    if not image_list:
        raise ValueError("没有有效的图像数据可供分析")
    
    # 调用视觉模型进行深度分析
    stream = self.client.generate(
        model=MODEL_VISION,
        prompt=prompt_vision,
        images=image_list,
        stream=True,
        keep_alive=-1,  # 保持模型常驻内存
        options={
            'num_ctx': 4096  # 增加上下文长度
        }
    )
    
    # 收集和处理响应
    full_response = []
    print("[系统] 已提交视觉深度分析请求，等待响应...")
    
    for chunk in stream:
        if 'response' in chunk:
            print(chunk['response'], end='', flush=True)
            full_response.append(chunk['response'])
    
    # 获取完整的深度分析结果
    vision_result = ''.join(full_response)
    
    # 记录分析结果到日志
    os.makedirs('./result', exist_ok=True)  # 确保目录存在
    with open('./result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
        log_file.write(vision_result)
        log_file.write('\n')
        
except Exception as e:
    print(f"[错误] 视觉深度分析失败: {str(e)}")
    # 可以选择是否重新抛出异常
    # raise
```

主要修改点：

1. 改进了图像处理逻辑，增加了错误处理和类型检查
2. 确保图像数据是有效的字节数据
3. 增加了对图像列表为空情况的检查
4. 添加了创建结果目录的逻辑
5. 在generate调用中添加了options参数以增加上下文长度
6. 改进了对stream响应数据的处理，增加了键存在性检查
7. 使异常处理更加友好，可以选择不重新抛出异常

如果问题仍然存在，可能需要：

1. 检查MODEL_VISION是否正确设置为支持视觉任务的模型
2. 验证图像数据是否完整且格式正确
3. 尝试减小图像尺寸或改变图像格式
4. 检查模型服务是否正常运行并有足够资源

# 问题
python的Sequence类型是什么意思?

# 问题
```py
# 1. 调用视觉模型分析图像
prompt_vision = "分析图片深度信息"
vision_result = ""
# 图像列表
image_base64_list = []
if image_base64:
    image_base64_list.append(image_base64)
if depth_imgs_base64:
    image_base64_list.append(depth_imgs_base64)
image_bytes_list = []

# 转换为字节数组
# image_bytes_list = [base64.b64decode(img) for img in image_base64_list]

try:
    print("\n[系统] 开始视觉深度分析处理...")
    
    # 假设image_base64_list是已经准备好的base64编码图像数据列表

    # 调用视觉模型进行深度分析，保持模型常驻
    stream = self.client.generate(
        model=MODEL_VISION,
        prompt=prompt_vision,
        images=[image_base64],
        stream=True,
        keep_alive=-1  # 保持模型常驻内存
    )
```
将`image_base64_list`的图片保存到临时文件夹并在generate的images中传入临时图片路径的列表;

## 回答
要将`image_base64_list`中的图片保存到临时文件夹并在`generate`的`images`参数中传入临时图片路径列表，你可以这样做：

```py
import os
import base64
import tempfile

# 1. 调用视觉模型分析图像
prompt_vision = "分析图片深度信息"
vision_result = ""
# 图像列表
image_base64_list = []
if image_base64:
    image_base64_list.append(image_base64)
if depth_imgs_base64:
    image_base64_list.append(depth_imgs_base64)

try:
    print("\n[系统] 开始视觉深度分析处理...")
    
    # 创建临时目录
    temp_dir = tempfile.mkdtemp()
    image_paths = []
    
    # 将base64图像保存到临时文件
    for i, img_base64 in enumerate(image_base64_list):
        # 解码base64数据
        img_data = base64.b64decode(img_base64)
        
        # 创建临时文件路径 (可以根据需要添加文件扩展名，如.jpg/.png)
        temp_path = os.path.join(temp_dir, f"temp_image_{i}.jpg")
        
        # 写入文件
        with open(temp_path, 'wb') as f:
            f.write(img_data)
        
        image_paths.append(temp_path)
    
    # 调用视觉模型进行深度分析，传入文件路径列表
    stream = self.client.generate(
        model=MODEL_VISION,
        prompt=prompt_vision,
        images=image_paths,  # 传入文件路径列表
        stream=True,
        keep_alive=-1  # 保持模型常驻内存
    )
    
    # 使用完后可以删除临时文件（可选）
    for path in image_paths:
        try:
            os.remove(path)
        except:
            pass
    try:
        os.rmdir(temp_dir)
    except:
        pass

except Exception as e:
    print(f"[系统] 视觉深度分析处理出错: {e}")
```

注意事项：
1. 这段代码使用了Python的`tempfile`模块创建临时目录
2. 临时文件默认保存为.jpg格式，如果原始图像是其他格式可能需要调整
3. 代码最后包含了清理临时文件的逻辑（可选）
4. 如果API需要特定的图像格式，请确保保存时使用正确的文件扩展名
5. 根据实际需求，你可能需要保留临时文件直到处理完成后再删除

# 问题
示例正确代码:
```py
# 无人机视觉识别与自主决策控制系统
# 功能：通过视觉大模型识别场景，然后使用函数调用模型进行决策和控制

import ollama
from ollama import Client
from typing_extensions import MutableMapping 
import time
import json
import requests  # 用于直接调用API接口

# 系统常量定义
PROMPT_VISION = '图片里面有几个字母？描述字母相对无人机的位置关系, 无人机更靠近哪个字母?'  # 视觉识别提示词
PROMPT_DECISION = '攻击目标为B, 根据相对关系移动无人机到B前方'  # 决策提示词
MODEL_VISION = 'minicpm-v:latest'  # 视觉大模型
MODEL_TOOL = 'MFDoom/deepseek-r1-tool-calling:1.5b'  # 函数调用模型

# 初始化Ollama客户端
client = Client(host='http://10.8.8.130:11434')

def preload_models():
    """
    预加载模型到内存中并保持常驻
    """
    print("\n[系统] 开始预加载模型并保持常驻内存...")
    
    try:
        # 预加载视觉模型并保持常驻
        print("[系统] 正在预加载视觉模型并保持常驻...")
        response = requests.post(
            'http://10.8.8.130:11434/api/generate',
            json={
                'model': MODEL_VISION,
                'prompt': ' ',  # 空提示
                'keep_alive': -1  # 保持模型常驻内存
            }
        )
        response.raise_for_status()
        
        # 预加载函数调用模型并保持常驻
        print("[系统] 正在预加载函数调用模型并保持常驻...")
        response = requests.post(
            'http://10.8.8.130:11434/api/generate',
            json={
                'model': MODEL_TOOL,
                'prompt': ' ',  # 空提示
                'keep_alive': -1  # 保持模型常驻内存
            }
        )
        response.raise_for_status()
        
        print("[系统] 模型预加载完成并保持常驻内存")
    except Exception as e:
        print(f"[错误] 模型预加载失败: {str(e)}")
        raise

def check_model_status():
    """
    检查模型是否已加载
    """
    try:
        response = requests.get('http://10.8.8.130:11434/api/tags')
        response.raise_for_status()
        models = [model['name'] for model in response.json()['models']]
        
        print("\n[系统] 当前已加载模型:")
        for model in models:
            print(f" - {model}")
            
        return MODEL_VISION in models and MODEL_TOOL in models
    except Exception as e:
        print(f"[错误] 检查模型状态失败: {str(e)}")
        return False

def vision_recognition(image_path):
    """
    视觉识别处理函数
    参数:
        image_path: 图片文件路径
    返回:
        str: 视觉模型返回的场景描述
    """
    print("\n[系统] 开始视觉识别处理...")
    
    try:
        # 读取图片文件
        with open(image_path, 'rb') as f:
            image_data = f.read()
            
            # 调用视觉模型进行识别，保持模型常驻
            stream = client.generate(
                model=MODEL_VISION,
                prompt=PROMPT_VISION,
                images=[image_data],
                stream=True,
                keep_alive=-1  # 保持模型常驻内存
            )
            
            # 收集和处理响应
            full_response = []
            print("[系统] 已提交视觉识别请求，等待响应...")
            
            for chunk in stream:
                print(chunk['response'], end='', flush=True)
                full_response.append(chunk['response'])
            
            # 返回完整的场景描述
            return ''.join(full_response)
    except Exception as e:
        print(f"[错误] 视觉识别失败: {str(e)}")
        raise

def decision_making(scene_description):
    """
    决策和函数调用处理
    参数:
        scene_description: 视觉模型返回的场景描述
    """
    print("\n[系统] 开始决策和函数调用处理...")
    
    try:
        # 准备对话消息
        messages = [
            {'role': 'system', 'content': '你是一个无人机控制系统，需要根据视觉描述做出决策'},
            {'role': 'user', 'content': scene_description},
            {'role': 'user', 'content': PROMPT_DECISION}
        ]
        
        # 调用函数模型，保持模型常驻
        response = client.chat(
            model=MODEL_TOOL,
            messages=messages,
            tools=[move],  # 注册可调用的函数
            stream=True,
            keep_alive=-1  # 保持模型常驻内存
        )
        
        # 处理响应
        full_response = []
        tool_calls = []
        print("[系统] 已提交决策请求，等待响应...")
        
        for chunk in response:
            if 'message' in chunk:
                message = chunk['message']
                
                # 收集文本响应
                if 'content' in message and message['content']:
                    content = message['content']
                    print(content, end='', flush=True)
                    full_response.append(content)
                
                # 收集函数调用请求
                if 'tool_calls' in message:
                    tool_calls.extend(message['tool_calls'])
        
        # 处理函数调用
        if tool_calls:
            print("\n[系统] 检测到函数调用请求")
            for tool_call in tool_calls:
                func_name = tool_call['function']['name']
                args = tool_call['function']['arguments']
                
                print(f"[系统] 调用函数: {func_name}, 参数: {args}")

                # 执行对应的函数
                if func_name == "move":
                    try:
                        # 参数已经是字典格式，直接处理
                        if isinstance(args, str):
                            # 如果是字符串，先转换为字典
                            args = json.loads(args.replace("'", '"'))
                        
                        # 确保所有值为数值类型
                        numeric_args = {
                            'duration': float(args.get('duration', 10)),
                            'forward': float(args.get('forward', 0)),
                            'backward': float(args.get('backward', 0)),
                            'left': float(args.get('left', 0)),
                            'right': float(args.get('right', 0)),
                            'up': float(args.get('up', 0)),
                            'down': float(args.get('down', 0))
                        }
                        move(**numeric_args)
                    except Exception as e:
                        print(f"[错误] 函数调用参数处理失败: {str(e)}")
                        raise
        
        # 记录完整日志
        with open('../result/ollama_decision.log', 'a', encoding='utf-8') as log_file:
            log_file.write(''.join(full_response))
            log_file.write('\n')
    except Exception as e:
        print(f"[错误] 决策处理失败: {str(e)}")
        raise

def move(duration=10.0, forward=0.0, backward=0.0, left=0.0, right=0.0, up=0.0, down=0.0):
    """
    无人机运动控制函数
    参数:
        duration: 运动持续时间(秒) 默认10秒
        forward: 向前速度(0~1) 默认0
        backward: 向后速度(0~1) 默认0
        left: 向左速度(0~1) 默认0
        right: 向右速度(0~1) 默认0
        up: 向上速度(0~1) 默认0
        down: 向下速度(0~1) 默认0
    """
    try:
        # 验证参数范围
        duration = max(0.1, min(float(duration), 60))  # 限制在0.1-60秒之间
        forward = max(0, min(float(forward), 1))
        backward = max(0, min(float(backward), 1))
        left = max(0, min(float(left), 1))
        right = max(0, min(float(right), 1))
        up = max(0, min(float(up), 1))
        down = max(0, min(float(down), 1))
        
        # 计算各轴速度
        velocity_x = forward - backward
        velocity_y = right - left
        velocity_z = down - up
        
        # 打印运动指令
        print(f"\n[控制] 执行运动指令:")
        print(f"时长: {duration}秒")
        print(f"X轴速度: {velocity_x:.2f} (前+/后-)")
        print(f"Y轴速度: {velocity_y:.2f} (右+/左-)")
        print(f"Z轴速度: {velocity_z:.2f} (下+/上-)")
        
    except Exception as e:
        print(f"[错误] 运动控制参数无效: {str(e)}")
        raise

# 主程序
if __name__ == "__main__":
    try:
        # 0. 预加载模型并保持常驻
        preload_models()
        
        # 检查模型状态
        if not check_model_status():
            print("[警告] 部分模型未正确加载，尝试重新加载...")
            preload_models()
        
        # 1. 视觉识别阶段
        image_path = './2.png'  # 图片路径
        scene_description = vision_recognition(image_path)
        
        # 2. 决策和控制阶段
        decision_making(scene_description)
        
        print("\n[系统] 任务执行完成")
    except Exception as e:
        print(f"[系统] 任务执行失败: {str(e)}")
```
报错代码:
```py
class _Ollama():
    def __init__(self, robot, controller):
        # 初始化ollama客户端
        self.client = Client(host='http://10.8.8.130:11434')
        # 用于函数调用的精简函数集合
        self.fn = _Move(robot, controller)
        # 添加完成数据的prompt
        self.prompt = ["", ]
        # 当前决策阶段
        self.current_stage = 1
        # 对话历史记录
        self.chat_history = []
        # 起飞前预加载模型
        self._preload_models()
    #########################################
    def _preload_models(self):
        """预加载并保持模型常驻内存"""
        try:
            print("[系统] 开始预加载AI模型...")
            # 预加载视觉模型
            requests.post(
                'http://10.8.8.130:11434/api/generate',
                json={'model': MODEL_VISION, 'prompt': ' ', 'keep_alive': -1}
            )
            # 预加载函数调用模型
            requests.post(
                'http://10.8.8.130:11434/api/generate', 
                json={'model': MODEL_TOOL, 'prompt': ' ', 'keep_alive': -1}
            )
            print("[系统] 模型预加载完成")
        except Exception as e:
            print(f"[错误] 模型预加载失败: {str(e)}")
            raise
    #########################################
    # 对外接口, 预处理prompt
    def preprecess_prompt(self, *args, **kwargs):
        '''
        接受任意类型和任意数量的参数, 转换为str并格式化到prompt中
        
        参数:
            *args: 位置参数列表
            **kwargs: 关键字参数字典
            
        返回:
            str: 格式化后的prompt字符串
        '''
        prompt_parts = []
        
        # 处理位置参数
        for i, arg in enumerate(args):
            if isinstance(arg, (list, dict)):
                # 对列表和字典使用json格式化
                prompt_parts.append(f"参数{i+1}:\n{json.dumps(arg, indent=2, ensure_ascii=False)}")
            else:
                # 其他类型直接转字符串
                prompt_parts.append(f"参数{i+1}: {str(arg)}")
        
        # 处理关键字参数
        for key, value in kwargs.items():
            if isinstance(value, (list, dict)):
                # 对列表和字典使用json格式化
                prompt_parts.append(f"{key}:\n{json.dumps(value, indent=2, ensure_ascii=False)}")
            else:
                # 其他类型直接转字符串
                prompt_parts.append(f"{key}: {str(value)}")
        
        # 将所有部分用换行符连接
        formatted_prompt = "\n".join(prompt_parts)
        
        # 更新当前prompt
        if hasattr(self, 'prompt'):
            self.prompt.append(formatted_prompt)
        else:
            self.prompt = [formatted_prompt]
            
        return formatted_prompt
    #########################################
    # 对外接口, 每个阶段明确思考内容
    def process_stage(self, stage=0, image_base64=None, depth_imgs_base64=None, ocr_result=None):
        """
        处理当前任务阶段
        参数:
            image_base64: 原始视觉图像(图像base64数据)
            depth_imgs_base64: 深度估计结果png_base64
            ocr_result: OCR识别结果json
        返回:
            dict: 控制指令
        """
        try:
            # 根据阶段处理提示词
            if stage == 1:    
                prompt = _PROMPT1
            elif stage == 2:
                # 将OCR结果格式化进入
                prompt = _PROMPT2.format(_OCR_RESULT=ocr_result)
            elif stage == 3:
                prompt = _PROMPT3.format(_OCR_RESULT=ocr_result)
            elif stage == 4:
                prompt = _PROMPT4
            else: 
                prompt = f"你是自主侦察无人机, 你的任务是在室内搜索{_TARGET}并攻击."

            # 1. 调用视觉模型分析图像
            prompt_vision = "分析图片深度信息"
            vision_result = ""
            # 图像列表
            image_base64_list = []
            if image_base64:
                image_base64_list.append(image_base64)
            if depth_imgs_base64:
                image_base64_list.append(depth_imgs_base64)
            image_bytes_list = []

            # 转换为字节数组
            # image_bytes_list = [base64.b64decode(img) for img in image_base64_list]
            
            try:
                print("\n[系统] 开始视觉深度分析处理...")
                
                # 假设image_base64_list是已经准备好的base64编码图像数据列表

                # 创建临时目录
                temp_dir = tempfile.mkdtemp()
                image_paths = []
                image_data_list = []

                # 将base64图像保存到临时文件
                for i, img_base64 in enumerate(image_base64_list):
                    # 解码base64数据
                    img_data = base64.b64decode(img_base64)
                    
                    # 创建临时文件路径 (可以根据需要添加文件扩展名，如.jpg/.png)
                    temp_path = os.path.join(temp_dir, f"temp_image_{i}.jpg")
                    
                    # 写入文件
                    with open(temp_path, 'wb') as f:
                        f.write(img_data)
                    
                    image_paths.append(temp_path)
                    with open(temp_path, 'rb') as f:
                        image_data_list.append(f.read())
                        
                    print(image_paths)

                # 调用视觉模型进行深度分析，保持模型常驻
                stream = self.client.generate(
                    model=MODEL_VISION,
                    prompt=prompt_vision,
                    images=image_data_list,
                    stream=True,
                    keep_alive=-1  # 保持模型常驻内存
                )

                # 收集和处理响应
                full_response = []
                print("[系统] 已提交视觉深度分析请求，等待响应...")
                
                for chunk in stream:
                    print(chunk['response'], end='', flush=True)
                    full_response.append(chunk['response'])
                
                # 获取完整的深度分析结果
                vision_result = ''.join(full_response)

                exit(0)
                
                # 记录分析结果到日志
                with open('./result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
                    log_file.write(vision_result)
                    log_file.write('\n')
                    
            except Exception as e:
                print(f"[错误] 视觉深度分析失败: {str(e)}")
                raise

            # 将视觉大模型返回的结果添加到prompt中
            this_prompt = ''
            try:
                this_prompt = prompt.format(_DEPTH_INFO=vision_result)
            except Exception as e:
                print("可能不需要深度信息")

            # 构建对话消息
            messages = [
                # 角色提示词
                {"role": "system", "content": _SYSTEM_PROMPT},
                # 对话历史
                *self._format_history(),
                # 阶段提示词
                {"role": "user", "content": this_prompt}
            ]
            
            # 2. 调用函数调用模型进行决策
            response = self.client.chat(
                model=MODEL_TOOL,
                messages=messages,
                tools=[self.fn.up, self.fn.down, self.fn.left, 
                      self.fn.right, self.fn.forward, self.fn.backward,
                      self.fn.rotate, self.fn.attack],
                stream=True
            )
            
            # 处理响应
            tool_calls = []
            full_response = []
            for chunk in response:
                if 'message' in chunk:
                    msg = chunk['message']
                    # 收集文本响应
                    if 'content' in msg:
                        print(msg['content'], end='', flush=True)
                        full_response.append(msg['content'])
                    # 收集函数调用
                    if 'tool_calls' in msg:
                        tool_calls.extend(msg['tool_calls'])
            
            # 执行函数调用
            if tool_calls:
                print("\n[系统] 执行函数调用...")
                for call in tool_calls:
                    self._execute_function_call(call)
            
            # 更新对话历史
            self._update_history(prompt, ''.join(full_response))
            
            # 检查阶段是否完成
            self._check_stage_completion()
            
            return {"status": "success", "stage": self.current_stage}
            
        except Exception as e:
            print(f"[错误] 决策处理失败: {str(e)}")
            return {"status": "error", "message": str(e)}
```
报错:
```sh
[系统] 开始预加载AI模型...
[系统] 模型预加载完成

=== 阶段1测试 ===

[系统] 开始视觉深度分析处理...
['/var/folders/yh/1_1fh4_94gj8_v1p5lkv61r40000gn/T/tmp9tu2_gex/temp_image_0.jpg']
['/var/folders/yh/1_1fh4_94gj8_v1p5lkv61r40000gn/T/tmp9tu2_gex/temp_image_0.jpg', '/var/folders/yh/1_1fh4_94gj8_v1p5lkv61r40000gn/T/tmp9tu2_gex/temp_image_1.jpg']
[系统] 已提交视觉深度分析请求，等待响应...
[错误] 视觉深度分析失败: Failed to create new sequence: failed to process inputs: unable to make llava embedding from image
 (status code: -1)
[错误] 决策处理失败: Failed to create new sequence: failed to process inputs: unable to make llava embedding from image
 (status code: -1)
```
根据示例代码修正报错代码, 函数参数不能变;

## 回答
根据示例代码和报错信息，问题主要出现在视觉模型处理图像的部分。以下是修正后的代码，保持原有函数参数不变：

```python
class _Ollama():
    def __init__(self, robot, controller):
        # 初始化ollama客户端
        self.client = Client(host='http://10.8.8.130:11434')
        # 用于函数调用的精简函数集合
        self.fn = _Move(robot, controller)
        # 添加完成数据的prompt
        self.prompt = ["", ]
        # 当前决策阶段
        self.current_stage = 1
        # 对话历史记录
        self.chat_history = []
        # 起飞前预加载模型
        self._preload_models()
    
    #########################################
    def _preload_models(self):
        """预加载并保持模型常驻内存"""
        try:
            print("[系统] 开始预加载AI模型...")
            # 预加载视觉模型
            requests.post(
                'http://10.8.8.130:11434/api/generate',
                json={'model': MODEL_VISION, 'prompt': ' ', 'keep_alive': -1}
            )
            # 预加载函数调用模型
            requests.post(
                'http://10.8.8.130:11434/api/generate', 
                json={'model': MODEL_TOOL, 'prompt': ' ', 'keep_alive': -1}
            )
            print("[系统] 模型预加载完成")
        except Exception as e:
            print(f"[错误] 模型预加载失败: {str(e)}")
            raise
    
    #########################################
    # 对外接口, 预处理prompt
    def preprecess_prompt(self, *args, **kwargs):
        '''
        接受任意类型和任意数量的参数, 转换为str并格式化到prompt中
        
        参数:
            *args: 位置参数列表
            **kwargs: 关键字参数字典
            
        返回:
            str: 格式化后的prompt字符串
        '''
        prompt_parts = []
        
        # 处理位置参数
        for i, arg in enumerate(args):
            if isinstance(arg, (list, dict)):
                # 对列表和字典使用json格式化
                prompt_parts.append(f"参数{i+1}:\n{json.dumps(arg, indent=2, ensure_ascii=False)}")
            else:
                # 其他类型直接转字符串
                prompt_parts.append(f"参数{i+1}: {str(arg)}")
        
        # 处理关键字参数
        for key, value in kwargs.items():
            if isinstance(value, (list, dict)):
                # 对列表和字典使用json格式化
                prompt_parts.append(f"{key}:\n{json.dumps(value, indent=2, ensure_ascii=False)}")
            else:
                # 其他类型直接转字符串
                prompt_parts.append(f"{key}: {str(value)}")
        
        # 将所有部分用换行符连接
        formatted_prompt = "\n".join(prompt_parts)
        
        # 更新当前prompt
        if hasattr(self, 'prompt'):
            self.prompt.append(formatted_prompt)
        else:
            self.prompt = [formatted_prompt]
            
        return formatted_prompt
    
    #########################################
    # 对外接口, 每个阶段明确思考内容
    def process_stage(self, stage=0, image_base64=None, depth_imgs_base64=None, ocr_result=None):
        """
        处理当前任务阶段
        参数:
            image_base64: 原始视觉图像(图像base64数据)
            depth_imgs_base64: 深度估计结果png_base64
            ocr_result: OCR识别结果json
        返回:
            dict: 控制指令
        """
        try:
            # 根据阶段处理提示词
            if stage == 1:    
                prompt = _PROMPT1
            elif stage == 2:
                # 将OCR结果格式化进入
                prompt = _PROMPT2.format(_OCR_RESULT=ocr_result)
            elif stage == 3:
                prompt = _PROMPT3.format(_OCR_RESULT=ocr_result)
            elif stage == 4:
                prompt = _PROMPT4
            else: 
                prompt = f"你是自主侦察无人机, 你的任务是在室内搜索{_TARGET}并攻击."

            # 1. 调用视觉模型分析图像
            prompt_vision = "分析图片深度信息"
            vision_result = ""
            
            # 准备图像数据列表
            image_data_list = []
            if image_base64:
                image_data_list.append(base64.b64decode(image_base64))
            if depth_imgs_base64:
                image_data_list.append(base64.b64decode(depth_imgs_base64))
            
            try:
                print("\n[系统] 开始视觉深度分析处理...")
                
                # 调用视觉模型进行深度分析，保持模型常驻
                stream = self.client.generate(
                    model=MODEL_VISION,
                    prompt=prompt_vision,
                    images=image_data_list,
                    stream=True,
                    keep_alive=-1  # 保持模型常驻内存
                )

                # 收集和处理响应
                full_response = []
                print("[系统] 已提交视觉深度分析请求，等待响应...")
                
                for chunk in stream:
                    print(chunk['response'], end='', flush=True)
                    full_response.append(chunk['response'])
                
                # 获取完整的深度分析结果
                vision_result = ''.join(full_response)
                
                # 记录分析结果到日志
                with open('./result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
                    log_file.write(vision_result)
                    log_file.write('\n')
                    
            except Exception as e:
                print(f"[错误] 视觉深度分析失败: {str(e)}")
                raise

            # 将视觉大模型返回的结果添加到prompt中
            this_prompt = ''
            try:
                this_prompt = prompt.format(_DEPTH_INFO=vision_result)
            except Exception as e:
                print("可能不需要深度信息")

            # 构建对话消息
            messages = [
                # 角色提示词
                {"role": "system", "content": _SYSTEM_PROMPT},
                # 对话历史
                *self._format_history(),
                # 阶段提示词
                {"role": "user", "content": this_prompt}
            ]
            
            # 2. 调用函数调用模型进行决策
            response = self.client.chat(
                model=MODEL_TOOL,
                messages=messages,
                tools=[self.fn.up, self.fn.down, self.fn.left, 
                      self.fn.right, self.fn.forward, self.fn.backward,
                      self.fn.rotate, self.fn.attack],
                stream=True,
                keep_alive=-1  # 保持模型常驻内存
            )
            
            # 处理响应
            tool_calls = []
            full_response = []
            for chunk in response:
                if 'message' in chunk:
                    msg = chunk['message']
                    # 收集文本响应
                    if 'content' in msg:
                        print(msg['content'], end='', flush=True)
                        full_response.append(msg['content'])
                    # 收集函数调用
                    if 'tool_calls' in msg:
                        tool_calls.extend(msg['tool_calls'])
            
            # 执行函数调用
            if tool_calls:
                print("\n[系统] 执行函数调用...")
                for call in tool_calls:
                    self._execute_function_call(call)
            
            # 更新对话历史
            self._update_history(prompt, ''.join(full_response))
            
            # 检查阶段是否完成
            self._check_stage_completion()
            
            return {"status": "success", "stage": self.current_stage}
            
        except Exception as e:
            print(f"[错误] 决策处理失败: {str(e)}")
            return {"status": "error", "message": str(e)}
```

主要修改点：

1. 移除了临时文件处理部分，直接使用base64解码后的图像数据
2. 简化了图像数据处理流程，直接从base64解码为字节数据
3. 在client.chat调用中添加了keep_alive=-1参数保持模型常驻
4. 移除了不必要的exit(0)调试代码
5. 保持了所有函数参数不变

这些修改应该能解决"unable to make llava embedding from image"错误，因为现在直接传递解码后的图像数据给模型，而不是通过临时文件。同时保持了原有函数接口不变。

如果仍然遇到问题，可能需要检查：
1. 图像base64数据是否正确
2. 视觉模型是否支持多图像输入
3. 服务器是否有足够内存处理图像

# 问题
```py
def _vlm_analyze_single_img(self, prompt, img_data_base64) -> str:
        '''使用VLM分析单张图片'''
        prompt_vision = prompt
        vision_result = ""
        
        try:
            print("\n[系统] 开始视觉深度分析处理...")
            
            # 假设image_base64_list是已经准备好的base64编码图像数据列表

            # 调用视觉模型进行深度分析，保持模型常驻
            stream = self.client.generate(
                model=MODEL_VISION,
                prompt=prompt_vision,
                # 注意: 一次只能推理一张图!!!
                images=[img_data_base64],
                stream=True,
                keep_alive=-1  # 保持模型常驻内存
            )

            # 收集和处理响应
            full_response = []
            print("[系统] 已提交视觉深度分析请求，等待响应...")
            
            for chunk in stream:
                print(chunk['response'], end='', flush=True)
                full_response.append(chunk['response'])
            
            # 获取完整的深度分析结果
            vision_result = ''.join(full_response)
            
            # 记录分析结果到日志
            with open('./result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
                log_file.write(vision_result)
                log_file.write('\n')

            # 返回结果
            return vision_result
                
        except Exception as e:
            print(f"[错误] 视觉分析失败: {str(e)}")
            raise
```
ollama的python库设置context为最大上下文长度;

## 回答
在 Ollama 的 Python 库中，可以通过在 API 请求中设置 `num_ctx` 参数来将上下文长度设置为最大值。以下是修改后的代码示例：

```python
def _vlm_analyze_single_img(self, prompt, img_data_base64) -> str:
    '''使用 VLM 分析单张图片'''
    prompt_vision = prompt
    vision_result = ""
    
    try:
        print("\n[系统] 开始视觉深度分析处理...")
        
        # 调用视觉模型进行深度分析，保持模型常驻
        stream = self.client.generate(
            model=MODEL_VISION,
            prompt=prompt_vision,
            # 注意: 一次只能推理一张图!!!
            images=[img_data_base64],
            stream=True,
            keep_alive=-1,  # 保持模型常驻内存
            options={"num_ctx": 32768}  # 设置上下文长度为最大值（根据模型和硬件限制调整）
        )

        # 收集和处理响应
        full_response = []
        print("[系统] 已提交视觉深度分析请求，等待响应...")
        
        for chunk in stream:
            print(chunk['response'], end='', flush=True)
            full_response.append(chunk['response'])
        
        # 获取完整的深度分析结果
        vision_result = ''.join(full_response)
        
        # 记录分析结果到日志
        with open('./result/depth_analysis.log', 'a', encoding='utf-8') as log_file:
            log_file.write(vision_result)
            log_file.write('\n')

        # 返回结果
        return vision_result
            
    except Exception as e:
        print(f"[错误] 视觉分析失败: {str(e)}")
        raise
```

### 说明
- 在 `self.client.generate` 方法中新增了 `options={"num_ctx": 32768}` 参数，用于设置上下文长度为最大值。这里的 `32768` 是一个示例值，具体最大值需要根据所使用的模型和硬件限制进行调整。
- 如果需要对所有模型统一设置上下文长度，也可以通过设置环境变量 `OLLAMA_CONTEXT_LENGTH` 来实现。

# 问题
```py
# 判断图像有效性
if image_base64:
    vision_result.append(self._vlm_analyze_single_img("分析图片的环境信息", image_base64))
if depth_imgs_base64:
    vision_result.append(self._vlm_analyze_single_img("分析图片的深度信息", depth_imgs_base64))

vision_result_plain = ''.join(vision_result)     
```
将两张base64编码的图片使用PIL上下拼接, 保存到磁盘`result/temp_images_{time.time()}`以及将新图base64编码再调用_vlm_analyze_single_img函数获取结果;

## 回答
Here's the code that combines two base64 encoded images vertically using PIL, saves the result to disk, and then analyzes the combined image:

```python
import os
import time
from PIL import Image
import io
import base64

# ... (your existing code)

if image_base64 and depth_imgs_base64:
    try:
        # Create result directory if it doesn't exist
        os.makedirs("result", exist_ok=True)
        
        # Decode both images
        img1 = Image.open(io.BytesIO(base64.b64decode(image_base64)))
        img2 = Image.open(io.BytesIO(base64.b64decode(depth_imgs_base64)))
        
        # Get the width of the wider image
        width = max(img1.width, img2.width)
        
        # Create a new image with combined height
        combined = Image.new('RGB', (width, img1.height + img2.height))
        
        # Paste the images (centered if widths differ)
        offset1 = (width - img1.width) // 2
        combined.paste(img1, (offset1, 0))
        
        offset2 = (width - img2.width) // 2
        combined.paste(img2, (offset2, img1.height))
        
        # Save to disk
        timestamp = str(time.time()).replace('.', '_')
        save_path = f"result/temp_images_{timestamp}.png"
        combined.save(save_path)
        
        # Convert combined image back to base64
        buffered = io.BytesIO()
        combined.save(buffered, format="PNG")
        combined_base64 = base64.b64encode(buffered.getvalue()).decode('utf-8')
        
        # Analyze the combined image
        combined_analysis = self._vlm_analyze_single_img("分析合并图片的环境和深度信息", combined_base64)
        vision_result.append(combined_analysis)
    except Exception as e:
        print(f"Error combining images: {e}")
        # Fall back to original separate analysis if combination fails
        if image_base64:
            vision_result.append(self._vlm_analyze_single_img("分析图片的环境信息", image_base64))
        if depth_imgs_base64:
            vision_result.append(self._vlm_analyze_single_img("分析图片的深度信息", depth_imgs_base64))
else:
    # Original separate analysis if only one image is available
    if image_base64:
        vision_result.append(self._vlm_analyze_single_img("分析图片的环境信息", image_base64))
    if depth_imgs_base64:
        vision_result.append(self._vlm_analyze_single_img("分析图片的深度信息", depth_imgs_base64))

vision_result_plain = ''.join(vision_result)
```

This code:
1. Checks if both images exist
2. Creates a directory called "result" if it doesn't exist
3. Decodes both base64 images into PIL Image objects
4. Creates a new image with the combined height of both images
5. Pastes the images centered vertically (handles different widths)
6. Saves the combined image with a timestamp-based filename
7. Converts the combined image back to base64
8. Analyzes the combined image
9. Includes error handling that falls back to separate analysis if anything goes wrong
10. Maintains the original behavior if only one image is available

The combined image will be saved in PNG format with a filename like `result/temp_images_1623456789_123456.png`.

## 回答
```python
import os
import time
from PIL import Image
import io
import base64

# ... (你的现有代码)

if image_base64 and depth_imgs_base64:
    try:
        # 创建结果目录（如果不存在）
        os.makedirs("result", exist_ok=True)
        
        # 解码两张图片
        img1 = Image.open(io.BytesIO(base64.b64decode(image_base64)))  # 环境图片
        img2 = Image.open(io.BytesIO(base64.b64decode(depth_imgs_base64)))  # 深度图片
        
        # 获取两张图片中较大的宽度
        width = max(img1.width, img2.width)
        
        # 创建新图片（高度为两张图片之和）
        combined = Image.new('RGB', (width, img1.height + img2.height))
        
        # 拼接图片（如果宽度不同则居中放置）
        offset1 = (width - img1.width) // 2  # 第一张图片的水平偏移量（居中）
        combined.paste(img1, (offset1, 0))  # 粘贴第一张图片到顶部
        
        offset2 = (width - img2.width) // 2  # 第二张图片的水平偏移量（居中）
        combined.paste(img2, (offset2, img1.height))  # 粘贴第二张图片到底部
        
        # 保存到磁盘
        timestamp = str(time.time()).replace('.', '_')  # 生成时间戳作为文件名
        save_path = f"result/temp_images_{timestamp}.png"  # 保存路径
        combined.save(save_path)  # 保存合并后的图片
        
        # 将合并后的图片转换为base64编码
        buffered = io.BytesIO()  # 创建内存缓冲区
        combined.save(buffered, format="PNG")  # 将图片写入缓冲区
        combined_base64 = base64.b64encode(buffered.getvalue()).decode('utf-8')  # 编码为base64字符串
        
        # 分析合并后的图片
        combined_analysis = self._vlm_analyze_single_img("分析合并图片的环境和深度信息", combined_base64)
        vision_result.append(combined_analysis)  # 添加分析结果
    except Exception as e:
        print(f"图片合并失败: {e}")
        # 如果合并失败，回退到单独分析模式
        if image_base64:
            vision_result.append(self._vlm_analyze_single_img("分析图片的环境信息", image_base64))
        if depth_imgs_base64:
            vision_result.append(self._vlm_analyze_single_img("分析图片的深度信息", depth_imgs_base64))
else:
    # 如果只有一张图片可用，执行原始单独分析逻辑
    if image_base64:
        vision_result.append(self._vlm_analyze_single_img("分析图片的环境信息", image_base64))
    if depth_imgs_base64:
        vision_result.append(self._vlm_analyze_single_img("分析图片的深度信息", depth_imgs_base64))

# 合并所有视觉分析结果
vision_result_plain = ''.join(vision_result)
```

主要修改内容：
1. 将所有注释改为中文说明
2. 对关键步骤添加了更详细的注释说明
3. 保持了原有的代码逻辑和结构
4. 错误提示信息也改为了中文
5. 添加了各步骤的中文解释

代码功能保持不变：
- 当两张图片都存在时，会垂直拼接并居中
- 拼接后的图片会保存到result目录
- 然后对拼接图进行分析
- 如果操作失败会回退到单独分析模式
- 如果只有一张图片就直接分析单张图片

# 问题
```py
def _execute_function_call(self, call):
        """执行函数调用"""
        func_name = call['function']['name']
        args = call['function']['arguments']
        
        try:
            # 转换参数格式
            if isinstance(args, str):
                args = json.loads(args.replace("'", '"'))
                
            print(f"[执行] {func_name}({args})")
            
            # 调用对应函数
            func = getattr(self.fn, func_name)
            if func_name == "attack":
                func()  # 无参数
            else:
                # 运动函数参数处理
                arg_name = 'cm' if func_name != 'rotate' else 'degrees'
                value = args.get(arg_name, 0)
                
                # 处理字符串值中的单位
                if isinstance(value, str):
                    if 'degrees' in value:
                        value = value.replace('degrees', '').strip()
                    elif 'cm' in value:
                        value = value.replace('cm', '').strip()
                
                # 转换为float
                value = float(value)
                
                func(value)
                
        except Exception as e:
            print(f"[错误] 函数执行失败: {str(e)}")
            raise
#####################################
class _Move():
    '''给大模型函数调用(Function Call)的精简函数'''
    def __init__(self, robot, controller):
        self.controller = controller
        self.robot = robot
        self.drone = self.robot
    def up(self, cm):
        '''上升'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_up(cm)
            print(f"函数up:{cm}厘米")
    def down(self, cm):
        '''下降'''
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_down(cm)
            print(f"函数down:{cm}厘米")
    def left(self, cm):
        '''左移'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_left(cm)
            print(f"函数left:{cm}厘米")
    def right(self, cm):
        '''右移'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_right(cm)
            print(f"函数right:{cm}厘米")
    def forward(self, cm):
        '''前进'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_forward(cm)
            print(f"函数forward:{cm}厘米")
    def backward(self, cm):
        '''后退'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.move_backward(cm)
            print(f"函数backward:{cm}厘米")
    def rotate(self, degrees):
        '''旋转'''
        # 仿真步进
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.rotate(degrees)
            print(f"函数rotate:{degrees}度")
    def attack(self):
        '''攻击'''
        # 打开激光
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.attack(True)
        # 等待一会
        time.sleep(5)
        # 关闭激光
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.attack(False)
        print("函数attack完成")
```
报错:
```sh
[错误] 函数执行失败: 'NoneType' object has no attribute 'step'
[错误] 决策处理失败: 'NoneType' object has no attribute 'step'
```
函数在call的时候没有传入self上下文;

## 回答
这个错误表明在调用 `_Move` 类的方法时，`self.drone` 是 `None`，导致无法访问 `step` 方法。这通常是因为 `_Move` 类的实例没有被正确初始化，或者 `robot` 参数传入了 `None`。

以下是解决方案：

1. **确保正确初始化 `_Move` 类**：
   在创建 `_Move` 实例时，确保传入了有效的 `robot` 和 `controller` 对象。

2. **检查 `_execute_function_call` 方法**：
   确保 `self.fn` 是正确初始化的 `_Move` 实例。

3. **修改 `_execute_function_call` 方法**：
   添加对 `self.fn` 的检查，确保它不是 `None`。

修改后的代码：

```python
def _execute_function_call(self, call):
    """执行函数调用"""
    if not hasattr(self, 'fn') or self.fn is None:
        raise ValueError("Move functions are not initialized (self.fn is None)")
        
    func_name = call['function']['name']
    args = call['function']['arguments']
    
    try:
        # 转换参数格式
        if isinstance(args, str):
            args = json.loads(args.replace("'", '"'))
            
        print(f"[执行] {func_name}({args})")
        
        # 调用对应函数
        func = getattr(self.fn, func_name)
        if func_name == "attack":
            func()  # 无参数
        else:
            # 运动函数参数处理
            arg_name = 'cm' if func_name != 'rotate' else 'degrees'
            value = args.get(arg_name, 0)
            
            # 处理字符串值中的单位
            if isinstance(value, str):
                if 'degrees' in value:
                    value = value.replace('degrees', '').strip()
                elif 'cm' in value:
                    value = value.replace('cm', '').strip()
            
            # 转换为float
            value = float(value)
            
            func(value)
            
    except Exception as e:
        print(f"[错误] 函数执行失败: {str(e)}")
        raise
```

4. **初始化检查**：
   在创建 `_Move` 实例的地方，确保 `robot` 和 `controller` 不是 `None`：

```python
# 示例初始化代码
robot = ...  # 确保这是有效的机器人对象
controller = ...  # 确保这是有效的控制器对象
move = _Move(robot, controller)
```

5. **调试建议**：
   - 在 `_Move.__init__` 中添加打印语句，检查传入的参数：
     ```python
     def __init__(self, robot, controller):
         print(f"Initializing _Move with robot: {robot}, controller: {controller}")
         self.controller = controller
         self.robot = robot
         self.drone = self.robot
     ```
   - 在执行函数调用前检查 `self.fn` 和 `self.fn.drone`：
     ```python
     print(f"self.fn: {self.fn}")
     if hasattr(self.fn, 'drone'):
         print(f"self.fn.drone: {self.fn.drone}")
     ```

通过以上修改和检查，你应该能够解决 `'NoneType' object has no attribute 'step'` 的错误。问题的根本原因是 `_Move` 类的实例没有被正确初始化，或者初始化时传入了 `None` 值。

## 回答
这段代码看起来是在处理一个提示词（prompt）中是否包含特定标记 `_DEPTH_INFO`，并根据情况决定是否用 `vision_result_plain` 替换该标记。以下是代码的分析和改进建议：

---

### **代码分析**
1. **`prompt.contain("_DEPTH_INFO")`**  
   - 假设 `prompt` 是一个字符串或某种支持 `contain()` 方法的对象（但 Python 标准字符串方法是 `str.contains()` 或 `in` 运算符）。
   - 如果 `prompt` 包含 `"_DEPTH_INFO"`，则用 `vision_result_plain` 替换它（类似 `str.format()` 操作）。
   - 否则，打印 `"可能不需要深度信息"`。

2. **潜在问题**
   - **`contain()` 方法不存在**：Python 字符串的标准方法是 `in` 或 `str.__contains__()`，而不是 `contain()`。
   - **`format()` 的安全性**：如果 `vision_result_plain` 包含 `{}`，可能会导致 `format()` 报错（除非 `_DEPTH_INFO` 是唯一需要替换的变量）。
   - **逻辑是否合理**：如果 `_DEPTH_INFO` 不存在，是否真的不需要深度信息？还是应该报错？

---

### **改进版本**
#### **方案 1：使用 `in` 检查字符串包含**
```python
if "_DEPTH_INFO" in prompt:
    this_prompt = prompt.format(_DEPTH_INFO=vision_result_plain)
else:
    print("可能不需要深度信息")
```

#### **方案 2：更安全的 `format()`（避免 `KeyError`）**
```python
try:
    this_prompt = prompt.format(_DEPTH_INFO=vision_result_plain)
except KeyError:
    print("提示词不包含 _DEPTH_INFO，可能不需要深度信息")
```

#### **方案 3：使用 `str.replace()`（如果不需要复杂格式化）**
```python
if "_DEPTH_INFO" in prompt:
    this_prompt = prompt.replace("_DEPTH_INFO", str(vision_result_plain))
else:
    print("可能不需要深度信息")
```

---

### **建议**
1. **检查 `prompt` 的类型**  
   - 如果 `prompt` 是自定义类，确保它实现了 `contain()` 方法或改用 `in`。
   - 如果是普通字符串，直接用 `in` 检查。

2. **考虑是否需要深度信息**  
   - 如果 `_DEPTH_INFO` 是必填项，建议抛出异常而不是仅打印日志：
     ```python
     if "_DEPTH_INFO" not in prompt:
         raise ValueError("提示词必须包含 _DEPTH_INFO，但未找到")
     ```

3. **处理 `vision_result_plain` 的格式**  
   - 如果 `vision_result_plain` 可能包含特殊字符（如 `{}`），建议先转义或使用 `str.replace()` 代替 `format()`。

---

### **最终推荐代码**
```python
if "_DEPTH_INFO" in prompt:
    this_prompt = prompt.replace("_DEPTH_INFO", str(vision_result_plain))  # 或使用 .format()
else:
    print("警告：提示词不包含 _DEPTH_INFO，可能不需要深度信息")
    this_prompt = prompt  # 保留原提示词
```

这样能确保：
1. 正确检查 `_DEPTH_INFO` 是否存在。
2. 安全替换内容（避免 `format()` 的潜在问题）。
3. 提供清晰的日志或错误提示。

# 问题


## 回答


















