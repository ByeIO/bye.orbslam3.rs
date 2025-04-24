# 无人机实例

#############################################
# 内置库
import time
# 第三方库
import numpy as np
import pytest
# webots专用库
from controller import Robot, Supervisor
#############################################

class _Drone(Robot):
    def __init__(self):
        """
        初始化无人机实例
        :param: Webots机器人实例
        """
        # 构造机器人实例
        self.robot = Supervisor()
        self.drone_node = self.robot.getSelf()  # 获取无人机节点用于直接控制位置

        # 初始化时间戳
        self.time_step = int(self.robot.getBasicTimeStep())
        
        # 初始化无人机硬件设备
        # 相机
        self.camera = self.robot.getDevice("camera")
        self.camera.enable(self.time_step)
        
        # 传感器
        self.imu = self.robot.getDevice("inertial unit")
        self.imu.enable(self.time_step)
        self.gps = self.robot.getDevice("gps")
        self.gps.enable(self.time_step)
        self.gyro = self.robot.getDevice("gyro")
        self.gyro.enable(self.time_step)
        
        # 初始化电机
        self.front_left_motor = self.robot.getDevice("front left propeller")
        self.front_right_motor = self.robot.getDevice("front right propeller")
        self.rear_left_motor = self.robot.getDevice("rear left propeller")
        self.rear_right_motor = self.robot.getDevice("rear right propeller")

        # 设置电机初始状态
        motors = [self.front_left_motor, self.front_right_motor,
                 self.rear_left_motor, self.rear_right_motor]
        for motor in motors:
            motor.setPosition(float('inf'))
            motor.setVelocity(50)  # 设置固定转速模拟电机运转