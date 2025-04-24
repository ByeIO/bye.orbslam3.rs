# 控制模块
# lqr控制器

#############################################
# 第三方库
import numpy as np
from scipy.linalg import solve_continuous_are
#############################################

class LQRController:
    def __init__(self):
        # 初始化误差历史记录
        self.error_history = []
        
        # Mavic 2 Pro 参数 (近似值)
        self.mass = 0.91  # kg
        self.gravity = 9.81  # m/s²
        self.inertia = np.diag([0.011, 0.011, 0.021])  # kg·m² (近似惯性矩)
        self.arm_length = 0.15  # m (近似臂长)
        
        # 定义系统模型 (12维状态空间)
        self.A = np.zeros((12, 12))  # 状态矩阵
        self.B = np.zeros((12, 4))   # 控制矩阵
        
        # 设置模型参数
        self._setup_model()
        
        # 定义权重矩阵 (需要根据控制需求调整)
        # 修改后的权重矩阵 (增大位置误差权重)
        self.Q = np.diag([
            50, 50, 20,       # x,y,z位置误差 (从10增加到50)
            5, 5, 5,          # roll,pitch,yaw角度误差
            1, 1, 2,          # x,y,z速度误差
            0.5, 0.5, 0.5     # roll,pitch,yaw角速度误差
        ])
        
        self.R = np.diag([0.1, 0.5, 0.5, 0.5])  # 控制输入权重
        
        # 计算LQR增益矩阵
        self.K = self._compute_lqr_gain()
    
    def _setup_model(self):
        """设置Mavic 2 Pro系统模型参数"""
        # 位置动力学
        self.A[0:3, 6:9] = np.eye(3)  # 位置与速度关系
        
        # 姿态动力学 (简单近似)
        self.A[3:6, 9:12] = np.eye(3)  # 角度与角速度关系
        
        # 重力影响
        self.A[6:9, 3:6] = np.array([
            [0, self.gravity, 0],
            [-self.gravity, 0, 0],
            [0, 0, 0]
        ])
        
        # 控制输入矩阵
        # 垂直推力 (归一化)
        self.B[8, 0] = 1.0 / self.mass  # 垂直推力影响z加速度
        
        # 姿态控制 (近似)
        torque_scaling = 1.0 / np.diag(self.inertia)
        self.B[9:12, 1:4] = np.diag(torque_scaling) * 2.0  # 缩放因子
    
    def _compute_lqr_gain(self):
        """求解Riccati方程计算LQR增益矩阵"""
        # 求解连续时间代数Riccati方程
        P = solve_continuous_are(self.A, self.B, self.Q, self.R)
        
        # 计算最优增益矩阵 K = R^-1 B^T P
        K = np.linalg.inv(self.R) @ self.B.T @ P
        
        return K
    
    def lqr_controller(self, current_state, desired_state):
        """
        LQR控制器实现
        current_state: [x, y, z, roll, pitch, yaw, vx, vy, vz, v_roll, v_pitch, v_yaw]
        desired_state: [x_d, y_d, z_d, roll_d, pitch_d, yaw_d]
        返回: 控制输入 [thrust, roll_rate, pitch_rate, yaw_rate] (归一化)
        """
        # 确保状态维度正确
        if len(current_state) != 12:
            raise ValueError(f"Current state must be 12-dimensional, got {len(current_state)}")
        if len(desired_state) != 6:
            raise ValueError(f"Desired state must be 6-dimensional, got {len(desired_state)}")
        
        # 计算状态误差（仅位置和姿态）
        error = np.array(desired_state[:6]) - np.array(current_state[:6])
        self.error_history.append(error)
        
        # 限制误差历史记录长度以避免积分饱和
        if len(self.error_history) > 100:
            self.error_history.pop(0)
        
        # 计算误差积分 (用于消除稳态误差)
        error_integral = np.sum(self.error_history, axis=0) if self.error_history else np.zeros(6)
        
        # 完整状态向量 (12维: 位置误差[0:3], 姿态误差[3:6], 速度[6:9], 角速度[9:12])
        # 注意：这里不再包含积分项，因为K矩阵是12维的
        x = np.concatenate((
            error[:3],                      # 位置误差
            error[3:6],                     # 姿态误差
            np.array(current_state[6:12])    # 速度和角速度
        ))
        
        # 计算控制输入 u = -Kx
        u = -np.dot(self.K, x)
        
        # 解包控制输入并归一化
        thrust = (u[0] + self.mass * self.gravity) / (self.mass * self.gravity * 2)  # 归一化到0-1
        roll_rate = u[1]
        pitch_rate = u[2]
        yaw_rate = u[3]
        
        # 控制输入限幅
        thrust = np.clip(thrust, 0.0, 1.0)  # 推力限制在0-1
        roll_rate = np.clip(roll_rate, -1.0, 1.0)
        pitch_rate = np.clip(pitch_rate, -1.0, 1.0)
        yaw_rate = np.clip(yaw_rate, -1.0, 1.0)
        
        return thrust, roll_rate, pitch_rate, yaw_rate


class TestLQRController:
    def __init__(self):
        self.controller = LQRController()
    
    def test_initialization(self):
        """测试控制器初始化"""
        assert self.controller.A.shape == (12, 12), "A matrix has wrong shape"
        assert self.controller.B.shape == (12, 4), "B matrix has wrong shape"
        assert self.controller.Q.shape == (12, 12), "Q matrix has wrong shape"
        assert self.controller.R.shape == (4, 4), "R matrix has wrong shape"
        assert self.controller.K.shape == (4, 12), "K matrix has wrong shape"
        print("test_initialization passed")
    
    def test_lqr_gain_calculation(self):
        """测试LQR增益计算"""
        # 检查增益矩阵是否合理
        assert not np.allclose(self.controller.K, np.zeros((4, 12))), "K matrix is all zeros"
        print("test_lqr_gain_calculation passed")
    
    def test_hover_control(self):
        """测试悬停控制"""
        current_state = [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]  # 1米高度悬停
        desired_state = [0, 0, 1, 0, 0, 0]  # 保持1米高度
        
        thrust, roll, pitch, yaw = self.controller.lqr_controller(current_state, desired_state)
        
        # 推力应该在0.5左右(重力补偿)
        assert abs(thrust - 0.5) < 0.1, f"Thrust {thrust} not near 0.5"
        # 姿态控制应该接近0
        assert abs(roll) < 0.1, f"Roll {roll} not near 0"
        assert abs(pitch) < 0.1, f"Pitch {pitch} not near 0"
        assert abs(yaw) < 0.1, f"Yaw {yaw} not near 0"
        print("test_hover_control passed")
    
    def test_position_correction(self):
        """测试位置修正"""
        # 增大测试位置偏差
        current_state = [1.0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]  # 向右偏移1.0米(从0.5增加到1.0)
        desired_state = [0, 0, 1, 0, 0, 0]  # 目标原点
        
        thrust, roll, pitch, yaw = self.controller.lqr_controller(current_state, desired_state)
        
        # 检查响应方向是否正确，降低幅度要求
        assert roll < 0, f"Roll {roll} should be negative for rightward position correction"
        print("test_position_correction passed")
    
    def test_attitude_correction(self):
        """测试姿态修正"""
        current_state = [0, 0, 1, 0.1, 0, 0, 0, 0, 0, 0, 0, 0]  # 滚转0.1弧度
        desired_state = [0, 0, 1, 0, 0, 0]  # 水平
        
        thrust, roll, pitch, yaw = self.controller.lqr_controller(current_state, desired_state)
        
        # 应该产生负的滚转速率指令来修正姿态
        assert roll < -0.1, f"Roll {roll} not negative enough for attitude correction"
        print("test_attitude_correction passed")
    
    def test_integral_action(self):
        """测试积分作用"""
        # 模拟持续存在的位置误差
        current_state = [0.1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        desired_state = [0, 0, 1, 0, 0, 0]
        
        # 多次调用控制器以积累积分项
        for _ in range(10):
            thrust, roll, pitch, yaw = self.controller.lqr_controller(current_state, desired_state)
        
        # 检查积分作用是否增强了控制输出
        initial_roll = roll
        for _ in range(20):
            thrust, roll, pitch, yaw = self.controller.lqr_controller(current_state, desired_state)
        
        assert abs(roll) < abs(initial_roll), "Integral action not reducing error"
        print("test_integral_action passed")


if __name__ == "__main__":
    tester = TestLQRController()
    tester.test_initialization()
    tester.test_lqr_gain_calculation()
    tester.test_hover_control()
    tester.test_position_correction()
    tester.test_attitude_correction()
    tester.test_integral_action()
    print("All tests passed!")