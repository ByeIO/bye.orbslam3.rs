use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionIKRequest {
    pub group_name: ::std::string::String,
    pub robot_state: crate::moveit_msgs::msg::RobotState,
    pub constraints: crate::moveit_msgs::msg::Constraints,
    pub avoid_collisions: bool,
    pub ik_link_name: ::std::string::String,
    pub pose_stamped: crate::geometry_msgs::msg::PoseStamped,
    pub ik_link_names: Vec<::std::string::String>,
    pub pose_stamped_vector: Vec<crate::geometry_msgs::msg::PoseStamped>,
    pub timeout: crate::builtin_interfaces::msg::Duration,
}

impl Default for PositionIKRequest {
    fn default() -> Self {
        PositionIKRequest {
            group_name: ::std::string::String::new(),
            robot_state: crate::moveit_msgs::msg::RobotState::default(),
            constraints: crate::moveit_msgs::msg::Constraints::default(),
            avoid_collisions: false,
            ik_link_name: ::std::string::String::new(),
            pose_stamped: crate::geometry_msgs::msg::PoseStamped::default(),
            ik_link_names: Vec::new(),
            pose_stamped_vector: Vec::new(),
            timeout: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for PositionIKRequest {}
