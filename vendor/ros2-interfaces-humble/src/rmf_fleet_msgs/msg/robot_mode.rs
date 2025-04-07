use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotMode {
    pub mode: u32,
    pub mode_request_id: u64,
}

impl RobotMode {
    pub const MODE_IDLE: u32 = 0;
    pub const MODE_CHARGING: u32 = 1;
    pub const MODE_MOVING: u32 = 2;
    pub const MODE_PAUSED: u32 = 3;
    pub const MODE_WAITING: u32 = 4;
    pub const MODE_EMERGENCY: u32 = 5;
    pub const MODE_GOING_HOME: u32 = 6;
    pub const MODE_DOCKING: u32 = 7;
    pub const MODE_ADAPTER_ERROR: u32 = 8;
    pub const MODE_CLEANING: u32 = 9;
}

impl Default for RobotMode {
    fn default() -> Self {
        RobotMode {
            mode: 0,
            mode_request_id: 0,
        }
    }
}

impl ros2_client::Message for RobotMode {}
