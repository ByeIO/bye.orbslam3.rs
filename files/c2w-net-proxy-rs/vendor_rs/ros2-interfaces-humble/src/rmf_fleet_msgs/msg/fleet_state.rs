use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetState {
    pub name: ::std::string::String,
    pub robots: Vec<crate::rmf_fleet_msgs::msg::RobotState>,
}

impl Default for FleetState {
    fn default() -> Self {
        FleetState {
            name: ::std::string::String::new(),
            robots: Vec::new(),
        }
    }
}

impl ros2_client::Message for FleetState {}
