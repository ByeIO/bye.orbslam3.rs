use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinationRequest {
    pub fleet_name: ::std::string::String,
    pub robot_name: ::std::string::String,
    pub destination: crate::rmf_fleet_msgs::msg::Location,
    pub task_id: ::std::string::String,
}

impl Default for DestinationRequest {
    fn default() -> Self {
        DestinationRequest {
            fleet_name: ::std::string::String::new(),
            robot_name: ::std::string::String::new(),
            destination: crate::rmf_fleet_msgs::msg::Location::default(),
            task_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DestinationRequest {}
