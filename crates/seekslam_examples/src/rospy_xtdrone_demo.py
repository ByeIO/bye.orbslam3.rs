#-*- coding: UTF-8 -*-
# 导入必要的ROS和Python库
import rospy  # ROS Python客户端库
import tty, termios  # 终端控制相关库
import sys,select  # 系统相关库
from mavros_msgs.msg import AttitudeTarget,PositionTarget,State  # MAVROS消息类型
from mavros_msgs.srv import CommandBool, SetMode  # MAVROS服务类型
from geometry_msgs.msg import PoseStamped, Twist, TwistStamped  # 几何消息类型
from nav_msgs.msg import Odometry  # 里程计消息类型
import time  # 时间库
import PyKDL  # KDL库，用于位姿转换
import math  # 数学库
import numpy as np  # 数值计算库
import sys  # 系统库

# 定义控制发布类
class pub:
    # 初始化函数
    def __init__(self, uav_type, control_type):
        # 初始化位置变量
        self.px = 0  # x轴位置
        self.py = 0  # y轴位置
        self.pz = 0  # z轴位置
        # 初始化速度变量
        self.vx,self.vy,self.vz = 0,0,0  # x,y,z轴速度
        # 初始化欧拉角
        self.roll,self.pitch,self.yaw=0,0,0  # 滚转、俯仰、偏航角
        # 初始化角速度
        self.roll_rate,self.pitch_rate,self.yaw_rate=0,0,0  # 滚转、俯仰、偏航角速度
        # 初始化无人机状态
        self.arm_state = False  # 解锁状态
        self.mavros_state = State()  # MAVROS状态
        self.control_type = control_type  # 控制类型
 
        # 控制指令说明信息
        self.msg2leader =  '''
        4/1: 增加/减少 x轴目标位置
        5/2: 增加/减少 y轴目标位置
        6/3: 增加/减少 z轴目标位置
        7/8: 增加/减少 偏航角目标
        a: 解锁无人机
        d: 锁定无人机
        r: 返航  
        l: 降落
        b: 开始姿态控制
        p: 打印控制信息
        q: 退出程序
         '''
        # 订阅者初始化
        # 订阅本地位置信息
        self.local_pose_sub = rospy.Subscriber(uav_type + "_0/mavros/local_position/pose", PoseStamped, self.local_pose_callback)
        # 订阅本地速度信息
        self.local_vel_sub = rospy.Subscriber(uav_type + "_0/mavros/local_position/velocity_body",TwistStamped, self.local_vel_callback)
        # 订阅里程计信息
        self.odom_sub=rospy.Subscriber(uav_type + "_0/mavros/local_position/odom",Odometry,self.odom_callback)
        # 订阅MAVROS状态信息
        self.mavros_sub = rospy.Subscriber(uav_type + "_0/mavros/state", State, self.mavros_state_callback)

        # 发布者初始化
        # 发布目标运动信息
        self.target_motion_pub = rospy.Publisher(uav_type + "_0/mavros/setpoint_raw/local", PositionTarget, queue_size=10)
        # 发布姿态目标信息
        self.body_target_pub = rospy.Publisher(uav_type + "_0/mavros/setpoint_raw/attitude", AttitudeTarget, queue_size=2)
        # 设置飞行模式服务
        self.flightModeService = rospy.ServiceProxy(uav_type + "_0/mavros/set_mode", SetMode)
        # 解锁服务
        self.armService = rospy.ServiceProxy(uav_type + "_0/mavros/cmd/arming", CommandBool)

    # 本地位置回调函数
    def local_pose_callback(self, msg):
        self.px = msg.pose.position.x  # 更新x轴位置
        self.py = -msg.pose.position.y  # 更新y轴位置(取反)
        self.pz = msg.pose.position.z  # 更新z轴位置

    # 本地速度回调函数
    def local_vel_callback(self, msg):
        self.vx=msg.twist.linear.x  # 更新x轴速度
        self.vy=-msg.twist.linear.y  # 更新y轴速度(取反)
        self.vz=msg.twist.linear.z  # 更新z轴速度

    # 里程计回调函数
    def odom_callback(self,msg):
        # 获取四元数
        x=msg.pose.pose.orientation.x
        y=msg.pose.pose.orientation.y
        z=msg.pose.pose.orientation.z
        w=msg.pose.pose.orientation.w
        # 将四元数转换为欧拉角
        rot = PyKDL.Rotation.Quaternion(x,y,z,w)
        self.roll =rot.GetRPY()[0]  # 更新滚转角
        self.pitch =rot.GetRPY()[1]  # 更新俯仰角
        self.yaw =rot.GetRPY()[2]  # 更新偏航角

        # 更新角速度
        self.roll_rate=msg.twist.twist.angular.x
        self.pitch_rate=msg.twist.twist.angular.y
        self.yaw_rate=msg.twist.twist.angular.z
        
    # MAVROS状态回调函数
    def mavros_state_callback(self, msg):
        self.mavros_state = msg.mode  # 更新飞行模式
        self.arm_state = msg.armed  # 更新解锁状态

    # 解锁无人机函数
    def arm(self):
        if self.armService(True):
            print('无人机已解锁')
            return True
        else:
            print("无人机解锁失败!")
            return False

    # 锁定无人机函数
    def disarm(self):
        if self.armService(False):
            print('无人机已锁定')
            return True
        else:
            print("无人机锁定失败!")
            return False

    # 欧拉角转四元数函数
    def euler_to_quaternion(self,roll, pitch, yaw):
        ox=math.sin(pitch/2)*math.sin(yaw/2)*math.cos(roll/2)+math.cos(pitch/2)*math.cos(yaw/2)*math.sin(roll/2)
        oy=math.sin(pitch/2)*math.cos(yaw/2)*math.cos(roll/2)+math.cos(pitch/2)*math.sin(yaw/2)*math.sin(roll/2)
        oz=math.cos(pitch/2)*math.sin(yaw/2)*math.cos(roll/2)-math.sin(pitch/2)*math.cos(yaw/2)*math.sin(roll/2)
        ow=math.cos(pitch/2)*math.cos(yaw/2)*math.cos(roll/2)-math.sin(pitch/2)*math.sin(yaw/2)*math.sin(roll/2)
        return np.array([ow, ox, oy, oz])

    # 获取键盘输入函数
    def getKey(self):
        settings = termios.tcgetattr(sys.stdin)  # 获取终端设置
        tty.setraw(sys.stdin.fileno())  # 设置终端为原始模式
        rlist, _, _ = select.select([sys.stdin], [], [], 0.1)  # 监听键盘输入
        if rlist:
            key = sys.stdin.read(1)  # 读取按键
        else:
            key = ''
        termios.tcsetattr(sys.stdin, termios.TCSADRAIN, settings)  # 恢复终端设置
        return key

    # 主控制函数
    def start(self):
        self.time_start=time.time()  # 记录开始时间
        rospy.init_node("attitude_contorl_node")  # 初始化ROS节点
        print('开始控制!')
        print(self.msg2leader)  # 打印控制指令说明
        offborad_mission=None  # 初始化任务模式
        self.att =AttitudeTarget()  # 初始化姿态目标消息
        target_raw_pose = PositionTarget()  # 初始化位置目标消息
        # 初始化起飞目标位置和偏航角
        takeoff_x,takeoff_y,takeoff_z,takeoff_yaw=0,0,2,0
        # 初始化控制目标位置和偏航角
        control_x,control_y,control_z,yaw_r=1,0,2,0
        angle_max=0.3  # 最大角度限制

        # 主循环
        while not rospy.is_shutdown():          
            self.time_now=time.time()  # 当前时间
            self.clock=self.time_now-self.time_start  # 运行时间
            key = self.getKey()  # 获取键盘输入
            # 处理键盘输入
            if  key == '4' :  # 增加x轴目标位置
                control_x =control_x +0.2
                print(self.msg2leader)
                print("x轴目标:%.2f   y轴目标: %.2f    z轴目标: %.2f 偏航角: %.2f " % (control_x,control_y,control_z,yaw_r))
            elif key == '1' :  # 减少x轴目标位置
                control_x =control_x -0.2
                print(self.msg2leader)
                print('x轴目标=',control_x)
                print("x轴目标:%.2f   y轴目标: %.2f    z轴目标: %.2f 偏航角: %.2f " % (control_x,control_y,control_z,yaw_r))
            elif key == '5' :  # 增加y轴目标位置
                control_y =control_y +0.2
                print(self.msg2leader)
                print("x轴目标:%.2f   y轴目标: %.2f    z轴目标: %.2f 偏航角: %.2f " % (control_x,control_y,control_z,yaw_r))
            elif key == '2' :  # 减少y轴目标位置
                control_y =control_y -0.2
                print(self.msg2leader)
                print("x轴目标:%.2f   y轴目标: %.2f    z轴目标: %.2f 偏航角: %.2f " % (control_x,control_y,control_z,yaw_r))
            elif key == '6' :  # 增加z轴目标位置
                control_z =control_z +0.2
                print(self.msg2leader)
                print("x轴目标:%.2f   y轴目标: %.2f    z轴目标: %.2f 偏航角: %.2f " % (control_x,control_y,control_z,yaw_r))
            elif key == '3' :  # 减少z轴目标位置
                control_z =control_z -0.2
                print(self.msg2leader)
                print("x轴目标:%.2f   y轴目标: %.2f    z轴目标: %.2f 偏航角: %.2f " % (control_x,control_y,control_z,yaw_r))
            elif key == '7' :  # 增加偏航角
                yaw_r =yaw_r +0.05
                print(self.msg2leader)
                print("x轴目标:%.2f   y轴目标: %.2f    z轴目标: %.2f 偏航角: %.2f " % (control_x,control_y,control_z,yaw_r))
            elif key == '8' :  # 减少偏航角
                yaw_r =yaw_r -0.05
                print(self.msg2leader)
                print("x轴目标:%.2f   y轴目标: %.2f    z轴目标: %.2f 偏航角: %.2f " % (control_x,control_y,control_z,yaw_r))
            elif key == 'a' :  # 解锁无人机                
                self.arm()
            elif key == 'd' :  # 锁定无人机
                self.disarm()
            elif key == 'r' :  # 返航
                offborad_mission='return'
                print('返航至起飞点',target_raw_pose.position)
            elif key == 'h':  # 悬停模式
                self.flightModeService(custom_mode='HOVER')                
                print('悬停模式 ')
            elif key == 'b':  # 开始姿态控制
                offborad_mission='contorl'               
                print('开始姿态控制')
                print("x轴目标:%.2f   y轴目标: %.2f    z轴目标: %.2f 偏航角: %.2f " % (control_x,control_y,control_z,yaw_r))
            elif key == 't' :  # 起飞
                self.arm()         
                offborad_mission='takeoff'
                print(self.msg2leader) 
                print('当前任务模式=',offborad_mission)               
                print("起飞点x:%.2f   起飞点y: %.2f    起飞高度: %.2f " % (takeoff_x,takeoff_y,takeoff_z))
            elif key == 'l':  # 降落
                self.flightModeService(custom_mode='AUTO.LAND') 
                offborad_mission='land'                
                print(self.msg2leader)
                print('自动降落模式')    
            elif key == 'p' :  # 打印控制信息
                print(self.msg2leader) 
                print("x轴目标:%.2f   y轴目标: %.2f    z轴目标: %.2f 偏航角: %.2f " % (control_x,control_y,control_z,yaw_r))
                print("起飞点x:%.2f   起飞点y: %.2f    起飞高度: %.2f " % (takeoff_x,takeoff_y,takeoff_z))
            elif key == 'q' :  # 退出程序
                break
                 
            # 返航或起飞任务处理
            if offborad_mission in['return','takeoff']:
                self.flightModeService(custom_mode='OFFBOARD')  # 设置OFFBOARD模式
                # 设置目标位置
                target_raw_pose.position.x = takeoff_x
                target_raw_pose.position.y = takeoff_y
                target_raw_pose.position.z = takeoff_z
                target_raw_pose.yaw = takeoff_yaw
                # 设置位置控制掩码
                target_raw_pose.type_mask = PositionTarget.IGNORE_VX + PositionTarget.IGNORE_VY + PositionTarget.IGNORE_VZ \
                                    + PositionTarget.IGNORE_AFX + PositionTarget.IGNORE_AFY + PositionTarget.IGNORE_AFZ \
                                    + PositionTarget.FORCE + PositionTarget.IGNORE_YAW_RATE
                self.target_motion_pub.publish(target_raw_pose)  # 发布目标位置
            
            # 姿态控制处理
            if offborad_mission=='contorl': 
                self.flightModeService(custom_mode='OFFBOARD')  # 设置OFFBOARD模式
                if self.control_type=="angular_speed":  # 角速度控制模式
                    # 级联PID控制
                    # 计算滚转角目标
                    roll_r = -0.54236707*(self.py-control_y)-0.42717549*(self.vy-0)
                    # 计算俯仰角目标
                    pitch_r= -0.54236707*(self.px-control_x)-0.42717549*(self.vx-0)

                    # 角度限幅
                    if pitch_r>angle_max:
                        pitch_r=angle_max
                    if pitch_r<-angle_max:
                        pitch_r=-angle_max
                    if roll_r>angle_max:
                        roll_r=angle_max
                    if roll_r<-angle_max:
                        roll_r=-angle_max
                    
                    # 计算角速度目标
                    roll_rate=-5*(self.roll-roll_r)-0.1*self.roll_rate
                    pitch_rate=-5*(self.pitch-pitch_r)-0.1*self.pitch_rate
                    yaw_rate=-1*(self.yaw-yaw_r)-0.1*self.yaw_rate
                    
                    # 设置姿态目标消息
                    self.att.body_rate.x=roll_rate
                    self.att.body_rate.y=pitch_rate             
                    self.att.body_rate.z=yaw_rate
                    # 计算推力(高度控制)
                    self.att.thrust = -0.5*(self.pz-control_z)-0.31774509*(self.vz-0)+0.55/math.cos(self.pitch)/math.cos(self.roll)#悬停油门设为0.55
                    self.att.type_mask=AttitudeTarget.IGNORE_ATTITUDE  # 设置忽略姿态标志
                    self.body_target_pub.publish(self.att)  # 发布姿态目标

                elif self.control_type=="angle":  # 角度控制模式
                    ##PD控制
                    # 计算滚转角目标
                    roll_r = -0.54236707*(self.py-control_y)-0.42717549*(self.vy-0)
                    # 计算俯仰角目标
                    pitch_r= -0.54236707*(self.px-control_x)-0.42717549*(self.vx-0)
                    yaw_r=yaw_r  # 偏航角目标
                    # 计算推力(高度控制)
                    thrust = -0.5*(self.pz-control_z)-0.31774509*(self.vz-0)+0.55/math.cos(self.pitch)/math.cos(self.roll)#悬停油门设为0.55

                    # 角度限幅
                    if pitch_r>angle_max:
                        pitch_r=angle_max
                    if pitch_r<-angle_max:
                        pitch_r=-angle_max
                    if roll_r>angle_max:
                        roll_r=angle_max
                    if roll_r<-angle_max:
                        roll_r=-angle_max

                    # 欧拉角转四元数
                    q=self.euler_to_quaternion(roll_r,pitch_r,yaw_r)
                    # 设置姿态目标消息
                    self.att.orientation.w=q[0]
                    self.att.orientation.x=q[1]
                    self.att.orientation.y=q[2]
                    self.att.orientation.z=q[3]
                    self.att.thrust=thrust  # 设置推力
                    self.att.type_mask = AttitudeTarget.IGNORE_ROLL_RATE + AttitudeTarget.IGNORE_PITCH_RATE+AttitudeTarget.IGNORE_YAW_RATE  # 设置忽略角速度标志
                    self.body_target_pub.publish(self.att)  # 发布姿态目标

# 主程序入口
if __name__ == '__main__':
    con = pub(sys.argv[1], sys.argv[2])  # 创建控制对象
    con.start()  # 启动控制