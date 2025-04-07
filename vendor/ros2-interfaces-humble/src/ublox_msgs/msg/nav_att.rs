use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavATT {
    pub i_tow: u32,
    pub version: u8,
    pub reserved0: [u8; 3],
    pub roll: i32,
    pub pitch: i32,
    pub heading: i32,
    pub acc_roll: u32,
    pub acc_pitch: u32,
    pub acc_heading: u32,
}

impl NavATT {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 5;
}

impl Default for NavATT {
    fn default() -> Self {
        NavATT {
            i_tow: 0,
            version: 0,
            reserved0: [0; 3],
            roll: 0,
            pitch: 0,
            heading: 0,
            acc_roll: 0,
            acc_pitch: 0,
            acc_heading: 0,
        }
    }
}

impl ros2_client::Message for NavATT {}
