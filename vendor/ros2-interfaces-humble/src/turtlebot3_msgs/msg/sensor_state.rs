use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorState {
    pub header: crate::std_msgs::msg::Header,
    pub bumper: u8,
    pub cliff: f32,
    pub sonar: f32,
    pub illumination: f32,
    pub led: u8,
    pub button: u8,
    pub torque: bool,
    pub left_encoder: i32,
    pub right_encoder: i32,
    pub battery: f32,
}

impl SensorState {
    pub const BUMPER_FORWARD: u8 = 1;
    pub const BUMPER_BACKWARD: u8 = 2;
    pub const CLIFF: u8 = 1;
    pub const SONAR: u8 = 1;
    pub const ILLUMINATION: u8 = 1;
    pub const BUTTON0: u8 = 1;
    pub const BUTTON1: u8 = 2;
    pub const ERROR_LEFT_MOTOR: u8 = 1;
    pub const ERROR_RIGHT_MOTOR: u8 = 2;
    pub const TORQUE_ON: u8 = 1;
    pub const TORQUE_OFF: u8 = 2;
}

impl Default for SensorState {
    fn default() -> Self {
        SensorState {
            header: crate::std_msgs::msg::Header::default(),
            bumper: 0,
            cliff: 0.0,
            sonar: 0.0,
            illumination: 0.0,
            led: 0,
            button: 0,
            torque: false,
            left_encoder: 0,
            right_encoder: 0,
            battery: 0.0,
        }
    }
}

impl ros2_client::Message for SensorState {}
