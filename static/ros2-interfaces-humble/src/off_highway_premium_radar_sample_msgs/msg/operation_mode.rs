use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationMode {
    pub operation_mode: u8,
}

impl OperationMode {
    pub const INITIALIZATION: u8 = 13;
    pub const NORMAL: u8 = 20;
    pub const MODULATION_OFF: u8 = 22;
    pub const ALIGNMENT_MODE: u8 = 40;
    pub const RADAR_OFF: u8 = 51;
    pub const DRIVE_TEST: u8 = 61;
    pub const ERROR: u8 = 100;
}

impl Default for OperationMode {
    fn default() -> Self {
        OperationMode {
            operation_mode: 0,
        }
    }
}

impl ros2_client::Message for OperationMode {}
