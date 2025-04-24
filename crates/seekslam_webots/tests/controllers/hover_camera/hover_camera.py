# 起飞，悬停空中并以15度每秒的速度自转一周，期间以1Hz速度保存camera图片到文件夹
# 自旋完成后保持在1.5m高度悬停

from controller import Robot
import sys
import time
import numpy as np
import cv2
import os
from collections import deque

def clamp(value, value_min, value_max):
    """将数值限制在最小值和最大值之间"""
    return min(max(value, value_min), value_max)


class Mavic(Robot):
    # 控制常量
    K_VERTICAL_THRUST = 68.5  # 基础升力
    K_VERTICAL_OFFSET = 0.6   # 垂直偏移补偿
    
    def __init__(self):
        Robot.__init__(self)
        self.time_step = int(self.getBasicTimeStep())

        # 初始化传感器和相机
        self.camera = self.getDevice("camera")
        self.camera.enable(self.time_step)
        self.imu = self.getDevice("inertial unit")
        self.imu.enable(self.time_step)
        self.gps = self.getDevice("gps")
        self.gps.enable(self.time_step)
        self.gyro = self.getDevice("gyro")
        self.gyro.enable(self.time_step)

        # 初始化电机
        self.front_left_motor = self.getDevice("front left propeller")
        self.front_right_motor = self.getDevice("front right propeller")
        self.rear_left_motor = self.getDevice("rear left propeller")
        self.rear_right_motor = self.getDevice("rear right propeller")
        
        motors = [self.front_left_motor, self.front_right_motor,
                  self.rear_left_motor, self.rear_right_motor]
        for motor in motors:
            motor.setPosition(float('inf'))
            motor.setVelocity(0)  # 初始速度为0

        # 飞行状态变量
        self.current_pose = [0, 0, 0, 0, 0, 0]  # X,Y,Z,roll,pitch,yaw
        self.target_altitude = 1.5  # 目标高度1.5米
        self.state = "TAKEOFF"      # 初始状态为起飞
        self.hover_start_time = 0    # 悬停开始时间
        self.rotation_start_time = 0 # 旋转开始时间
        self.target_yaw = 0         # 目标偏航角
        self.total_rotation = 0     # 累计旋转角度
        self.rotation_speed = 15    # 旋转速度(度/秒)
        
        # 图像保存设置
        self.save_folder = 'captured_images'
        self.image_counter = 0
        self.last_capture_time = 0
        self.capture_interval = 1000  # 1秒 = 1000毫秒
        
        # LQR控制器参数
        self.Q = np.diag([10, 10, 10, 1, 1, 1])  # 状态权重矩阵
        self.R = np.diag([0.1, 0.1, 0.1, 0.1])   # 控制输入权重矩阵
        self.error_history = deque(maxlen=10)    # 误差历史记录
        
        # 创建保存目录
        if not os.path.exists(self.save_folder):
            os.makedirs(self.save_folder)
            print(f"已创建图像保存目录: {self.save_folder}")

    def set_position(self, pos):
        """更新当前姿态"""
        self.current_pose = pos

    def capture_image(self):
        """捕获当前相机图像并保存"""
        # 获取图像数据
        image_data = self.camera.getImage()
        
        if image_data:
            # 转换为OpenCV格式 (BGRA)
            width = self.camera.getWidth()
            height = self.camera.getHeight()
            image = np.frombuffer(image_data, dtype=np.uint8).reshape((height, width, 4))
            
            # 转换为BGR格式 (去掉Alpha通道)
            image_bgr = cv2.cvtColor(image, cv2.COLOR_BGRA2BGR)
            
            # 保存图像
            filename = os.path.join(self.save_folder, f"img_{self.image_counter:04d}.jpg")
            cv2.imwrite(filename, image_bgr)
            
            print(f"已保存图像: {filename}")
            self.image_counter += 1

    def lqr_controller(self, current_state, desired_state):
        """
        LQR控制器实现
        current_state: [x, y, z, roll, pitch, yaw]
        desired_state: [x_d, y_d, z_d, roll_d, pitch_d, yaw_d]
        返回: 控制输入 [vertical_input, roll_input, pitch_input, yaw_input]
        """
        # 计算状态误差
        error = np.array(desired_state) - np.array(current_state)
        self.error_history.append(error)
        
        # 计算误差积分 (用于消除稳态误差)
        error_integral = np.sum(self.error_history, axis=0) if self.error_history else np.zeros(6)
        
        # 状态向量 (包含误差和误差积分)
        x = np.concatenate((error, 0.1 * error_integral))
        
        # 简化的LQR控制计算 (实际应用中需要求解Riccati方程)
        # 这里使用简化的反馈控制代替完整的LQR计算
        K = np.array([
            [0, 0, 3.0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0],  # 垂直控制
            [0, 0, 0, 50.0, 0, 0, 0, 0, 0, 5.0, 0, 0],  # 横滚控制
            [0, 0, 0, 0, 30.0, 0, 0, 0, 0, 0, 3.0, 0],  # 俯仰控制
            [0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0.2]    # 偏航控制
        ])
        
        # 计算控制输入
        u = np.dot(K, x)
        
        vertical_input = u[0]
        roll_input = u[1]
        pitch_input = u[2]
        yaw_input = u[3]
        
        return vertical_input, roll_input, pitch_input, yaw_input

    def update_state_machine(self, altitude, current_time):
        """
        更新飞行状态机
        altitude: 当前高度
        current_time: 当前时间(毫秒)
        """
        if self.state == "TAKEOFF":
            # 起飞阶段：达到目标高度后进入悬停状态
            if altitude > self.target_altitude - 0.1:  # 达到目标高度附近
                self.state = "ROTATE"
                self.hover_start_time = self.getTime()
                self.rotation_start_time = self.getTime()
                print("已达到目标高度，开始悬停并旋转")
        
        elif self.state == "ROTATE":
            # 悬停并旋转阶段
            elapsed_time = self.getTime() - self.rotation_start_time
            self.total_rotation = elapsed_time * self.rotation_speed
            
            # 检查是否完成一周旋转(360度)
            if self.total_rotation >= 360:
                self.state = "HOVER"
                print("旋转完成，保持悬停")
            elif elapsed_time > 30.0:  # 超时保护
                self.state = "HOVER"
                print("超时自动悬停")
                
        elif self.state == "HOVER":
            self.target_altitude = 1.5  # 确保目标高度为1.5米

    def run(self):
        """主控制循环"""
        while self.step(self.time_step) != -1:
            current_time = self.getTime() * 1000  # 转换为毫秒
            
            # 读取传感器数据
            roll, pitch, yaw = self.imu.getRollPitchYaw()
            x_pos, y_pos, altitude = self.gps.getValues()
            roll_acceleration, pitch_acceleration, _ = self.gyro.getValues()
            self.set_position([x_pos, y_pos, altitude, roll, pitch, yaw])

            # 检查是否到达捕获间隔
            if current_time - self.last_capture_time >= self.capture_interval:
                self.last_capture_time = current_time
                self.capture_image()

            # 更新状态机
            self.update_state_machine(altitude, current_time)

            # 根据状态设置目标高度和偏航角
            target_altitude = self.target_altitude
            
            if self.state == "ROTATE":
                # 15度/秒的旋转速度
                self.target_yaw = (self.getTime() - self.rotation_start_time) * self.rotation_speed * (np.pi / 180)
            else:
                self.target_yaw = yaw  # 保持当前偏航角

            # 使用LQR控制器计算控制输入
            desired_state = [0, 0, target_altitude, 0, 0, self.target_yaw]  # 期望状态
            vertical_input, roll_input, pitch_input, yaw_input = self.lqr_controller(
                [x_pos, y_pos, altitude, roll, pitch, yaw], desired_state)

            # 添加陀螺仪反馈
            roll_input += roll_acceleration
            pitch_input += pitch_acceleration

            # 计算电机输入（保持稳定）
            front_left = self.K_VERTICAL_THRUST + vertical_input - yaw_input + pitch_input - roll_input
            front_right = self.K_VERTICAL_THRUST + vertical_input + yaw_input + pitch_input + roll_input
            rear_left = self.K_VERTICAL_THRUST + vertical_input + yaw_input - pitch_input - roll_input
            rear_right = self.K_VERTICAL_THRUST + vertical_input - yaw_input - pitch_input + roll_input

            # 设置电机速度（注意不同电机旋转方向不同）
            self.front_left_motor.setVelocity(front_left)
            self.front_right_motor.setVelocity(-front_right)
            self.rear_left_motor.setVelocity(-rear_left)
            self.rear_right_motor.setVelocity(rear_right)


# 创建并运行控制器
robot = Mavic()
print("启动无人机控制程序")
print("起飞到1.5米高度，悬停并旋转一周，然后保持悬停...")
robot.run()
print("程序正常结束")