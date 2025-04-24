# 测试起飞、悬停和降落mav2pro

# webots2025a环境 

from controller import Robot
import sys
import time
import numpy as np

def clamp(value, value_min, value_max):
    """将数值限制在最小值和最大值之间"""
    return min(max(value, value_min), value_max)


class Mavic (Robot):
    # 控制常量
    K_VERTICAL_THRUST = 68.5  # 基础升力
    K_VERTICAL_OFFSET = 0.6   # 垂直偏移补偿
    K_VERTICAL_P = 3.0        # 垂直PID的P参数
    K_ROLL_P = 50.0           # 横滚PID的P参数
    K_PITCH_P = 30.0          # 俯仰PID的P参数

    def __init__(self):
        Robot.__init__(self)
        self.time_step = int(self.getBasicTimeStep())

        # 初始化传感器

        # 初始化相机，否则预览画面全黑
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
        self.target_altitude = 2.5  # 目标高度2.5米
        self.state = "TAKEOFF"      # 初始状态为起飞
        self.hover_start_time = 0    # 悬停开始时间

    def set_position(self, pos):
        """更新当前姿态"""
        self.current_pose = pos

    def run(self):
        """主控制循环"""
        while self.step(self.time_step) != -1:
            # 读取传感器数据
            roll, pitch, yaw = self.imu.getRollPitchYaw()
            x_pos, y_pos, altitude = self.gps.getValues()
            roll_acceleration, pitch_acceleration, _ = self.gyro.getValues()
            self.set_position([x_pos, y_pos, altitude, roll, pitch, yaw])

            # 状态机控制
            if self.state == "TAKEOFF":
                # 起飞阶段：达到目标高度后进入悬停状态
                if altitude > self.target_altitude - 0.1:  # 达到目标高度附近
                    self.state = "HOVER"
                    self.hover_start_time = self.getTime()
                    print("已达到目标高度，开始悬停")
            
            elif self.state == "HOVER":
                # 悬停阶段：保持5秒后进入降落状态
                if self.getTime() - self.hover_start_time > 5.0:
                    self.state = "LAND"
                    print("悬停完成，开始降落")
            
            elif self.state == "LAND":
                # 降落阶段：当高度低于0.3米时关闭电机并退出
                if altitude < 0.3:
                    self.state = "SHUTDOWN"
                    print("降落完成，关闭电机")
                    # 关闭所有电机
                    # self.front_left_motor.setVelocity(0)
                    # self.front_right_motor.setVelocity(0)
                    # self.rear_left_motor.setVelocity(0)
                    # self.rear_right_motor.setVelocity(0)
                    return  # 退出程序
            
            # 根据状态设置目标高度
            if self.state == "LAND":
                target_altitude = 0  # 降落目标高度为0
            else:
                target_altitude = self.target_altitude

            # PID控制计算
            roll_input = self.K_ROLL_P * clamp(roll, -1, 1) + roll_acceleration
            pitch_input = self.K_PITCH_P * clamp(pitch, -1, 1) + pitch_acceleration
            yaw_input = 0  # 不需要偏航控制
            
            # 高度控制
            clamped_altitude_diff = clamp(target_altitude - altitude + self.K_VERTICAL_OFFSET, -1, 1)
            vertical_input = self.K_VERTICAL_P * pow(clamped_altitude_diff, 3.0)

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
print("起飞到2.5米高度...")
robot.run()
print("程序正常结束")