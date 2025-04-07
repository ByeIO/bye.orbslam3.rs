use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerState {
    pub header: crate::std_msgs::msg::Header,
    pub voltage: f64,
    pub current: f64,
    pub power_consumption: f64,
    pub remaining_capacity: f64,
    pub relative_remaining_capacity: f64,
    pub connected: bool,
    pub charging: bool,
    pub time_remaining: f64,
    pub temperature: f64,
}

impl Default for PowerState {
    fn default() -> Self {
        PowerState {
            header: crate::std_msgs::msg::Header::default(),
            voltage: 0.0,
            current: 0.0,
            power_consumption: 0.0,
            remaining_capacity: 0.0,
            relative_remaining_capacity: 0.0,
            connected: false,
            charging: false,
            time_remaining: 0.0,
            temperature: 0.0,
        }
    }
}

impl ros2_client::Message for PowerState {}
