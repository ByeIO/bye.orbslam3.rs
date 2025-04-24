# 例程的pid控制器封装

# _pid.py
"""PID控制算法实现模块"""

#############################################
# 内置库
import time
# 第三方库
import numpy as np
import pytest
import simple_pid
#############################################
class _PIDController:
    """
    PID控制器实现类
    提供比例-积分-微分控制功能
    
    参数:
        kp (float): 比例增益
        ki (float): 积分增益
        kd (float): 微分增益
        output_limits (tuple): 输出限制 (min, max)
        sample_time (float): 采样时间(秒)
        auto_mode (bool): 是否自动模式(True=自动，False=手动)
    """
    
    def __init__(self, kp=1.0, ki=0.0, kd=0.0, output_limits=(None, None), 
                 sample_time=0.01, auto_mode=True):
        # 增益参数
        self.kp = kp
        self.ki = ki
        self.kd = kd
        
        # 输出限制
        self.output_limits = output_limits
        
        # 采样时间(秒)
        self.sample_time = sample_time
        
        # 控制器模式
        self.auto_mode = auto_mode
        
        # 控制器状态
        self._proportional = 0
        self._integral = 0
        self._derivative = 0
        self._last_error = 0
        self._last_output = 0
        self._last_time = None
        
        # 抗积分饱和
        self._integral_windup_guard = 20.0
        
        # 初始化
        self.reset()
    
    def __call__(self, error, dt=None):
        """
        计算PID输出
        
        参数:
            error (float): 当前误差(设定值-测量值)
            dt (float): 时间步长(秒)。如果为None，则使用预设的sample_time
            
        返回:
            float: PID控制输出
        """
        if not self.auto_mode:
            return self._last_output
            
        # 获取当前时间
        current_time = time.time()
        
        # 计算时间步长
        if dt is None:
            dt = current_time - self._last_time if self._last_time is not None else self.sample_time
        else:
            dt = float(dt)
            
        # 检查时间步长是否有效
        if dt <= 0:
            raise ValueError('dt必须为正数')
            
        # 存储当前时间用于下次计算
        self._last_time = current_time
        
        # 比例项
        self._proportional = self.kp * error
        
        # 积分项(使用梯形积分)
        self._integral += self.ki * (error + self._last_error) * 0.5 * dt
        
        # 抗积分饱和
        if self._integral > self._integral_windup_guard:
            self._integral = self._integral_windup_guard
        elif self._integral < -self._integral_windup_guard:
            self._integral = -self._integral_windup_guard
        
        # 微分项(防止设定值变化导致的微分冲击)
        self._derivative = self.kd * (error - self._last_error) / dt if dt > 0 else 0
        
        # 存储当前误差用于下次计算
        self._last_error = error
        
        # 计算输出
        output = self._proportional + self._integral + self._derivative
        
        # 应用输出限制
        if self.output_limits[0] is not None and output < self.output_limits[0]:
            output = self.output_limits[0]
            # 抗积分饱和(反向限制)
            if self._integral > 0:
                self._integral = 0
        elif self.output_limits[1] is not None and output > self.output_limits[1]:
            output = self.output_limits[1]
            # 抗积分饱和(反向限制)
            if self._integral < 0:
                self._integral = 0
        
        # 存储最后输出
        self._last_output = output
        
        return output
    
    def update(self, error, dt=None):
        """
        更新PID控制器(与__call__相同)
        """
        return self(error, dt)
    
    def reset(self):
        """
        重置PID控制器状态
        """
        self._proportional = 0
        self._integral = 0
        self._derivative = 0
        self._last_error = 0
        self._last_output = 0
        self._last_time = None
    
    def set_auto_mode(self, auto_mode, last_output=None):
        """
        设置控制器模式
        
        参数:
            auto_mode (bool): 是否自动模式
            last_output (float): 切换到自动模式时的初始输出值
        """
        if auto_mode and not self.auto_mode:
            # 从手动切换到自动
            self.reset()
            if last_output is not None:
                self._last_output = last_output
        
        self.auto_mode = auto_mode
    
    def set_tunings(self, kp=None, ki=None, kd=None):
        """
        设置PID参数
        
        参数:
            kp (float): 比例增益
            ki (float): 积分增益
            kd (float): 微分增益
        """
        if kp is not None:
            self.kp = float(kp)
        if ki is not None:
            self.ki = float(ki)
        if kd is not None:
            self.kd = float(kd)
    
    @property
    def components(self):
        """
        获取PID各项分量
        
        返回:
            tuple: (比例项, 积分项, 微分项)
        """
        return self._proportional, self._integral, self._derivative
    
    @property
    def tunings(self):
        """
        获取当前PID参数
        
        返回:
            tuple: (kp, ki, kd)
        """
        return self.kp, self.ki, self.kd

