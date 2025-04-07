use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsfINS {
    pub bitfield0: u32,
    pub reserved1: [u8; 4],
    pub i_tow: u32,
    pub x_ang_rate: i32,
    pub y_ang_rate: i32,
    pub z_ang_rate: i32,
    pub x_accel: i32,
    pub y_accel: i32,
    pub z_accel: i32,
}

impl EsfINS {
    pub const CLASS_ID: u8 = 16;
    pub const MESSAGE_ID: u8 = 21;
    pub const BITFIELD0_VERSION: u32 = 255;
    pub const BITFIELD0_X_ANG_RATE_VALID: u32 = 256;
    pub const BITFIELD0_Y_ANG_RATE_VALID: u32 = 512;
    pub const BITFIELD0_Z_ANG_RATE_VALID: u32 = 1024;
    pub const BITFIELD0_X_ACCEL_VALID: u32 = 2048;
    pub const BITFIELD0_Y_ACCEL_VALID: u32 = 4096;
    pub const BITFIELD0_Z_ACCEL_VALID: u32 = 8192;
}

impl Default for EsfINS {
    fn default() -> Self {
        EsfINS {
            bitfield0: 0,
            reserved1: [0; 4],
            i_tow: 0,
            x_ang_rate: 0,
            y_ang_rate: 0,
            z_ang_rate: 0,
            x_accel: 0,
            y_accel: 0,
            z_accel: 0,
        }
    }
}

impl ros2_client::Message for EsfINS {}
