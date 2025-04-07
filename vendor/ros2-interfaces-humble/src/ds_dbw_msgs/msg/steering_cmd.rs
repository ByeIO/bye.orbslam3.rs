use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteeringCmd {
    pub header: crate::std_msgs::msg::Header,
    pub cmd: f32,
    pub cmd_rate: f32,
    pub cmd_accel: f32,
    pub cmd_type: u8,
    pub enable: bool,
    pub clear: bool,
    pub ignore: bool,
}

impl SteeringCmd {
    pub const CMD_NONE: u8 = 0;
    pub const CMD_TORQUE: u8 = 1;
    pub const CMD_ANGLE: u8 = 2;
    pub const CMD_CURVATURE: u8 = 3;
    pub const CMD_YAW_RATE: u8 = 4;
    pub const CMD_PERCENT: u8 = 14;
    pub const CMD_CALIBRATE: u8 = 15;
}

impl Default for SteeringCmd {
    fn default() -> Self {
        SteeringCmd {
            header: crate::std_msgs::msg::Header::default(),
            cmd: 0.0,
            cmd_rate: 0.0,
            cmd_accel: 0.0,
            cmd_type: 0,
            enable: false,
            clear: false,
            ignore: false,
        }
    }
}

impl ros2_client::Message for SteeringCmd {}
