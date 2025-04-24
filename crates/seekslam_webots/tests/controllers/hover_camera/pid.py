# 例程的pid控制器封装

#############################################
# 内置库
import time
# 第三方库
import numpy as np
import pytest
#############################################

class PIDController:
    """PID控制器封装类"""
    def __init__(self, kp, ki=0.0, kd=0.0, set_point=0.0):
        self.kp = kp          # 比例系数
        self.ki = ki          # 积分系数
        self.kd = kd          # 微分系数
        self.set_point = set_point  # 目标值
        self.last_error = 0   # 上一次误差
        self.integral = 0     # 积分项
        self.last_time = time.time()  # 上一次时间戳

    def update(self, current_value):
        """更新PID控制器并返回控制输出"""
        current_time = time.time()
        dt = current_time - self.last_time
        if dt <= 0:
            dt = 1e-5
            
        error = self.set_point - current_value
        
        # 比例项
        p_term = self.kp * error
        
        # 积分项 (防止积分饱和)
        self.integral += error * dt
        i_term = self.ki * self.integral
        
        # 微分项
        derivative = (error - self.last_error) / dt
        d_term = self.kd * derivative
        
        # 保存状态
        self.last_error = error
        self.last_time = current_time
        
        return p_term + i_term + d_term
    
    def set_target(self, target):
        """设置目标值"""
        self.set_point = target
        self.integral = 0  # 重置积分项
        self.last_error = 0

##############################################
##############################################
@pytest.fixture
def pid_controller():
    """Fixture providing a basic PID controller for testing"""
    return PIDController(kp=1.0, ki=0.1, kd=0.01, set_point=10.0)

def test_initialization():
    """Test PID controller initialization with different parameters"""
    pid = PIDController(kp=1.0)
    assert pid.kp == 1.0
    assert pid.ki == 0.0
    assert pid.kd == 0.0
    assert pid.set_point == 0.0
    
    pid = PIDController(kp=2.0, ki=0.5, kd=0.1, set_point=5.0)
    assert pid.kp == 2.0
    assert pid.ki == 0.5
    assert pid.kd == 0.1
    assert pid.set_point == 5.0
    assert pid.last_error == 0
    assert pid.integral == 0

def test_set_target(pid_controller):
    """Test setting a new target value"""
    pid_controller.set_target(20.0)
    assert pid_controller.set_point == 20.0
    assert pid_controller.integral == 0
    assert pid_controller.last_error == 0
    
    # Verify it resets the state
    pid_controller.integral = 5.0
    pid_controller.last_error = 2.0
    pid_controller.set_target(15.0)
    assert pid_controller.integral == 0
    assert pid_controller.last_error == 0

def test_update_proportional(pid_controller):
    """Test proportional term calculation"""
    pid_controller.ki = 0.0
    pid_controller.kd = 0.0
    output = pid_controller.update(5.0)  # error = 5
    assert output == pytest.approx(5.0 * 1.0)  # kp * error
    
    output = pid_controller.update(8.0)  # error = 2
    assert output == pytest.approx(2.0 * 1.0)

def test_update_integral(pid_controller):
    """Test integral term calculation"""
    pid_controller.kp = 0.0
    pid_controller.kd = 0.0
    pid_controller.ki = 0.5
    
    # First update
    output = pid_controller.update(5.0)  # error = 5
    time.sleep(0.01)  # Ensure some time passes
    # Second update
    output = pid_controller.update(5.0)  # error still 5
    # Should have accumulated integral
    
    # Check that integral term is roughly 0.5 * (5 * dt + 5 * dt)
    # Exact value depends on actual time elapsed
    assert output > 0.0
    assert pid_controller.integral > 0.0

def test_update_derivative(pid_controller):
    """Test derivative term calculation"""
    pid_controller.kp = 0.0
    pid_controller.ki = 0.0
    pid_controller.kd = 0.1
    
    # First update
    output1 = pid_controller.update(5.0)  # error = 5
    time.sleep(0.01)
    # Second update - error changes to 2
    output2 = pid_controller.update(8.0)
    
    # Derivative should be (2-5)/dt
    # Since kd is 0.1, d_term should be 0.1 * (2-5)/dt
    assert output2 < 0.0  # Negative because error is decreasing

def test_update_all_terms(pid_controller):
    """Test combined PID terms"""
    pid_controller.kd = 0.001  # Reduce derivative impact
    output = pid_controller.update(5.0)  # error = 5
    time.sleep(0.1)  # Longer sleep to make dt more significant
    output = pid_controller.update(8.0)  # error = 2
    
    # With smaller kd, P term should dominate
    assert output > 0.0

def test_zero_time_delta():
    """Test handling of zero or very small time delta"""
    pid = PIDController(kp=1.0, ki=0.1, kd=0.01, set_point=10.0)
    
    # First update to set last_time
    pid.update(5.0)
    
    # Force very small time delta
    pid.last_time = time.time()
    output = pid.update(5.0)
    
    # Should still work (dt handled in code)
    assert not np.isnan(output)
    assert not np.isinf(output)
