use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModeRequest {
    pub fleet_name: ::std::string::String,
    pub robot_name: ::std::string::String,
    pub mode: crate::rmf_fleet_msgs::msg::RobotMode,
    pub task_id: ::std::string::String,
    pub parameters: Vec<crate::rmf_fleet_msgs::msg::ModeParameter>,
}

impl Default for ModeRequest {
    fn default() -> Self {
        ModeRequest {
            fleet_name: ::std::string::String::new(),
            robot_name: ::std::string::String::new(),
            mode: crate::rmf_fleet_msgs::msg::RobotMode::default(),
            task_id: ::std::string::String::new(),
            parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for ModeRequest {}
