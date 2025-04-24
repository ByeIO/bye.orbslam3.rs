# Linear quadratic estimator design (Kalman filter) for discrete-time systems.
# 离散时间系统的线性二次估计器设计（卡尔曼滤波器）

# 用于Dji Mavic2Pro 四旋翼无人机的dlqe控制器

#############################################
# 内置库
import os
# 第三方库
import numpy as np
import matplotlib.pyplot as plt 
# 闭环控制库
import control as ct
# MATLAB兼容函数
import control.matlab as ctl
# 单元测试
import pytest
#############################################

class DLQEController():
    """
    DJI Mavic2Pro 四旋翼无人机的离散线性二次估计器(DLQE)控制器
    
    参数:
        A (np.ndarray): 状态转移矩阵
        B (np.ndarray): 控制输入矩阵
        C (np.ndarray): 观测矩阵
        Q (np.ndarray): 过程噪声协方差矩阵
        R (np.ndarray): 测量噪声协方差矩阵
        G (np.ndarray): 过程噪声驱动矩阵(可选)
        dt (float): 采样时间(秒)
    """
    def __init__(self, A, B, C, Q, R, G=None, dt=0.01):
        # 检查矩阵维度一致性
        n_states = A.shape[0]  # 状态维度
        n_outputs = C.shape[0]  # 输出维度
        
        if G is None:
            G = np.eye(n_states)  # 默认过程噪声直接作用于所有状态
        
        # 验证矩阵维度
        assert A.shape == (n_states, n_states), "A矩阵维度不正确"
        assert B.shape[0] == n_states, "B矩阵维度不正确"
        assert C.shape[1] == n_states, "C矩阵维度不正确"
        assert Q.shape == (n_states, n_states), "Q矩阵维度不正确"
        assert R.shape == (n_outputs, n_outputs), "R矩阵维度不正确"
        assert G.shape[0] == n_states, "G矩阵维度不正确"
        
        self.A = A  # 状态转移矩阵
        self.B = B  # 控制输入矩阵
        self.C = C  # 观测矩阵
        self.Q = Q  # 过程噪声协方差
        self.R = R  # 测量噪声协方差
        self.G = G  # 过程噪声驱动矩阵
        self.dt = dt  # 采样时间
        
        # 卡尔曼增益和误差协方差矩阵
        self.L = None  
        self.P = None  
        
        # 当前状态估计
        self.x_hat = np.zeros((n_states, 1))  
        
        # 初始化卡尔曼滤波器
        self._init_kalman_filter()


        
    
    def _init_kalman_filter(self):
        """初始化卡尔曼滤波器，计算稳态卡尔曼增益和误差协方差"""
        try:
            # 使用scipy.linalg.solve_discrete_are直接求解DARE
            from scipy.linalg import solve_discrete_are
            P = solve_discrete_are(self.A.T, self.C.T, self.G @ self.Q @ self.G.T, self.R)
            L = self.P @ self.C.T @ np.linalg.inv(self.C @ self.P @ self.C.T + self.R)
            self.L = L
            self.P = P
        except Exception as e:
            raise RuntimeError(f"卡尔曼滤波器初始化失败: {str(e)}")


            
            
    def update(self, y, u):
        """
        更新状态估计(测量更新和时间更新)
        
        参数:
            y (np.ndarray): 当前测量值
            u (np.ndarray): 当前控制输入
            
        返回:
            np.ndarray: 更新后的状态估计
        """
        # 确保输入是列向量
        y = np.asarray(y).reshape(-1, 1)
        u = np.asarray(u).reshape(-1, 1)
        
        # 测量更新(校正)
        y_hat = self.C @ self.x_hat  # 预测测量值
        residual = y - y_hat  # 测量残差
        residual = residual.reshape(-1, 1)  # 确保 residual 是列向量
        self.x_hat = self.x_hat + self.L @ residual.reshape(-1, 1)  # 校正状态估计
        
        # 时间更新(预测)
        self.x_hat = self.A @ self.x_hat + self.B @ u
        
        return self.x_hat.copy()

        
    
    def reset(self, x0=None):
        """重置状态估计"""
        if x0 is None:
            self.x_hat = np.zeros((self.A.shape[0], 1))
        else:
            self.x_hat = np.asarray(x0).reshape(-1, 1)
    
    def get_state_estimate(self):
        """获取当前状态估计"""
        return self.x_hat.copy()
    
    def get_kalman_gain(self):
        """获取卡尔曼增益矩阵"""
        return self.L.copy()
    
    def get_error_covariance(self):
        """获取误差协方差矩阵"""
        return self.P.copy()


# 单元测试
class TestDLQEController:
    """DLQE控制器的单元测试"""
    
    @pytest.fixture
    def setup_system(self):
        """设置测试用的简单二阶系统"""
        dt = 0.01  # 采样时间
        
        # 系统矩阵 (简单的质量-弹簧-阻尼系统模型)
        A = np.array([[1, dt],
                      [-0.1*dt, 1-0.2*dt]])
        B = np.array([[0],
                      [dt]])
        C = np.array([[1, 0]])
        
        # 噪声协方差矩阵
        Q = np.diag([0.01, 0.01])  # 过程噪声
        R = np.array([[0.1]])      # 测量噪声
        
        # 创建控制器实例
        controller = DLQEController(A, B, C, Q, R, dt=dt)
        
        return controller
    
    def test_initialization(self, setup_system):
        """测试初始化是否正确"""
        controller = setup_system
        
        # 检查卡尔曼增益和协方差矩阵是否计算
        assert controller.L is not None
        assert controller.P is not None
        
        # 检查矩阵维度
        assert controller.L.shape == (2, 1)  # 卡尔曼增益维度
        assert controller.P.shape == (2, 2)  # 误差协方差维度
        
    def test_state_update(self, setup_system):
        """测试状态更新功能"""
        controller = setup_system
        
        # 初始状态
        x0 = np.array([[1.0], [0.5]])
        controller.reset(x0)
        
        # 模拟测量和控制输入
        y = np.array([[1.05]])  # 带有噪声的测量
        u = np.array([[0.1]])   # 控制输入
        
        # 更新状态估计
        x_hat = controller.update(y, u)
        
        # 检查输出维度
        assert x_hat.shape == (2, 1)
        
        # 检查状态估计是否变化(不应该等于初始状态)
        assert not np.allclose(x_hat, x0)
        
    def test_reset(self, setup_system):
        """测试重置功能"""
        controller = setup_system
        
        # 初始状态
        x0 = np.array([[1.0], [0.5]])
        controller.reset(x0)
        
        # 检查状态是否重置正确
        assert np.allclose(controller.get_state_estimate(), x0)
        
        # 带默认值的重置
        controller.reset()
        assert np.allclose(controller.get_state_estimate(), np.zeros((2, 1)))
    
    def test_invalid_inputs(self):
        """测试无效输入处理"""
        # 创建无效系统矩阵
        A = np.array([[1, 0.01],
                      [-0.001, 0.99]])
        B = np.array([[0], [0.01]])
        C = np.array([[1, 0]])
        Q = np.diag([0.01])  # 错误维度
        R = np.array([[0.1]])
        
        # 检查是否会引发断言错误
        with pytest.raises(AssertionError):
            DLQEController(A, B, C, Q, R)
    
    def test_kalman_filter_stability(self, setup_system):
        """测试卡尔曼滤波器稳定性"""
        controller = setup_system
        
        # 获取误差协方差矩阵
        P = controller.get_error_covariance()
        
        # 检查P是否是正定矩阵(所有特征值>0)
        eigenvalues = np.linalg.eigvals(P)
        assert np.all(eigenvalues > 0)
        
        # 检查卡尔曼增益是否合理
        L = controller.get_kalman_gain()
        assert np.all(np.abs(L) < 1)  # 对于稳定系统，增益通常小于1


if __name__ == "__main__":
    # 示例使用
    
    # 无人机系统参数 (简化模型)
    dt = 0.02  # 采样时间20ms
    
    # 状态矩阵 (x=[位置, 速度, 角度, 角速度]')
    A = np.array([
        [1, dt, 0, 0],
        [0, 1, -9.81*dt, 0],  # 重力影响
        [0, 0, 1, dt],
        [0, 0, 0, 1]
    ])
    
    # 控制矩阵 (u=[推力, 力矩]')
    B = np.array([
        [0, 0],
        [0, 0],
        [0, 0],
        [1, 1]  # 假设控制输入直接影响角加速度
    ])
    
    # 观测矩阵 (只能测量位置和角度)
    C = np.array([
        [1, 0, 0, 0],
        [0, 0, 1, 0]
    ])
    
    # 噪声协方差
    Q = np.diag([0.01, 0.01, 0.01, 0.01])  # 过程噪声
    R = np.diag([0.1, 0.1])                # 测量噪声
    
    # 创建DLQE控制器
    controller = DLQEController(A, B, C, Q, R, dt=dt)
    
    # 模拟运行
    n_steps = 100
    true_states = np.zeros((4, n_steps))
    measurements = np.zeros((2, n_steps))
    estimates = np.zeros((4, n_steps))
    
    # 初始状态
    true_state = np.array([[0], [0], [0.1], [0]])  # 初始有0.1弧度倾斜
    
    # 模拟控制输入 (正弦信号)
    for t in range(n_steps):
        # 模拟控制输入
        u = np.array([[0.5], [0.5 * np.sin(0.1 * t)]])
        
        # 模拟真实状态更新
        true_state = A @ true_state + B @ u + np.random.multivariate_normal(
            np.zeros(4), Q).reshape(-1, 1)
        
        # 模拟测量 (带有噪声)
        y = C @ true_state + np.random.multivariate_normal(
            np.zeros(2), R).reshape(-1, 1)
        
        # 更新状态估计
        x_hat = controller.update(y, u)
        
        # 存储结果
        true_states[:, t] = true_state.flatten()
        measurements[:, t] = y.flatten()
        estimates[:, t] = x_hat.flatten()
    
    # 绘制结果
    plt.figure(figsize=(12, 8))
    
    # 位置估计
    plt.subplot(2, 1, 1)
    plt.plot(true_states[0, :], 'r-', label='真实位置')
    plt.plot(measurements[0, :], 'b.', label='测量位置')
    plt.plot(estimates[0, :], 'g--', label='估计位置')
    plt.title('位置估计')
    plt.legend()
    
    # 角度估计
    plt.subplot(2, 1, 2)
    plt.plot(true_states[2, :], 'r-', label='真实角度')
    plt.plot(measurements[1, :], 'b.', label='测量角度')
    plt.plot(estimates[2, :], 'g--', label='估计角度')
    plt.title('角度估计')
    plt.legend()
    
    plt.tight_layout()
    plt.show()