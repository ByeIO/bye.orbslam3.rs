use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotState {
    pub joint_state: crate::sensor_msgs::msg::JointState,
    pub multi_dof_joint_state: crate::sensor_msgs::msg::MultiDOFJointState,
    pub attached_collision_objects: Vec<crate::moveit_msgs::msg::AttachedCollisionObject>,
    pub is_diff: bool,
}

impl Default for RobotState {
    fn default() -> Self {
        RobotState {
            joint_state: crate::sensor_msgs::msg::JointState::default(),
            multi_dof_joint_state: crate::sensor_msgs::msg::MultiDOFJointState::default(),
            attached_collision_objects: Vec::new(),
            is_diff: false,
        }
    }
}

impl ros2_client::Message for RobotState {}
