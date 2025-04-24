"""
重构后的无人机控制程序
主要改进：
1. 使用DroneController类封装底层控制逻辑
2. 解耦飞行控制、图像采集和状态机模块
3. 添加详细中文注释
4. 保持原有功能：起飞、悬停、旋转并采集图像
"""

#############################################
# 第三方库
from controller import Robot
import numpy as np
import cv2
# 内置库
import os
import time
from collections import deque
# 项目库
from .pid import PIDController
#############################################

class DroneController:
    """无人机控制封装类，处理底层电机控制和传感器数据"""
    
    def __init__(self, robot):
        """
        初始化无人机控制器
        :param robot: Webots机器人实例
        """
        self.robot = robot
        self.time_step = int(self.robot.getBasicTimeStep())
        
        # 初始化设备
        self._init_devices()
        
        # PID控制器参数
        self.altitude_pid = PIDController(kp=3.0)
        self.roll_pid = PIDController(kp=50.0)
        self.pitch_pid = PIDController(kp=30.0)
        self.yaw_pid = PIDController(kp=2.0, ki=0.2)
        
        # 无人机状态
        self.current_pose = [0, 0, 0, 0, 0, 0]  # [X,Y,Z,roll,pitch,yaw]
        self.target_pose = [0, 0, 1.5, 0, 0, 0]  # 默认目标高度1.5米
        
        # Mavic2Pro专用参数
        self.K_VERTICAL_THRUST = 68.5  # 基础升力
        self.K_VERTICAL_OFFSET = 0.6   # 垂直偏移补偿
        
    def _init_devices(self):
        """初始化无人机硬件设备"""
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
            motor.setVelocity(1)
    
    def update_sensors(self):
        """更新所有传感器数据"""
        # 获取姿态角(roll, pitch, yaw)
        roll, pitch, yaw = self.imu.getRollPitchYaw()
        # 获取位置坐标(X,Y,Z)
        x_pos, y_pos, altitude = self.gps.getValues()
        # 更新当前状态
        self.current_pose = [x_pos, y_pos, altitude, roll, pitch, yaw]
        return self.current_pose
    
    def set_target_altitude(self, altitude):
        """设置目标高度"""
        self.target_pose[2] = altitude
    
    def set_target_yaw(self, yaw):
        """设置目标偏航角"""
        self.target_pose[5] = yaw
    
    def stabilize(self):
        """
        稳定无人机核心控制函数
        使用PID控制器保持无人机稳定
        """
        # 获取当前传感器数据
        current_altitude = self.current_pose[2]
        roll, pitch, yaw = self.current_pose[3:6]
        roll_acc, pitch_acc, _ = self.gyro.getValues()
        
        # 计算各轴PID控制量
        vertical_input = self.altitude_pid.update(self.target_pose[2] - current_altitude)
        roll_input = self.roll_pid.update(self.target_pose[3] - roll) + roll_acc
        pitch_input = self.pitch_pid.update(self.target_pose[4] - pitch) + pitch_acc
        yaw_input = self.yaw_pid.update(self.target_pose[5] - yaw)
        
        # 计算电机输出 (考虑不同电机旋转方向)
        thrust = self.K_VERTICAL_THRUST + vertical_input
        front_left = thrust - yaw_input + pitch_input - roll_input
        front_right = thrust + yaw_input + pitch_input + roll_input
        rear_left = thrust + yaw_input - pitch_input - roll_input
        rear_right = thrust - yaw_input - pitch_input + roll_input
        
        # 设置电机速度
        self.front_left_motor.setVelocity(front_left)
        self.front_right_motor.setVelocity(-front_right)
        self.rear_left_motor.setVelocity(-rear_left)
        self.rear_right_motor.setVelocity(rear_right)
    
    def capture_image(self, save_folder, image_counter):
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

# class PIDController:
#     """简易PID控制器实现"""
    
#     def __init__(self, kp=0.0, ki=0.0, kd=0.0):
#         self.kp = kp  # 比例系数
#         self.ki = ki  # 积分系数
#         self.kd = kd  # 微分系数
#         self.last_error = 0
#         self.integral = 0
    
#     def update(self, error):
#         """更新PID控制器并返回控制量"""
#         self.integral += error
#         derivative = error - self.last_error
#         self.last_error = error
        
#         return self.kp * error + self.ki * self.integral + self.kd * derivative

class FlightMission:
    """飞行任务管理器，处理高级飞行逻辑"""
    
    def __init__(self, controller):
        """
        初始化飞行任务
        :param controller: 无人机控制器实例
        """
        self.controller = controller
        self.state = "TAKEOFF"  # 初始状态为起飞
        self.start_time = 0
        self.rotation_speed = 15  # 旋转速度(度/秒)
        
        # 图像采集设置
        self.save_folder = 'captured_images'
        self.image_counter = 0
        self.last_capture_time = 0
        self.capture_interval = 1000  # 1秒 = 1000毫秒
        
        # 创建图像保存目录
        if not os.path.exists(self.save_folder):
            os.makedirs(self.save_folder)
            print(f"已创建图像保存目录: {self.save_folder}")
    
    def update_state(self, current_time):
        """
        更新飞行状态机
        :param current_time: 当前时间(毫秒)
        """
        altitude = self.controller.current_pose[2]
        
        if self.state == "TAKEOFF":
            # 起飞阶段：达到目标高度后进入旋转状态
            if altitude > 1.4:  # 接近目标高度1.5米
                self.state = "ROTATE"
                self.start_time = current_time
                print("已达到目标高度，开始旋转")
        
        elif self.state == "ROTATE":
            # 旋转阶段：以15度/秒的速度旋转
            elapsed_time = (current_time - self.start_time) / 1000  # 转换为秒
            rotation_angle = elapsed_time * self.rotation_speed
            
            # 设置目标偏航角
            self.controller.set_target_yaw(np.radians(rotation_angle))
            
            # 检查是否完成360度旋转
            if rotation_angle >= 360:
                self.state = "HOVER"
                print("旋转完成，进入悬停状态")
        
        elif self.state == "HOVER":
            # 悬停状态：保持当前高度和方向
            pass
    
    def capture_image_if_needed(self, current_time):
        """
        检查并执行图像采集
        :param current_time: 当前时间(毫秒)
        :return: 是否采集了图像
        """
        if current_time - self.last_capture_time >= self.capture_interval:
            self.last_capture_time = current_time
            self.image_counter = self.controller.capture_image(
                self.save_folder, self.image_counter)
            return True
        return False
    
    def run(self):
        """执行主飞行循环"""
        print("启动无人机控制程序")
        print("任务流程：起飞到1.5米高度 → 旋转一周 → 保持悬停")
        
        # 设置目标高度
        self.controller.set_target_altitude(1.5)
        
        # 主控制循环
        while self.controller.robot.step(self.controller.time_step) != -1:
            # 更新传感器数据
            self.controller.update_sensors()
            
            # 获取当前时间(毫秒)
            current_time = self.controller.robot.getTime() * 1000
            
            # 更新飞行状态
            self.update_state(current_time)
            
            # 图像采集
            self.capture_image_if_needed(current_time)
            
            # 稳定无人机
            self.controller.stabilize()
        
        print("飞行任务完成")

# 主程序入口
if __name__ == "__main__":
    # 创建机器人实例
    robot = Robot()
    
    # 初始化控制器和任务
    controller = DroneController(robot)
    mission = FlightMission(controller)
    
    # 执行飞行任务
    mission.run()