use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UlcCmd {
    pub header: crate::std_msgs::msg::Header,
    pub cmd: f32,
    pub limit_accel: f32,
    pub limit_decel: f32,
    pub limit_jerk_throttle: f32,
    pub limit_jerk_brake: f32,
    pub cmd_type: u8,
    pub enable: bool,
    pub clear: bool,
    pub enable_shift: bool,
    pub enable_shift_park: bool,
    pub coast_decel: bool,
}

impl UlcCmd {
    pub const CMD_NONE: u8 = 0;
    pub const CMD_VELOCITY: u8 = 1;
    pub const CMD_ACCEL: u8 = 2;
}

impl Default for UlcCmd {
    fn default() -> Self {
        UlcCmd {
            header: crate::std_msgs::msg::Header::default(),
            cmd: 0.0,
            limit_accel: 0.0,
            limit_decel: 0.0,
            limit_jerk_throttle: 0.0,
            limit_jerk_brake: 0.0,
            cmd_type: 0,
            enable: false,
            clear: false,
            enable_shift: false,
            enable_shift_park: false,
            coast_decel: false,
        }
    }
}

impl ros2_client::Message for UlcCmd {}
