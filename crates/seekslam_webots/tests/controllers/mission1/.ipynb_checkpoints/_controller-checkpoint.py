# 顶层控制模块封装
# 运动学模型: Dji Mavic2Pro

#############################################
# 内置库
import time
import csv
import os
# 第三方库
import numpy as np
import pytest
import asyncio
# 项目库
from _pid import _PIDController
from _trajectory import _track
#############################################

class _DroneController:
    """无人机控制封装类，处理底层电机控制和传感器数据，提供顶层接口"""
    
    def __init__(self, drone):
        """
        初始化无人机控制器
        :param drone: Webots无人机实例，需包含传感器和电机设备
        """
        self.drone = drone

        # 初始化时间步长（单位：ms）
        self.time_step = self.drone.time_step

        # 获取传感器设备
        self.imu = self.drone.imu
        self.gyro = self.drone.gyro
        self.gps = self.drone.gps

        # 获取电机设备
        self.front_left_motor = self.drone.front_left_motor
        self.front_right_motor = self.drone.front_right_motor
        self.rear_left_motor = self.drone.rear_left_motor
        self.rear_right_motor = self.drone.rear_right_motor

        # 无人机参数（根据Mavic2Pro实测调整）
        self.K_VERTICAL_THRUST = 68.5      # 基础升力推力
        self.K_VERTICAL_OFFSET = 0.6       # 垂直高度补偿偏移
        self.K_VERTICAL_P = 3.0            # 垂直高度比例系数
        self.K_ROLL_P = 50.0               # 横滚比例系数
        self.K_PITCH_P = 30.0              # 俯仰比例系数
        self.MAX_YAW_DISTURBANCE = 0.4     # 最大偏航干扰量
        self.MAX_PITCH_DISTURBANCE = -1    # 最大俯仰干扰量
        self.target_precision = 0.1        # 目标位置精度（米）

        # 状态变量
        self.current_pose = [0, 0, 0, 0, 0, 0]  # [X,Y,Z,roll,pitch,yaw]
        self.target_pose = [0, 0, 1.5, 0, 0, 0]  # 初始目标高度1.5米
        self.waypoints = []                    # 巡逻航点列表
        self.target_index = 0                  # 当前目标航点索引
        self.yaw_disturbance = 0               # 当前偏航干扰量
        self.pitch_disturbance = 0             # 当前俯仰干扰量

        # 初始化偏航PID控制器
        self.yaw_pid = _PIDController(kp=1.0, ki=0.1, kd=0.1)

    def _update_sensors(self):
        """更新传感器数据并返回当前姿态"""
        # 获取IMU数据（横滚、俯仰、偏航，单位：弧度）
        roll, pitch, yaw = self.imu.getRollPitchYaw()
        # 获取GPS数据（X,Y,Z，单位：米）
        x, y, altitude = self.gps.getValues()
        # 获取陀螺仪数据（角速度）
        roll_velocity, pitch_velocity, yaw_velocity = self.gyro.getValues()
        # 更新当前状态
        self.current_pose = [x, y, altitude, roll, pitch, yaw]
        return self.current_pose

    def _clamp(self, value, min_val, max_val):
        """限制数值在[min_val, max_val]范围内"""
        return max(min(value, max_val), min_val)

    def _calculate_motor_inputs(self):
        """计算各电机输入量"""
        # 获取当前状态
        current_altitude = self.current_pose[2]
        roll, pitch, yaw = self.current_pose[3:6]
        roll_velocity, pitch_velocity, yaw_velocity = self.gyro.getValues()

        # 垂直控制（基于立方比例项）
        alt_error = (self.target_pose[2] - current_altitude) + self.K_VERTICAL_OFFSET
        clamped_alt_error = self._clamp(alt_error, -1, 1)
        vertical_input = self.K_VERTICAL_P * pow(clamped_alt_error, 3.0)

        # 横滚控制（比例项 + 角速度补偿）
        roll_input = self.K_ROLL_P * self._clamp(roll, -1, 1) + roll_velocity
        
        # 俯仰控制（比例项 + 角速度补偿）
        pitch_input = self.K_PITCH_P * self._clamp(pitch, -1, 1) + pitch_velocity
        
        # 偏航控制（干扰量）
        yaw_input = self.yaw_disturbance

        # 计算各电机推力（注意旋转方向）
        thrust = self.K_VERTICAL_THRUST + vertical_input
        front_left = thrust - yaw_input + pitch_input - roll_input
        front_right = thrust + yaw_input + pitch_input + roll_input
        rear_left = thrust + yaw_input - pitch_input - roll_input
        rear_right = thrust - yaw_input - pitch_input + roll_input

        return front_left, front_right, rear_left, rear_right

    def _stabilize(self):
        """执行稳定控制，更新电机速度"""
        # 更新传感器数据
        self._update_sensors()

        # 计算电机输入
        fl, fr, rl, rr = self._calculate_motor_inputs()

        # 设置电机速度（注意转向方向，与示例代码保持一致）
        self.front_left_motor.setVelocity(fl)
        self.front_right_motor.setVelocity(-fr)
        self.rear_left_motor.setVelocity(-rl)
        self.rear_right_motor.setVelocity(rr)

    def _apply_landing_thrust(self):
        """在降落时逐渐减小推力(软着陆)"""
        min_thrust = 30  # 防止突然断电
        for motor in [
            self.front_left_motor,
            self.front_right_motor,
            self.rear_left_motor,
            self.rear_right_motor
        ]:
            current_velocity = motor.getVelocity()
            if current_velocity > min_thrust:
                motor.setVelocity(current_velocity * 0.95)

    def _log_takeoff_data(self, **kwargs):
        """记录起飞数据到CSV文件"""
        file_exists = os.path.isfile('take_off.csv')
        
        with open('take_off.csv', 'a', newline='') as f:
            writer = csv.writer(f)
            
            # 如果文件不存在，写入表头
            if not file_exists:
                headers = list(kwargs.keys())
                writer.writerow(headers)
            
            # 写入数据
            writer.writerow([kwargs[key] for key in kwargs.keys()])

    def hover(self) -> bool:
        """保持当前高度悬停"""
        # 保持当前高度
        self.target_pose[2] = self.current_pose[2]
        # 执行稳定控制
        self._stabilize()
        return True

    def take_off(self, target_altitude=150) -> bool:
        """起飞至指定高度cm，加入姿态稳定控制"""
        # 转换为米单位
        target_altitude_m = target_altitude / 100.0
        self.target_pose[2] = target_altitude_m
        
        # 执行稳定控制（会自动调整到目标高度）
        self._stabilize()
        
        # 检查是否达到目标高度
        current_altitude = self.current_pose[2]
        if abs(current_altitude - target_altitude_m) <= self.target_precision:
            _track({"name":"up", "value":target_altitude})
            return True
        
        return False

    def land(self) -> bool:
        """降落至地面"""
        self.target_pose[2] = 0.0
        self._update_sensors()
        current_altitude = self.current_pose[2]
        
        # 执行稳定控制（会自动降低高度）
        self._stabilize()
        
        # 确保推力不低于最小值以实现软着陆
        self._apply_landing_thrust()
        
        # 判断降落成功
        if current_altitude <= 0.1:  # 稍微放宽判断条件
            return True
        return False

    def rotate(self, degrees) -> bool:
        """自旋指定角度（度）"""
        target_yaw = (self.current_pose[5] + np.deg2rad(degrees) + np.pi) % (2 * np.pi) - np.pi
        self.target_pose[5] = target_yaw
        self._update_sensors()
        current_yaw = self.current_pose[5]
        yaw_error = (target_yaw - current_yaw + np.pi) % (2 * np.pi) - np.pi  # 最小角度差
        
        # 更新偏航干扰量（PID控制）
        self.yaw_disturbance = self.yaw_pid.update(yaw_error, self.time_step / 1000.0)
        self.yaw_disturbance = self._clamp(
            self.yaw_disturbance,
            -self.MAX_YAW_DISTURBANCE,
            self.MAX_YAW_DISTURBANCE
        )
        
        # 实际控制四个电机
        self._stabilize()
        
        # 判断旋转到目标位置
        if abs(yaw_error) < np.deg2rad(1):  # 1度误差范围内认为完成
            _track({"name":"rotate", "value":degrees})
            return True
        return False