use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotState {
    pub name: ::std::string::String,
    pub model: ::std::string::String,
    pub task_id: ::std::string::String,
    pub seq: u64,
    pub mode: crate::rmf_fleet_msgs::msg::RobotMode,
    pub battery_percent: f32,
    pub location: crate::rmf_fleet_msgs::msg::Location,
    pub path: Vec<crate::rmf_fleet_msgs::msg::Location>,
}

impl Default for RobotState {
    fn default() -> Self {
        RobotState {
            name: ::std::string::String::new(),
            model: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
            seq: 0,
            mode: crate::rmf_fleet_msgs::msg::RobotMode::default(),
            battery_percent: 0.0,
            location: crate::rmf_fleet_msgs::msg::Location::default(),
            path: Vec::new(),
        }
    }
}

impl ros2_client::Message for RobotState {}
