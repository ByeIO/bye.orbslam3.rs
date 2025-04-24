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