use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavDGPS {
    pub i_tow: u32,
    pub age: i32,
    pub base_id: i16,
    pub base_health: i16,
    pub num_ch: i8,
    pub status: u8,
    pub reserved1: u16,
    pub sv: Vec<crate::ublox_msgs::msg::NavDGPSSV>,
}

impl NavDGPS {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 49;
    pub const DGPS_CORRECTION_NONE: u8 = 0;
    pub const DGPS_CORRECTION_PR_PRR: u8 = 1;
}

impl Default for NavDGPS {
    fn default() -> Self {
        NavDGPS {
            i_tow: 0,
            age: 0,
            base_id: 0,
            base_health: 0,
            num_ch: 0,
            status: 0,
            reserved1: 0,
            sv: Vec::new(),
        }
    }
}

impl ros2_client::Message for NavDGPS {}
