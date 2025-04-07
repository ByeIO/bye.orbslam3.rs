use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotMode {
    pub mode: i8,
}

impl RobotMode {
    pub const NO_CONTROLLER: i8 = -1;
    pub const DISCONNECTED: i8 = 0;
    pub const CONFIRM_SAFETY: i8 = 1;
    pub const BOOTING: i8 = 2;
    pub const POWER_OFF: i8 = 3;
    pub const POWER_ON: i8 = 4;
    pub const IDLE: i8 = 5;
    pub const BACKDRIVE: i8 = 6;
    pub const RUNNING: i8 = 7;
    pub const UPDATING_FIRMWARE: i8 = 8;
}

impl Default for RobotMode {
    fn default() -> Self {
        RobotMode {
            mode: 0,
        }
    }
}

impl ros2_client::Message for RobotMode {}
