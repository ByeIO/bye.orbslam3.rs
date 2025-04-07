use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YawMode {
    pub mode: u8,
    pub angle: f32,
}

impl YawMode {
    pub const KEEP_YAW: u8 = 0;
    pub const PATH_FACING: u8 = 1;
    pub const FIXED_YAW: u8 = 2;
    pub const YAW_FROM_TOPIC: u8 = 3;
    pub const YAW_FROM_ORIENTATION: u8 = 4;
    pub const YAW_TO_FRAME: u8 = 5;
}

impl Default for YawMode {
    fn default() -> Self {
        YawMode {
            mode: 0,
            angle: 0.0,
        }
    }
}

impl ros2_client::Message for YawMode {}
