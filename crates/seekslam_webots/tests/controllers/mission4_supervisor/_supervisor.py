# 超级控制器(女娲模式)

# 无人机实例

#############################################
# 内置库
import time
# 第三方库
import numpy as np
import pytest
# webots专用库
from controller import Supervisor 
#############################################

# 直接继承Supervisor
class _DroneSuper(Supervisor):  
    def __init__(self):
        """
        初始化无人机实例
        """
        # 调用父类初始化
        super().__init__()
        
        # 获取无人机节点用于直接控制位置
        self.drone_node = self.getSelf()  
        self.drone_translation = self.drone_node.getField("translation")
        self.drone_rotation = self.drone_node.getField("rotation")

        # 初始化时间戳
        self.time_step = int(self.getBasicTimeStep())
        
        # 初始化无人机硬件设备
        # 相机
        self.camera = self.getDevice("camera")
        self.camera.enable(self.time_step)
        
        # 传感器
        self.imu = self.getDevice("inertial unit")
        self.imu.enable(self.time_step)
        self.gps = self.getDevice("gps")
        self.gps.enable(self.time_step)
        self.gyro = self.getDevice("gyro")
        self.gyro.enable(self.time_step)
        
        # 初始化电机
        # self.front_left_motor = self.getDevice("front left propeller")
        # self.front_right_motor = self.getDevice("front right propeller")
        # self.rear_left_motor = self.getDevice("rear left propeller")
        # self.rear_right_motor = self.getDevice("rear right propeller")

        # # 设置电机初始状态
        # motors = [self.front_left_motor, self.front_right_motor,
        #          self.rear_left_motor, self.rear_right_motor]
        # for motor in motors:
        #     motor.setPosition(float('inf'))
        #     # 设置固定转速模拟电机运转
        #     motor.setVelocity(10)  