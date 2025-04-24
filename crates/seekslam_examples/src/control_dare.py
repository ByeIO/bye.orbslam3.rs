# 求解离散代数 Riccati 方程 (DARE)

import numpy as np
from scipy import linalg as sp

# 系统矩阵 (简单的质量-弹簧-阻尼系统模型)
dt = 0.01

# 系统矩阵
A = np.array([[1, dt], [-0.1 * dt, 1 - 0.2 * dt]])
B = np.array([[0], [dt]])
C = np.array([[1, 0]])

# 噪声协方差矩阵
Q = np.diag([0.01, 0.01])  # 过程噪声
R = np.array([[0.1]])      # 测量噪声

# 求解离散代数 Riccati 方程 (DARE)
try:
    X = sp.solve_discrete_are(A, B, Q, R, balanced=False)
    print("DARE 的解:", X)
except np.linalg.LinAlgError as e:
    print("求解 DARE 失败:", e)