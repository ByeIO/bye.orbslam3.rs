"""
seekslam_mission1控制顶层任务
无人机: Dji Mavic2Pro
"""

#############################################
# 第三方库
## 线性代数
import numpy as np
# 图像处理
import cv2
# 异步
import asyncio
# 内置库
import os
import time
from collections import deque
from enum import Enum
# 项目库
## 无人机实例
from _supervisor import _DroneSuper
## 控制无人机移动
from _controller import _DroneController
## 任务状态枚举
from _state import _MissionState
## 原子任务
from _tasks import _MissionTasks
## 智能决策
from _ollama import _Ollama
#############################################

# 使用装饰器模式实现只执行一次的功能
def run_once_per_arg(func):
    """
    装饰器：确保被装饰的每个不同参数组合只执行一次
    """
    cache = set()
    def wrapper(*args, **kwargs):
        # 创建参数的唯一标识
        key = (args, frozenset(kwargs.items()))
        if key not in cache:
            cache.add(key)
            return func(*args, **kwargs)
    return wrapper

@run_once_per_arg
def print_once(text):
    print(text)

class _FlightMission:
    """飞行任务管理器，处理高级飞行逻辑"""
    def __init__(self):
        print("hello from _FlightMission")
        # 上电初始化任务状态
        self.mission_state = _MissionState.INIT
        # 机器人实例
        self.drone = _DroneSuper()
        # 控制器
        self.controller = _DroneController(self.drone)
        # 智能决策
        self.agent = _Ollama(self.drone, self.controller)
        # 原子任务
        self.tasks = _MissionTasks(self.drone, self.controller, self.agent)
        
    def run(self):
        '''开始自动执行任务'''
        print("mission started")
        while self.drone.step(self.drone.time_step) != -1:
            # print("调试信息:", self.drone.step(self.drone.time_step))
            ###############################################   
            if self.mission_state == _MissionState.INIT:
                # 进行初始化动作
                print_once("开始初始化算法模型, 加载模型到内存中...")
                pass
                # 初始化完成
                self.mission_state = _MissionState.READY
                # 设置无人机初始位置
                self.controller.drone_rotation.setSFRotation([0, 0, 1, 0])
                print_once("初始化完成")
            ###############################################    
            elif self.mission_state == _MissionState.READY:
                # 准备起飞的状态
                print_once("准备起飞")
                self.mission_state = _MissionState.TAKEOFF
                # 起飞到150cm高度成功
                if self.controller.take_off(150) == True:
                    self.tasks.init_rot()
                    # 切换到下个状态 
                    # self.mission_state = _MissionState.ROTATE
                    # 临时调试
                    self.mission_state = _MissionState.DEBUG
            ###############################################   
            elif self.mission_state == _MissionState.ROTATE:
                # 2. 自旋一周并建图, 图片和建图结果发送给决策模块
                print_once("准备自旋")
                if self.tasks.mapping_task() == True:  
                    print("建图成功, 即将进行智能决策")
                    self.mission_state = _MissionState.HOVER
                else:
                    print("建图失败, 但还是继续执行任务")
                    self.mission_state = _MissionState.HOVER
            ###############################################       
            elif self.mission_state == _MissionState.HOVER:
                print_once("等待决策模块指令")
                # 3. 保持悬停在50cm, 等待决策模块的指令然后才切换状态
                self.controller.hover()
                pass
            elif self.mission_state == _MissionState.HOME:
                print_once("正在查询轨迹记录器, 准备返航")
                if self.tasks._debug_return_home() == True:
                    # 标记结束任务
                    self.mission_state = _MissionState.LAND
            ###############################################   
            elif self.mission_state == _MissionState.DEBUG:
                print_once("调试状态")
                # 移动到打击位置
                if self.tasks._debug_full() == True:
                    # 结束任务
                    self.mission_state = _MissionState.HOME
                    print("结束任务")
            ###############################################   
            # 执行定时任务
            self.tasks.camera_task()
            ############################################### 
            
if __name__ == "__main__":
    print("byeefree seekslam_webots\n")
    print("hello from seekslam_mission1")
    # 初始化顶层任务, 开始自动执行
    mission = _FlightMission()
    # 主循环, 循环步进仿真
    mission.run()
    