# 顶层控制模块封装
# 运动学模型: Dji Mavic2Pro

#############################################
# 内置库
import time
import csv
import os
import math
# 第三方库
import numpy as np
import pytest
import asyncio
# 项目库
from _trajectory import _track
#############################################

class _DroneController:
    """无人机控制封装类，使用supervisor直接控制无人机位置和姿态"""
    
    def __init__(self, drone_super):
        """
        初始化无人机控制器
        :param drone: Webots无人机超级实例，需包含传感器和电机设备
        """
        self.drone = drone_super
        
        # 获取无人机节点和地板节点
        self.drone_node = self.drone.getFromDef("MY_ROBOT4")  # 使用你的无人机DEF名称
        self.floor_node = self.drone.getFromDef("FLOOR")     # 获取地板节点
        self.pad_node = self.drone.getFromDef("PAD") # 获取无人机减震垫板节点
        
        if self.drone_node is None:
            raise ValueError("无法获取无人机节点，请检查DEF名称是否正确")
        if self.floor_node is None:
            raise ValueError("无法获取地板节点，请检查DEF名称是否正确")
            
        # 获取字段(直角坐标, 旋转轴角)
        self.drone_translation = self.drone_node.getField("translation")
        self.drone_rotation = self.drone_node.getField("rotation")
        self.floor_translation = self.floor_node.getField("translation")
        self.floor_rotation = self.floor_node.getField("rotation")
        self.pad_translation = self.pad_node.getField("translation")
        self.pad_rotation = self.pad_node.getField("rotation")
        
        if self.drone_translation is None or self.drone_rotation is None:
            raise ValueError("无法获取无人机的位置或旋转字段")

        # 转换单位(旋转轴角转欧拉角)
        self.drone_pose = [0, 0, 0, 0, 0, 0]

        # 禁用物理模拟或设置为悬浮状态
        self._disable_physics()

        # 初始化时间步长（单位：ms）
        self.time_step = self.drone.time_step
        
        # 固定姿态（与地板平行）
        self.fixed_roll = 0
        self.fixed_pitch = 0

        # 旋转值记录, 初始值为-2.0
        self.rot = -2.0

    def _disable_physics(self):
        """禁用无人机的物理模拟"""
        # 方法1：设置无人机为悬浮状态（如果无人机有该属性）
        if self.drone_node.getField("physics") is not None:
            self.drone_node.getField("physics").setSFString("none")
        
        # 方法2：禁用物理模拟
        if self.drone_node.getField("physicsDisable") is not None:
            self.drone_node.getField("physicsDisable").setSFBool(True)
        
        # 方法3：设置质量为零（不推荐，可能影响其他模拟）
        # if self.drone_node.getField("mass") is not None:
        #     self.drone_node.getField("mass").setSFFloat(0.0)

    def _update_sensors(self):
        """更新传感器数据并返回当前姿态"""
        # 使用Supervisor方法获取位置和旋转
        position = self.drone_translation.getSFVec3f()
        rotation = self.drone_rotation.getSFRotation()
        
        # 转换为欧拉角
        roll, pitch, yaw = self._rotation_to_euler(rotation)
        
        # 更新当前状态(欧拉角, 有万向节锁问题)
        self.drone_pose = [position[0], position[1], position[2], roll, pitch, yaw]
        return self.drone_pose

    def _rotation_to_euler(self, rotation):
        """将Webots的旋转轴角转换为欧拉角(roll, pitch, yaw)"""
        # Webots rotation是轴角表示法 [x, y, z, angle]
        axis = rotation[:3]
        angle = rotation[3]
        
        # 简化处理：假设旋转轴是Z轴
        if np.allclose(axis, [0, 0, 1]):
            # roll, pitch, yaw
            return 0, 0, angle  
        else:
            # 更复杂的转换在实际应用中需要实现
            return 0, 0, angle
    def _euler_to_axis_angle(self, roll, pitch, yaw) -> [float, float, float, float]:
        """
        将欧拉角(roll, pitch, yaw)转换为旋转轴角表示法 [x, y, z, angle]
        
        参数:
            roll: 绕X轴旋转的角度(弧度)
            pitch: 绕Y轴旋转的角度(弧度)
            yaw: 绕Z轴旋转的角度(弧度)
            
        返回:
            list: 轴角表示法 [x, y, z, angle]
        """
        # 计算旋转矩阵
        # 这里使用ZYX顺序(先yaw，再pitch，最后roll)
        cy = math.cos(yaw)
        sy = math.sin(yaw)
        cp = math.cos(pitch)
        sp = math.sin(pitch)
        cr = math.cos(roll)
        sr = math.sin(roll)
        
        # 组合旋转矩阵 R = Rz * Ry * Rx
        rotation_matrix = np.array([
            [cy*cp, cy*sp*sr - sy*cr, cy*sp*cr + sy*sr],
            [sy*cp, sy*sp*sr + cy*cr, sy*sp*cr - cy*sr],
            [-sp, cp*sr, cp*cr]
        ])
        
        # 从旋转矩阵中提取轴角
        angle = math.acos((np.trace(rotation_matrix) - 1) / 2)
        
        # 避免除以零(当angle接近0时)
        if angle < 1e-10:
            return [0, 0, 1, 0]  # 无旋转，默认Z轴
        
        axis = np.array([
            rotation_matrix[2,1] - rotation_matrix[1,2],
            rotation_matrix[0,2] - rotation_matrix[2,0],
            rotation_matrix[1,0] - rotation_matrix[0,1]
        ]) / (2 * math.sin(angle))
        
        # 归一化轴向量
        axis = axis / np.linalg.norm(axis)
        
        # return [axis[0], axis[1], axis[2], angle]
        
        # 仅绕Z轴旋转
        # 角度转弧度
        target_yaw = np.deg2rad(yaw) # + self.drone_rotation.getSFRotation()[3]
        # -2.0是无人机的初始rad, 好像是表示绝对旋转的...
        # return [0.0, 0.0, 1.0, -2.0 + target_yaw]
        this_yaw = self._update_rot(target_yaw)
        return [0.0, 0.0, 1.0, this_yaw]

    def _update_rot(self, value):
        '''维护旋转值的记录'''
        self.rot = self.rot + value
        return self.rot

    def _get_floor_rotation(self):
        """获取地板的旋转角度"""
        floor_rotation = self.floor_rotation.getSFRotation()
        # 返回地板绕Z轴的旋转角度
        if np.allclose(floor_rotation[:3], [0, 0, 1]):
            return floor_rotation[3]
        return 0

    def _move_drone_to_target(self, relative_translation, relative_rotation) -> bool:
        """基于当前位置的相对移动无人机到目标位置"""
        # 强制重置物理状态
        self.drone_node.resetPhysics()
        
        # 获取当前位置
        drone_current_translation = self.drone_translation.getSFVec3f()
        drone_current_rotation = self.drone_rotation.getSFRotation()

        # 相对位移
        dx_self = relative_translation[0]
        dy_self = relative_translation[1]
        dz = relative_translation[2]

        # 获取当前偏航角（yaw）
        current_yaw = drone_current_rotation[2]  # 假设rotation格式为[axis_x, axis_y, axis_z, angle]
    
        # 将局部坐标位移转换为全局坐标位移
        dx_global = dx_self * math.cos(current_yaw) - dy_self * math.sin(current_yaw)
        dy_global = dx_self * math.sin(current_yaw) + dy_self * math.cos(current_yaw)
    
        # 符合直觉表示的欧拉角
        d_roll = relative_rotation[0]
        d_pitch = relative_rotation[1]
        d_yaw = relative_rotation[2]
    
        # 新位置(drone_current_translation是全局坐标, dx_self是机器人局部坐标)
        new_translation = [
            drone_current_translation[0] + dx_global,
            drone_current_translation[1] + dy_global,
            drone_current_translation[2] + dz
        ]
        
        # 计算新的旋转
        current_axis_angle = self._euler_to_axis_angle(d_roll, d_pitch, d_yaw)
        new_rotation = [
            # drone_current_rotation[0] + current_axis_angle[0],
            current_axis_angle[0],
            # drone_current_rotation[1] + current_axis_angle[1],
            current_axis_angle[1],
            # drone_current_rotation[2] + current_axis_angle[2],
            current_axis_angle[2],
            # drone_current_rotation[3] + current_axis_angle[3]
            current_axis_angle[3],
        ]
        
        # 直接设置新位置
        self.drone_translation.setSFVec3f(new_translation)
        # 如果是水平移动则不更新旋转值
        if not (d_roll == 0.0 and d_pitch == 0.0 and d_yaw == 0.0):
            self.drone_rotation.setSFRotation(new_rotation)
        # 立即更新当前状态
        self._update_sensors()
        
        return True
    
    def _move_pad_to_target(self, relative_translation, relative_rotation) -> bool:
        """基于当前位置的相对移动垫板到目标位置"""
        # 获取当前位置
        pad_current_translation = self.pad_translation.getSFVec3f()
        pad_current_rotation = self.pad_rotation.getSFRotation()
    
        # 相对位移
        dx_self = relative_translation[0]
        dy_self = relative_translation[1]
        dz = relative_translation[2]
    
        # 符合直觉表示的欧拉角
        d_roll = relative_rotation[0]
        d_pitch = relative_rotation[1]
        d_yaw = relative_rotation[2]

        # 获取当前偏航角（yaw）
        current_yaw = pad_current_rotation[2]  # 假设rotation格式为[axis_x, axis_y, axis_z, angle]

        # 将局部坐标位移转换为全局坐标位移(默认就是使用弧度进行计算)
        dx_global = dx_self * math.cos(current_yaw) - dy_self * math.sin(current_yaw)
        dy_global = dx_self * math.sin(current_yaw) + dy_self * math.cos(current_yaw)
    
        # 新位置(drone_current_translation是全局坐标, dx_self是机器人局部坐标)
        new_translation = [
            pad_current_translation[0] + dx_global,
            pad_current_translation[1] + dy_global,
            pad_current_translation[2] + dz
        ]
        
        # 计算新的旋转
        current_axis_angle = self._euler_to_axis_angle(d_roll, d_pitch, d_yaw)
        new_rotation = [
            # pad_current_rotation[0] + current_axis_angle[0],
            current_axis_angle[0],
            # pad_current_rotation[1] + current_axis_angle[1],
            current_axis_angle[1],
            # pad_current_rotation[2] + current_axis_angle[2],
            current_axis_angle[2],
            # pad_current_rotation[3] + current_axis_angle[3]
            current_axis_angle[3],
        ]
        
        # 直接设置新位置
        self.pad_translation.setSFVec3f(new_translation)

        # 如果是水平移动则不更新旋转值
        if not (d_roll == 0.0 and d_pitch == 0.0 and d_yaw == 0.0):
            self.pad_rotation.setSFRotation(new_rotation)
        
        return True

    def _move_to_target(self, relative_translation, relative_rotation):
        '''先移动垫板, 然后再移动无人机'''
        self._move_pad_to_target(relative_translation, relative_rotation)
        # self._move_drone_to_target(relative_translation, relative_rotation)
        pass

    #######################################################################
    def hover(self) -> bool:
        """保持当前高度悬停"""
        # 保持当前姿态
        self._update_sensors()
        # 不动即可
        return True

    def take_off(self, target_altitude=150) -> bool:
        """起飞"""
        # 计算z轴距离(m)
        dz = target_altitude / 100.0
        # 移动
        self._move_to_target([0, 0, dz], [0, 0, 0])
        # 记录轨迹
        _track({"name":"up", "value":target_altitude})
        # 返回
        return True

    def land(self) -> bool:
        """降落至地面"""
        self._update_sensors()
        temp = self.drone_translation.getSFVec3f()[2]
        # 移动
        self._move_to_target([0.0, 0.0, 0.0], [0, 0, 0])
        # 记录轨迹
        _track({"name":"down", "value":temp})
        # 返回
        return True
        
    def rotate(self, degrees) -> bool:
        """绕z轴自旋指定角度（度）"""
        self._update_sensors()
        # 修正参数(临时解决方案)
        _degrees = degrees / 2.0
        # 移动
        self._move_to_target([0.0, 0.0, 0.0], [0, 0, degrees])  
        # 记录轨迹
        _track({"name":"rotate", "value": degrees})
        # 返回
        return True

    def move_forward(self, cm) -> bool:
        '''前进x厘米'''
        m = cm / 100.0
        # 更新传感器
        self._update_sensors()
        # 移动(负号调整方向)
        self._move_to_target([-m, 0.0, 0.0], [0.0, 0.0, 0.0])  
        # 记录轨迹
        _track({"name":"forward", "value": m})
        # 返回
        return True

    def move_backward(self, cm) -> bool:
        '''后退x厘米'''
        m = cm / 100.0
        # 更新传感器
        self._update_sensors()
        # 移动(负号调整方向)
        self._move_to_target([m, 0.0, 0.0], [0.0, 0.0, 0.0])  
        # 记录轨迹
        _track({"name":"backward", "value": m})
        # 返回
        return True

    def move_left(self, cm) -> bool:
        '''左移x厘米'''
        m = cm / 100.0
        # 更新传感器
        self._update_sensors()
        # 移动(负号调整方向)
        self._move_to_target([0.0, -m, 0.0], [0.0, 0.0, 0.0])  
        # 记录轨迹
        _track({"name":"left", "value": m})
        # 返回
        return True

    def move_right(self, cm) -> bool:
        '''右移x厘米'''
        m = cm / 100.0
        # 更新传感器
        self._update_sensors()
        # 移动(负号调整方向)
        self._move_to_target([0.0, m, 0.0], [0.0, 0.0, 0.0])  
        # 记录轨迹
        _track({"name":"right", "value": m})
        # 返回
        return True

    def move_up(self, cm) -> bool:
        '''上移x厘米'''
        m = cm / 100.0
        # 更新传感器
        self._update_sensors()
        # 移动(负号调整方向)
        self._move_to_target([0.0, 0.0, m], [0.0, 0.0, 0.0])  
        # 记录轨迹
        _track({"name":"up", "value": m})
        # 返回
        return True

    def move_down(self, cm) -> bool:
        '''下移x厘米'''
        m = cm / 100.0
        # 更新传感器
        self._update_sensors()
        # 移动(负号调整方向)
        self._move_to_target([0.0, 0.0, -m], [0.0, 0.0, 0.0])  
        # 记录轨迹
        _track({"name":"down", "value": m})
        # 返回
        return True
    def attack(self, is_open:bool) -> bool:
        '''打开激光器'''
        if is_open:
            print("激光器已打开")
            return True
        else:
            print("激光器已关闭")
            return False
        return True