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
import logging
import math
# 项目库
## 控制无人机移动
from _controller import _DroneController
## 任务状态枚举
from _state import _MissionState
## 建图
from _mapping import process_images_to_3dmodel, save_glb_base64
## 相机处理
from _camera import _Camera
## 路径
from _trajectory import _KEY_PATH_POINTS, _KEY_PATH_ROTATIONS, _track
#############################################

# 配置日志
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class _MissionTasks():
    def __init__(self, drone_super, controller):
        print("hello from _MissionTasks")
        # 机器人实例
        self.drone = drone_super
        # 控制器实例
        self.controller = controller
        # 相机处理
        self.camera = _Camera(self.drone)
        # 任务定时器
        self.timer = time.time()
        # 全局地图
        self.map = []
    #############################################    
    def camera_task(self) -> bool:
        '''1Hz采集图像, 保存到文件夹(实现侦察功能)'''
        # 检查是否达到1Hz频率
        current_time = time.time()
        if current_time - self.timer >= 1.0:
            # 调用_camera模块的对应函数
            self.camera.capture_image_and_save()
            # 重置计时器
            self.timer = current_time
            return True
        return False
    #############################################
    def mapping_task(self) -> bool:
        '''建图任务'''
        try:
            # 图片base64数组
            img_data_base64 = []
            
            # 自旋一周机身每旋转45度采集一张图像
            for i in range(18):
                # 仿真步进
                if(self.drone.step(self.drone.time_step) != -1):
                    # 旋转
                    print("旋转#", i)
                    self.controller.rotate(22.5)
                    # 采集图像并添加到数组
                    img_data = self.camera.capture_image_and_save_and_return()
                    if img_data:
                        img_data_base64.append(img_data)

                    # 不要太快了
                    time.sleep(1)
            
            # 检查是否成功采集到图像
            if not img_data_base64:
                logger.error("未采集到任何图像数据")
                return False
            print("图像采集完成, 正在提交建图请求, 等待结果需要约3分钟")
            time.sleep(1)
            
            # 调用建图模块处理图像(第一张图有问题, 忽略)
            map_base64 = process_images_to_3dmodel(img_data_base64[1:])
            # 添加地图
            self.map.append(map_base64)
            
            # 保存生成的3D模型
            if map_base64:
                # 设置保存路径，可以根据需要修改
                save_path = "output/map.glb"
                if save_glb_base64(map_base64, save_path):
                    print("建图完成!!!")
                    logger.info(f"3D模型成功保存到: {save_path}")
                    return True
                else:
                    logger.error("保存3D模型失败")
                    return False
            else:
                logger.error("建图失败，未生成有效的3D模型")
                return False
                
        except Exception as e:
            logger.error(f"建图任务执行过程中出错: {str(e)}")
            return False
    #############################################    
    def ocr_task(self) -> bool:
        '''字母识别任务'''
        # 调用_ocr的对应函数
        pass
        # TODO
    #############################################
    def decision_task(self) -> bool:
        '''核心任务, 智能决策并形成控制指令'''
        # 调用_decision模块的对应函数
        pass
        # TODO
    #############################################
    def attack_task(self, target) -> bool:
        '''一次性的模拟攻击函数, 打印并切换状态'''
        '''注意: 这个函数被决策任务自动调用'''
        target_node = self.drone.getFromDef(target)
        
        # 设置target_node的textureUrl为none
        if target_node:
            # Change texture
            texture_url_field = target_node.getField("textureUrl")
            if texture_url_field:
                texture_url_field.setSFString("../../controllers/angel.png")
            else:
                print(f"警告: 目标节点 {target} 没有textUrl字段")
            
            # Set rotation to 180 degrees around z-axis
            rotation_field = target_node.getField("rotation")
            if rotation_field:
                rotation_field.setSFRotation([0, 0, 1, 1.55])  # [x, y, z, angle] where angle is in radians
            else:
                print(f"警告: 目标节点 {target} 没有rotation字段")
        else:
            print(f"错误: 找不到目标节点 {target}")
            return False
    
        # 打印信息
        print("攻击目标`", target, "`成功!!!!")
        print("返航!!")
        self.mission_state = _MissionState.HOME
        return True
    #############################################
    def _debug_full(self) -> bool:
        '''全任务调试'''
        print("全任务调试, 将会移动到作战位置(-5, -3, 1.5)")
        # 地图全局坐标在第三象限!!!
        # 机器人全局坐标出生点(-1.41, -2.03, 0)
        # PAD自身坐标的x轴正向为前进正向, y轴为左正向
        # 仿真步进
        if(self.drone.step(self.drone.time_step) != -1):
            # 前进2.5m, 暂停1.3s防止移动太快了
            self.controller.move_forward(250)
            # 目标位置: (-2.45, -4.31, 1.5), 两个房间交界处
            # 检查关键点误差并修正
            if self.key_path_check(1):
                print("已修正关键路径点误差")
            time.sleep(1)
        if(self.drone.step(self.drone.time_step) != -1):
            # 右转90度
            self.controller.rotate(-90)
            time.sleep(1)
        if(self.drone.step(self.drone.time_step) != -1):
            # 前进2米
            # self.controller.move_forward(200)
            # 到达侦察位置
            if self.key_path_check(2):
                print("已修正关键路径点误差")
            time.sleep(1)
        if(self.drone.step(self.drone.time_step) != -1):
            # 右转45度
            self.controller.rotate(-45)
            time.sleep(1)
        if(self.drone.step(self.drone.time_step) != -1):
            # 前进1米
            # self.controller.move_forward(100)
            time.sleep(1)
        for i in range(15):
            if(self.drone.step(self.drone.time_step) != -1):
                # 自旋一周, 建图
                self.controller.rotate(-30)
                time.sleep(1)
        if(self.drone.step(self.drone.time_step) != -1):
            # 悬停等待决策
            if self.key_path_check(3):
                print("已修正关键路径点误差")
            time.sleep(1)
        if(self.drone.step(self.drone.time_step) != -1):
            # 去往攻击点
            if self.key_path_check(4):
                print("已修正关键路径点误差")
            time.sleep(1)
        if(self.drone.step(self.drone.time_step) != -1):
            # 模拟攻击
            if self.attack_task("B"):
                print("返回")
            time.sleep(1)
        return True
    #############################################
    def init_rot(self) -> bool:
        '''自旋以校正坐标'''
        # 自旋一圈以校准坐标
        if(self.drone.step(self.drone.time_step) != -1):
            # self.controller.rotate(15)
            time.sleep(1.2)
        # if(self.drone.step(self.drone.time_step) != -1):
        #     self.controller.rotate(350)
        #     time.sleep(1.3)
        return True
    #############################################
    def key_path_check(self, num) -> bool:
        '''
        查询关键途径点[num]与当前位置并计算l2距离和比较位姿,
        如果距离小于0.1则不用修正, 否则直接传送`PAD`过去并设置`PAD`位姿
        
        参数:
            num: 关键路径点的索引(0-4)
            
        返回:
            bool: 是否进行了修正(True表示进行了传送修正)
        '''
        # 检查索引是否有效
        if num < 0 or num >= len(_KEY_PATH_POINTS) or num >= len(_KEY_PATH_ROTATIONS):
            raise ValueError(f"无效的关键路径点索引: {num}")
        
        # 获取当前位置和旋转
        current_pos = self.controller.drone_translation.getSFVec3f()
        current_rot = self.controller.drone_rotation.getSFRotation()
        
        # 获取目标位置和旋转
        target_pos = _KEY_PATH_POINTS[num]
        target_rot = _KEY_PATH_ROTATIONS[num]
        
        # 计算L2距离
        distance = math.sqrt(
            (current_pos[0] - target_pos[0])**2 +
            (current_pos[1] - target_pos[1])**2 +
            (current_pos[2] - target_pos[2])**2
        )
        
        # 如果距离小于0.1，不需要修正
        if distance < 0.1:
            return False
        
        # 否则直接传送PAD到目标位置和姿态
        if self.drone.step(self.drone.time_step) != -1:
            self.controller.pad_translation.setSFVec3f(target_pos)
            self.controller.pad_rotation.setSFRotation(target_rot)
        # 记录轨迹
        _track({
            "name": "key_path_correction",
            "value": {
                "point_num": num,
                "from_pos": current_pos,
                "to_pos": target_pos,
                "from_rot": current_rot,
                "to_rot": target_rot
            }
        })
        
        return True
    ##############################################   
    def _debug_return_home(self) -> bool:
        '''沿着关键路径点返回起飞点并降落'''
        print("返航准备完成, 正在返回起飞点...")
        for i in reversed(range(len(_KEY_PATH_POINTS))):
            if(self.drone.step(self.drone.time_step) != -1):
                print(f"回到关键点# {i}")
                # 返回到关键路径点
                self.key_path_check(i)
            if(self.drone.step(self.drone.time_step) != -1):
                # 朝向与来时相反
                self.controller.rotate(180)
            # 不要太快了
            time.sleep(1.5)
        return True
    ##############################################                  
    