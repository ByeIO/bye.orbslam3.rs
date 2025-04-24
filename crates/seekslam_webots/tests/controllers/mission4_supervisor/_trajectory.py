# 记录从上电到结束的轨迹, 以便于返航时进行参考
# 如: 记录move_forward, move_left等指令以及参数, 返航时将参数逆向执行即可.

# 字典列表, 例如[{"key":"up","value":"100"},]
_MISSION_TRAJECTORY = [{}]

##############################################
# 硬编码的关键途径点(全局坐标)及旋转轴角
_KEY_PATH_POINTS = [ [-1.41, -2.03, 0], [-2.45, -4.31, 1.5], [-4.62, -3.95, 1.5], [-4.75, -2.6, 1.5], [-4.87, -1.39, 1.5]  ]
_KEY_PATH_ROTATIONS = [ [0, 0, 1, -2.0], [0, 0, 1, 2.97], [0, 0, -1, -1.67], [0, 0, -1, -1.67], [0, 0, -1, -1.67] ]
##############################################

def _track(act):
    '''记录动作指令到字典列表以记录轨迹
    
    参数:
        act: dict - 要记录的动作指令字典，例如 {"key":"up","value":"100"}
    '''
    if not isinstance(act, dict):
        raise ValueError("动作指令必须是字典类型")
    
    # 将动作指令添加到轨迹列表中
    _MISSION_TRAJECTORY.append(act)
    print("轨迹", act, "记录成功")

def _track_reverse(trajectory):
    '''将轨迹列表中的动作取反并反转整个列表顺序
    
    参数:
        trajectory: list - 要处理的轨迹列表
        
    返回:
        list - 处理后的反向轨迹列表
    '''
    if not trajectory:
        return []
    
    reversed_trajectory = []
    
    # 动作取反映射表
    reverse_map = {
        'up': 'down',
        'down': 'up',
        'left': 'right',
        'right': 'left'
    }
    
    # 先反转整个列表顺序
    reversed_order = trajectory[::-1]
    
    # 然后对每个动作进行取反操作
    for action in reversed_order:
        if not isinstance(action, dict) or 'key' not in action:
            continue
            
        reversed_action = action.copy()
        key = action['key']
        
        if key in reverse_map:
            # 处理方向键
            reversed_action['key'] = reverse_map[key]
        elif key == 'rotate':
            # 处理旋转，将value取反
            if 'value' in action:
                try:
                    reversed_action['value'] = str(-int(action['value']))
                except ValueError:
                    pass
        
        reversed_trajectory.append(reversed_action)
    
    return reversed_trajectory