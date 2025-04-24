# 智慧无人机决策控制系统
# 特点: 接入ollama大模型能力, 实现智能决策
# 功能: 通过视觉大模型识别场景，使用函数调用模型进行多阶段决策和控制

# 攻击目标(与tasks中一致)
_TARGET = "B"
# 系统提示词
_SYSTEM_PROMPT = f"你是一架智慧无人机, 你的任务是找到{_TARGET}并进行攻击, 然后返回到起飞点.你可以通过深度图、OCR识别图、三维重建图进行决策.你可以使用up, down, left, right, forward, backward, rotate, attack函数; 优先使用forward以前进; 函数参数只需要提供数字, 不要带单位.你需要不断地进行思考以应对各种情况."
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
# OLLAMA服务地址
OLLAMA_SERVER = "http://10.8.8.20:11435"
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
import tempfile
#####################################

# 输入: 原图, 深度图, ocr结果
# 输出: 控制指令 up, down, forward, backward, left, right, rotate, attack

class _Ollama():
    def __init__(self, robot, controller):
        # 初始化ollama客户端
        self.client = Client(host=OLLAMA_SERVER)
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
                f'{OLLAMA_SERVER}/api/generate',
                json={'model': MODEL_VISION, 'prompt': ' ', 'keep_alive': -1}
            )
            # 预加载函数调用模型
            requests.post(
                f'{OLLAMA_SERVER}/api/generate',
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
                prompt = _PROMPT2.replace("_OCR_RESULT", str(ocr_result))
            elif stage == 3:
                prompt = _PROMPT3.format(_OCR_RESULT=ocr_result)
            elif stage == 4:
                prompt = _PROMPT4
            else: 
                prompt = f"你是自主侦察无人机, 你的任务是在室内搜索{_TARGET}并攻击."

            # 1. 调用视觉模型分析图像
            vision_result = []
            
            # 拼接并分析图片
            vision_result_plain = self._analyze_ori_and_depth_imgs(image_base64, depth_imgs_base64)

            # 将视觉大模型返回的结果添加到prompt中(判断结果非空)
            this_prompt = ''
            if "_DEPTH_INFO" in prompt and vision_result_plain:
                this_prompt = prompt.replace("_DEPTH_INFO", str(vision_result_plain)) 
            else:
                print("可能不需要深度信息")
                this_prompt = prompt

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
    #########################################
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
    #########################################
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
    #########################################
    def _update_history(self, question, answer):
        """更新对话历史"""
        self.chat_history.append({"q": question, "a": answer})
        # 打印最后一条记录
        print("最新对话记录:", self.chat_history[-1])
    #########################################    
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
    #############################################
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
                # 保持模型常驻内存
                keep_alive=-1,  
                # 设置上下文长度为最大值（根据模型和硬件限制调整）
                options={"num_ctx": 32768},  
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
    #############################################
    def _analyze_ori_and_depth_imgs(self, image_base64, depth_imgs_base64) -> str:
        '''拼接图片并分析原图和深度图'''
        vision_result = []
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
        return vision_result_plain
        #############################################

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
    #########################################
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

    # 智能体
    _agent = _Ollama(None, MockController())

    #########################################
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

    #########################################
    # 辅助函数：将图片转换为base64
    def image_to_base64(image_path):
        with open(image_path, "rb") as image_file:
            return base64.b64encode(image_file.read()).decode('ascii')
            
    #########################################
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
                base64_str = base64.b64encode(img_bytes).decode('ascii')
                return base64_str
                
        except Exception as e:
            print(f"[错误] 图像转换失败({image_path}): {str(e)}", file=sys.stderr)
            return None
    ##################################################
    
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
        image_base64=any_image_to_base64(stage2_original),
        depth_imgs_base64=any_image_to_base64(stage2_depth),
        ocr_result=stage2_ocr
    )

    # 模拟阶段3处理
    print("\n=== 阶段3测试 ===")
    _agent.process_stage(
        stage=3,
        image_base64=any_image_to_base64(stage3_original),
        depth_imgs_base64=any_image_to_base64(stage3_depth),
        ocr_result=stage3_ocr
    )

    # 模拟阶段4处理
    print("\n=== 阶段4测试 ===")
    _agent.process_stage(
        stage=4,
        image_base64=any_image_to_base64(stage4_original),
        depth_imgs_base64=any_image_to_base64(stage4_depth),
        ocr_result=stage4_ocr
    )