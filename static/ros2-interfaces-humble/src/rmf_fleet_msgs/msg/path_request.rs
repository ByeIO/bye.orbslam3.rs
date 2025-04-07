use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathRequest {
    pub fleet_name: ::std::string::String,
    pub robot_name: ::std::string::String,
    pub path: Vec<crate::rmf_fleet_msgs::msg::Location>,
    pub task_id: ::std::string::String,
}

impl Default for PathRequest {
    fn default() -> Self {
        PathRequest {
            fleet_name: ::std::string::String::new(),
            robot_name: ::std::string::String::new(),
            path: Vec::new(),
            task_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PathRequest {}
