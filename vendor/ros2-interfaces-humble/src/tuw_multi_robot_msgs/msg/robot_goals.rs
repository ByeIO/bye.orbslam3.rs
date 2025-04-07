use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotGoals {
    pub robot_name: ::std::string::String,
    pub destinations: Vec<crate::geometry_msgs::msg::Pose>,
}

impl Default for RobotGoals {
    fn default() -> Self {
        RobotGoals {
            robot_name: ::std::string::String::new(),
            destinations: Vec::new(),
        }
    }
}

impl ros2_client::Message for RobotGoals {}
