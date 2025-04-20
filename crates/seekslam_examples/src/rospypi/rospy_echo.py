# 测试向http://home.qsbye.cn:11311主节点发布hello消息

import rospy
import std_msgs.msg
import time

# 设置ROS_MASTER_URI环境变量
import os
os.environ['ROS_MASTER_URI'] = 'http://rock.home:11311'

print("Hello from rospypi!")

def callback(msg):
    """订阅消息的回调函数"""
    print("收到消息:", msg)

def rosout_callback(msg):
    """订阅/rosout话题的回调函数"""
    print("[ROS日志]:", msg)

def main():
    # 初始化节点
    rospy.init_node("hoge")
    rospy.loginfo('节点已启动')
    
    # 订阅sub话题
    sub = rospy.Subscriber("/num_pub2", std_msgs.msg.String, callback)
    # 订阅/rosout话题
    # rosout_sub = rospy.Subscriber("/rosout", rospy.msg.Log, rosout_callback)
    
    # 发布pub话题
    pub = rospy.Publisher('/num_pub2', std_msgs.msg.Int16, queue_size=10)
    
    rate = rospy.Rate(1)  # 1Hz频率
    # while not rospy.is_shutdown():
    #     pub.publish(4)    # 发布数字3
    #     print("发布数字4")
    #     rate.sleep()

    while True:
        pub.publish(4)    # 发布数字3
        print("发布数字4")
        # rate.sleep()
        time.sleep(1)


if __name__ == "__main__":
    main()