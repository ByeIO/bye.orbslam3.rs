use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimaryControl {
    pub header: crate::std_msgs::msg::Header,
    pub active: bool,
    pub estop: bool,
    pub steering_command: f32,
    pub throttle_command: f32,
    pub brake_command: f32,
}

impl Default for PrimaryControl {
    fn default() -> Self {
        PrimaryControl {
            header: crate::std_msgs::msg::Header::default(),
            active: false,
            estop: false,
            steering_command: 0.0,
            throttle_command: 0.0,
            brake_command: 0.0,
        }
    }
}

impl ros2_client::Message for PrimaryControl {}
