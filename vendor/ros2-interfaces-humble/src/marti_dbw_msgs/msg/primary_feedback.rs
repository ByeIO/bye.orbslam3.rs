use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimaryFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub present: bool,
    pub robotic_mode: bool,
    pub steering_command: f32,
    pub steering_measure: f32,
    pub throttle_command: f32,
    pub throttle_measure: f32,
    pub brake_command: f32,
    pub brake_measure: f32,
    pub estop_command: bool,
    pub estop_measure: bool,
    pub error_steering: bool,
    pub error_throttle: bool,
    pub error_brake: bool,
    pub error_other: bool,
}

impl Default for PrimaryFeedback {
    fn default() -> Self {
        PrimaryFeedback {
            header: crate::std_msgs::msg::Header::default(),
            present: false,
            robotic_mode: false,
            steering_command: 0.0,
            steering_measure: 0.0,
            throttle_command: 0.0,
            throttle_measure: 0.0,
            brake_command: 0.0,
            brake_measure: 0.0,
            estop_command: false,
            estop_measure: false,
            error_steering: false,
            error_throttle: false,
            error_brake: false,
            error_other: false,
        }
    }
}

impl ros2_client::Message for PrimaryFeedback {}
