"""
 * File: offb_node.py
 * Stack and tested in Gazebo Classic 9 SITL
"""

#! /usr/bin/env python

import rospy
from geometry_msgs.msg import PoseStamped
from mavros_msgs.msg import State
from mavros_msgs.srv import CommandBool, CommandBoolRequest, SetMode, SetModeRequest

current_state = State()

def state_cb(msg):
    global current_state
    current_state = msg


if __name__ == "__main__":
    print("hello from rospy_px4_demo")

    rospy.init_node("offb_node_py")

    state_sub = rospy.Subscriber("/iris_0/mavros/state", State, callback = state_cb)

    local_pos_pub = rospy.Publisher("/iris_0/mavros/setpoint_position/local", PoseStamped, queue_size=10)

    rospy.wait_for_service("/iris_0/mavros/cmd/arming")
    arming_client = rospy.ServiceProxy("/iris_0/mavros/cmd/arming", CommandBool)

    rospy.wait_for_service("/iris_0/mavros/set_mode")
    set_mode_client = rospy.ServiceProxy("/iris_0/mavros/set_mode", SetMode)


    # Setpoint publishing MUST be faster than 2Hz
    rate = rospy.Rate(20)

    # Wait for Flight Controller connection
    while(not rospy.is_shutdown() and not current_state.connected):
        print("rospy is shutdown")
        rate.sleep()

    pose = PoseStamped()

    # Initial position (takeoff position)
    pose.pose.position.x = 0
    pose.pose.position.y = 0
    pose.pose.position.z = 0  # Start on ground

    # Send a few setpoints before starting
    for i in range(100):
        if(rospy.is_shutdown()):
            break

        local_pos_pub.publish(pose)
        rate.sleep()

    offb_set_mode = SetModeRequest()
    offb_set_mode.custom_mode = 'OFFBOARD'

    arm_cmd = CommandBoolRequest()
    arm_cmd.value = True

    last_req = rospy.Time.now()

    # Flight state machine variables
    flight_phase = "TAKEOFF"  # TAKEOFF, HOVER, LAND, COMPLETE
    hover_start_time = None
    target_altitude = 2.0  # Target altitude in meters
    land_speed = 0.1  # Landing speed (m/s)

    while(not rospy.is_shutdown()):
        # First ensure we're in OFFBOARD mode and armed
        if(current_state.mode != "OFFBOARD" and (rospy.Time.now() - last_req) > rospy.Duration(5.0)):
            if(set_mode_client.call(offb_set_mode).mode_sent == True):
                print("OFFBOARD enabled")
                # rospy.loginfo("OFFBOARD enabled")
            last_req = rospy.Time.now()
        else:
            if(not current_state.armed and (rospy.Time.now() - last_req) > rospy.Duration(5.0)):
                if(arming_client.call(arm_cmd).success == True):
                    print("Vehicle armed")
                    # rospy.loginfo("Vehicle armed")
                last_req = rospy.Time.now()

        # Flight state machine
        if flight_phase == "TAKEOFF":
            # Command takeoff altitude
            pose.pose.position.z = target_altitude
            
            # Check if we've reached target altitude (with some tolerance)
            if pose.pose.position.z - pose.pose.position.z < 0.1:  # Simple check
                flight_phase = "HOVER"
                hover_start_time = rospy.Time.now()
                print("Reached target altitude, starting hover")
                rospy.loginfo("Reached target altitude, starting hover")

        elif flight_phase == "HOVER":
            # Maintain current position
            pose.pose.position.z = target_altitude
            
            # Check if hover duration has elapsed (5 seconds)
            if (rospy.Time.now() - hover_start_time) > rospy.Duration(5.0):
                flight_phase = "LAND"
                rospy.loginfo("Hover complete, beginning landing")

        elif flight_phase == "LAND":
            # Gradually decrease altitude
            if pose.pose.position.z > 0.1:  # Until very close to ground
                pose.pose.position.z -= land_speed * (1.0/20.0)  # Adjust based on rate
            else:
                pose.pose.position.z = 0
                flight_phase = "COMPLETE"
                rospy.loginfo("Landing complete")
                exit(0)

        elif flight_phase == "COMPLETE":
            # Disarm after landing
            if current_state.armed:
                arm_cmd.value = False
                if(arming_client.call(arm_cmd).success == True):
                    print("DONE")
                    rospy.loginfo("Vehicle disarmed")
            
            # Optionally switch back to STABILIZED mode
            if current_state.mode == "OFFBOARD":
                stablized_set_mode = SetModeRequest()
                stablized_set_mode.custom_mode = 'STABILIZED'
                if(set_mode_client.call(stablized_set_mode).mode_sent == True):
                    rospy.loginfo("STABILIZED mode enabled")
                    exit(0)

        # Publish the position setpoint
        local_pos_pub.publish(pose)
        rate.sleep()