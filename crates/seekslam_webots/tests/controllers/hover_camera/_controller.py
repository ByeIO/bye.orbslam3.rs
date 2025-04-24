# 顶层控制模块封装
# 运动学模型: Dji Mavic2Pro

#############################################
# 内置库
import time
# 第三方库
from controller import Robot
import numpy as np
import pytest
# 项目库
from .pid import PIDController
#############################################

class DroneController:
    """无人机简化控制接口"""
    def __init__(self):
        self.robot = Robot()
        self.time_step = int(self.robot.getBasicTimeStep())
        
        # 初始化设备
        self._init_devices()
        
        # 初始化PID控制器
        self.altitude_pid = PIDController(kp=3.0)
        self.roll_pid = PIDController(kp=50.0)
        self.pitch_pid = PIDController(kp=30.0)
        
        # 状态变量
        self.current_pose = [0, 0, 0, 0, 0, 0]  # [X,Y,Z,roll,pitch,yaw]
        self.target_altitude = 0
        self.is_flying = False
        # Mavic2Pro专用参数
        self.K_VERTICAL_THRUST = 68.5
        self.K_VERTICAL_OFFSET = 0.6
        
    def _init_devices(self):
        """初始化无人机设备"""
        self.camera = self.robot.getDevice("camera")
        self.camera.enable(self.time_step)
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
        
        motors = [self.front_left_motor, self.front_right_motor,
                  self.rear_left_motor, self.rear_right_motor]
        for motor in motors:
            motor.setPosition(float('inf'))
            motor.setVelocity(1)
    
    def _update_sensors(self):
        """更新传感器数据"""
        roll, pitch, yaw = self.imu.getRollPitchYaw()
        x_pos, y_pos, altitude = self.gps.getValues()
        self.current_pose = [x_pos, y_pos, altitude, roll, pitch, yaw]
    
    def _set_motors(self, front_left, front_right, rear_left, rear_right):
        """设置电机速度"""
        self.front_left_motor.setVelocity(front_left)
        self.front_right_motor.setVelocity(-front_right)
        self.rear_left_motor.setVelocity(-rear_left)
        self.rear_right_motor.setVelocity(rear_right)
    
    def takeoff(self, altitude=1.5):
        """起飞到指定高度"""
        self.target_altitude = altitude
        self.is_flying = True
        
        while self.robot.step(self.time_step) != -1 and self.is_flying:
            self._update_sensors()
            current_altitude = self.current_pose[2]
            
            if current_altitude >= self.target_altitude - 0.1:
                break
                
            # PID控制高度
            vertical_input = self.altitude_pid.update(current_altitude)
            thrust = self.K_VERTICAL_THRUST + vertical_input
            
            # 设置电机速度
            self._set_motors(thrust, thrust, thrust, thrust)
    
    def land(self):
        """降落"""
        self.target_altitude = 0
        self.is_flying = False
        
        while self.robot.step(self.time_step) != -1:
            self._update_sensors()
            current_altitude = self.current_pose[2]
            
            if current_altitude <= 0.1:
                self._set_motors(0, 0, 0, 0)
                break
                
            # PID控制高度
            vertical_input = self.altitude_pid.update(current_altitude)
            thrust = max(0, self.K_VERTICAL_THRUST + vertical_input)
            
            # 设置电机速度
            self._set_motors(thrust, thrust, thrust, thrust)
    
    def move_forward(self, distance_cm):
        """向前移动指定距离(cm)"""
        distance_m = distance_cm / 100
        start_pos = self.current_pose[0:2]
        
        while self.robot.step(self.time_step) != -1 and self.is_flying:
            self._update_sensors()
            current_pos = self.current_pose[0:2]
            
            # 计算已移动距离
            moved_distance = np.sqrt((current_pos[0] - start_pos[0])**2 + 
                                    (current_pos[1] - start_pos[1])**2)
            
            if moved_distance >= distance_m:
                break
                
            # 控制逻辑
            self._stabilize(forward_thrust=0.5)
    
    def move_left(self, distance_cm):
        """向左移动指定距离(cm)"""
        distance_m = distance_cm / 100
        start_pos = self.current_pose[0:2]
        
        while self.robot.step(self.time_step) != -1 and self.is_flying:
            self._update_sensors()
            current_pos = self.current_pose[0:2]
            
            # 计算已移动距离
            moved_distance = np.sqrt((current_pos[0] - start_pos[0])**2 + 
                                    (current_pos[1] - start_pos[1])**2)
            
            if moved_distance >= distance_m:
                break
                
            # 控制逻辑
            self._stabilize(left_thrust=0.5)
    
    def rotate_counter_clockwise(self, degrees):
        """逆时针旋转指定角度"""
        target_angle = self.current_pose[5] + np.radians(degrees)
        
        while self.robot.step(self.time_step) != -1 and self.is_flying:
            self._update_sensors()
            current_yaw = self.current_pose[5]
            
            # 角度差
            angle_diff = (target_angle - current_yaw + np.pi) % (2 * np.pi) - np.pi
            
            if abs(angle_diff) < np.radians(2):  # 2度容差
                break
                
            # 控制逻辑
            self._stabilize(yaw_thrust=0.3 if angle_diff > 0 else -0.3)
    
    def hover(self, altitude=None):
        """悬停在当前高度或指定高度"""
        if altitude is not None:
            self.target_altitude = altitude
            
        while self.robot.step(self.time_step) != -1 and self.is_flying:
            self._update_sensors()
            self._stabilize()
    
    def _stabilize(self, forward_thrust=0, left_thrust=0, yaw_thrust=0):
        """稳定无人机并应用控制输入"""
        current_altitude = self.current_pose[2]
        roll, pitch, yaw = self.current_pose[3:6]
        roll_acceleration, pitch_acceleration, _ = self.gyro.getValues()
        
        # PID控制
        vertical_input = self.altitude_pid.update(current_altitude)
        roll_input = self.roll_pid.update(roll) + roll_acceleration
        pitch_input = self.pitch_pid.update(pitch) + pitch_acceleration
        
        # 应用额外控制输入
        pitch_input -= forward_thrust
        roll_input -= left_thrust
        
        # 计算电机输入
        thrust = self.K_VERTICAL_THRUST + vertical_input
        front_left = thrust - yaw_thrust + pitch_input - roll_input
        front_right = thrust + yaw_thrust + pitch_input + roll_input
        rear_left = thrust + yaw_thrust - pitch_input - roll_input
        rear_right = thrust - yaw_thrust - pitch_input + roll_input
        
        # 设置电机速度
        self._set_motors(front_left, front_right, rear_left, rear_right)