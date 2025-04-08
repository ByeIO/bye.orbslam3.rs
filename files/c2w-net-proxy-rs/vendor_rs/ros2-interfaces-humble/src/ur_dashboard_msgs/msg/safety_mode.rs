use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyMode {
    pub mode: u8,
}

impl SafetyMode {
    pub const NORMAL: u8 = 1;
    pub const REDUCED: u8 = 2;
    pub const PROTECTIVE_STOP: u8 = 3;
    pub const RECOVERY: u8 = 4;
    pub const SAFEGUARD_STOP: u8 = 5;
    pub const SYSTEM_EMERGENCY_STOP: u8 = 6;
    pub const ROBOT_EMERGENCY_STOP: u8 = 7;
    pub const VIOLATION: u8 = 8;
    pub const FAULT: u8 = 9;
    pub const VALIDATE_JOINT_ID: u8 = 10;
    pub const UNDEFINED_SAFETY_MODE: u8 = 11;
    pub const AUTOMATIC_MODE_SAFEGUARD_STOP: u8 = 12;
    pub const SYSTEM_THREE_POSITION_ENABLING_STOP: u8 = 13;
}

impl Default for SafetyMode {
    fn default() -> Self {
        SafetyMode {
            mode: 0,
        }
    }
}

impl ros2_client::Message for SafetyMode {}
