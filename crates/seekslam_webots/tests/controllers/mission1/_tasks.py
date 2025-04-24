# 定义各个原子任务

#############################################
# 第三方库
## 线性代数
import numpy as np
# 图像处理
import cv2
# 异步
import asyncio
# webots专用库
from controller import Robot
# 内置库
import os
import time
from collections import deque
from enum import Enum
# 项目库
## 控制无人机移动
from _controller import _DroneController
## 任务状态枚举
from _state import _MissionState
#############################################

class _MissionTasks():
    def __init__(self, drone, controller):
        print("hello from _MissionTasks")
        # 机器人实例
        self.drone = drone
        # 控制器实例
        self.controller = controller
        
    def camera_task(self) -> bool:
        '''1Hz采集图像, 保存到文件夹'''
        # 调用_camera模块的对应函数
        pass
        # TODO
        
    def mapping_task(self) -> bool:
        '''建图任务'''
        # 自旋一周机身每旋转45度采集一张图像
        self.controller
        # 采集图像完成后, 编码8张图像为list[base64]数据列表, 调用_mapping模块建图
        process_images_to_3dmodel()
        # TODO
    def ocr_task(self) -> bool:
        '''字母识别任务'''
        # 调用_ocr的对应函数
        pass
        # TODO
    def decision_task(self) -> bool:
        '''核心任务, 智能决策并形成控制指令'''
        # 调用_decision模块的对应函数
        pass
        # TODO
    def attack_task(self) -> bool:
        '''一次性的模拟攻击函数, 打印并切换状态'''
        '''注意: 这个函数被决策任务自动调用'''
        print("攻击目标`B`成功!!!!")
        print("返航!!")
        self.mission_state = _MissionState.HOME