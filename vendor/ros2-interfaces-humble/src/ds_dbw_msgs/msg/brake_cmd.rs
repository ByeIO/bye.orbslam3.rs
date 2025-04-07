use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrakeCmd {
    pub header: crate::std_msgs::msg::Header,
    pub cmd: f32,
    pub rate_inc: f32,
    pub rate_dec: f32,
    pub cmd_type: u8,
    pub enable: bool,
    pub clear: bool,
    pub ignore: bool,
}

impl BrakeCmd {
    pub const CMD_NONE: u8 = 0;
    pub const CMD_PRESSURE: u8 = 1;
    pub const CMD_TORQUE: u8 = 2;
    pub const CMD_ACCEL: u8 = 8;
    pub const CMD_ACCEL_ACC: u8 = 9;
    pub const CMD_ACCEL_AEB: u8 = 10;
    pub const CMD_PEDAL_RAW: u8 = 13;
    pub const CMD_PERCENT: u8 = 14;
}

impl Default for BrakeCmd {
    fn default() -> Self {
        BrakeCmd {
            header: crate::std_msgs::msg::Header::default(),
            cmd: 0.0,
            rate_inc: 0.0,
            rate_dec: 0.0,
            cmd_type: 0,
            enable: false,
            clear: false,
            ignore: false,
        }
    }
}

impl ros2_client::Message for BrakeCmd {}
