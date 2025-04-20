# 使用rospy控制PX4四旋翼飞控起飞到2.5米高度悬停5秒然后降落
# roslaunch px4 indoor1.launch

# -*- coding: utf-8 -*-
import rospy
import time
from geometry_msgs.msg import PoseStamped
from mavros_msgs.msg import State
from mavros_msgs.srv import CommandBool, CommandBoolRequest, SetMode, SetModeRequest

# 设置ROS_MASTER_URI环境变量（根据实际IP修改）
import os
os.environ['ROS_MASTER_URI'] = 'http://rock.home:11311'

class PX4Controller:
    def __init__(self):
        # 初始化ROS节点
        rospy.init_node("px4_controller_node")
        
        # 无人机当前状态
        self.current_state = State()
        
        # 目标位置消息
        self.target_pose = PoseStamped()
        
        # 订阅MAVROS状态信息
        self.state_sub = rospy.Subscriber("/mavros/state", State, self.state_cb, queue_size=10)
        
        # 发布本地目标位置（ENU坐标系）
        self.local_pos_pub = rospy.Publisher("/mavros/setpoint_position/local", PoseStamped, queue_size=10)
        
        # 服务客户端：解锁无人机
        self.arming_client = rospy.ServiceProxy("/mavros/cmd/arming", CommandBool)
        
        # 服务客户端：设置飞行模式
        self.set_mode_client = rospy.ServiceProxy("/mavros/set_mode", SetMode)
        
        # 设置发布频率（推荐20-30Hz）
        self.rate = rospy.Rate(30)

    def state_cb(self, msg):
        """状态回调函数，更新无人机状态"""
        self.current_state = msg

    def arm(self):
        """解锁无人机"""
        req = CommandBoolRequest()
        req.value = True
        while not self.arming_client.call(req).success:
            rospy.logwarn("尝试解锁...")
            self.rate.sleep()
        rospy.loginfo("无人机已解锁")

    def set_mode(self, mode):
        """设置飞行模式"""
        req = SetModeRequest()
        req.custom_mode = mode
        while not self.set_mode_client.call(req).mode_sent:
            rospy.logwarn(f"尝试切换至{mode}模式...")
            self.rate.sleep()
        rospy.loginfo(f"已切换至{mode}模式")

    def takeoff(self, height):
        """起飞到指定高度"""
        # 持续发布目标点（必须保持高频发送）
        for _ in range(100):
            self.target_pose.pose.position.z = height
            self.local_pos_pub.publish(self.target_pose)
            self.rate.sleep()
        
        # 切换至OFFBOARD模式
        self.set_mode("OFFBOARD")
        
        # 解锁无人机
        self.arm()
        
        # 持续控制无人机上升
        start_time = time.time()
        while not rospy.is_shutdown():
            # 检查是否到达目标高度（容差0.1m）
            if abs(self.target_pose.pose.position.z - height) < 0.1:
                break
                
            self.local_pos_pub.publish(self.target_pose)
            self.rate.sleep()
        
        rospy.loginfo(f"到达目标高度 {height}m")

    def land(self):
        """降落流程"""
        self.set_mode("LAND")
        rospy.loginfo("开始降落...")
        
        # 等待降落完成
        while self.current_state.armed and not rospy.is_shutdown():
            self.rate.sleep()
        
        rospy.loginfo("降落完成")

    def run(self):
        """主控制流程"""
        # 等待飞控连接
        while not self.current_state.connected:
            self.rate.sleep()
        
        # 起飞到2.5米高度
        self.takeoff(2.5)
        
        # 悬停5秒
        rospy.loginfo("开始悬停...")
        hover_start = time.time()
        while time.time() - hover_start < 5:
            self.local_pos_pub.publish(self.target_pose)
            self.rate.sleep()
        
        # 降落
        self.land()

if __name__ == "__main__":
    controller = PX4Controller()
    try:
        controller.run()
    except rospy.ROSInterruptException:
        pass