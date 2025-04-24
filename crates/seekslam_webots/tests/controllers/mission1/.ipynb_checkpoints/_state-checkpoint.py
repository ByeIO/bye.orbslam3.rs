# 存储各种状态

from enum import Enum
# 任务状态枚举
class _MissionState(Enum):
    # 刚上电传感器和算法模块还没初始化
    INIT = 1  
    # 传感器和算法模块都初始化完成，可以起飞
    READY = 2     
    # 起飞状态
    TAKEOFF = 3   
    # 悬停状态
    HOVER = 4     
    # 自旋#1状态
    ROTATE = 5    
    # 空中移动并搜索目标状态
    SEEK = 6      
    # 自旋#2状态
    ROTATE2 = 7   
    # 发现目标进入攻击模式
    ATTACK = 8   
    # 返航状态
    HOME = 9     
    # 降落状态
    LAND = 10     