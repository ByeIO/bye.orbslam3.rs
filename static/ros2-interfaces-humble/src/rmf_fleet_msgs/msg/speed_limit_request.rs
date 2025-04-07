use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeedLimitRequest {
    pub fleet_name: ::std::string::String,
    pub speed_limits: Vec<crate::rmf_fleet_msgs::msg::SpeedLimitedLane>,
    pub remove_limits: Vec<u64>,
}

impl Default for SpeedLimitRequest {
    fn default() -> Self {
        SpeedLimitRequest {
            fleet_name: ::std::string::String::new(),
            speed_limits: Vec::new(),
            remove_limits: Vec::new(),
        }
    }
}

impl ros2_client::Message for SpeedLimitRequest {}
