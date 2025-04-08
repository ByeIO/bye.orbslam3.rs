use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavSVIN {
    pub version: u8,
    pub reserved0: [u8; 3],
    pub i_tow: u32,
    pub dur: u32,
    pub mean_x: i32,
    pub mean_y: i32,
    pub mean_z: i32,
    pub mean_xhp: i8,
    pub mean_yhp: i8,
    pub mean_zhp: i8,
    pub reserved1: u8,
    pub mean_acc: u32,
    pub obs: u32,
    pub valid: u8,
    pub active: u8,
    pub reserved3: [u8; 2],
}

impl NavSVIN {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 59;
}

impl Default for NavSVIN {
    fn default() -> Self {
        NavSVIN {
            version: 0,
            reserved0: [0; 3],
            i_tow: 0,
            dur: 0,
            mean_x: 0,
            mean_y: 0,
            mean_z: 0,
            mean_xhp: 0,
            mean_yhp: 0,
            mean_zhp: 0,
            reserved1: 0,
            mean_acc: 0,
            obs: 0,
            valid: 0,
            active: 0,
            reserved3: [0; 2],
        }
    }
}

impl ros2_client::Message for NavSVIN {}
