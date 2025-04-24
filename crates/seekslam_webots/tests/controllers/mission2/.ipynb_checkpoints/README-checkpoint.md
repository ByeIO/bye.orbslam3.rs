# seekslam webots仿真

## 使用说明
1. 使用webots启动seekslam_apartment.wbt
2. 运行仿真(注意时间轴需要在0:0处开始仿真, 否则无人机不会启动)

## 开发说明
### 代码结构
```sh
- mission1.py : 任务总入口, webots使用这个作为控制器入口
- _drone.py : 无人机实例, 提供传感器、执行器
- _state.py : 当前状态的枚举
- _controller.py : 封装的顶层傻瓜式移动API, 可以例如`controller.take_off(1.5)`起飞.
- _pid.py : pid控制器
```
