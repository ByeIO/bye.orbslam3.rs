use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmRAWX {
    pub rcv_tow: f64,
    pub week: u16,
    pub leap_s: i8,
    pub num_meas: u8,
    pub rec_stat: u8,
    pub version: u8,
    pub reserved1: [u8; 2],
    pub meas: Vec<crate::ublox_msgs::msg::RxmRAWXMeas>,
}

impl RxmRAWX {
    pub const CLASS_ID: u8 = 2;
    pub const MESSAGE_ID: u8 = 21;
    pub const REC_STAT_LEAP_SEC: u8 = 1;
    pub const REC_STAT_CLK_RESET: u8 = 2;
}

impl Default for RxmRAWX {
    fn default() -> Self {
        RxmRAWX {
            rcv_tow: 0.0,
            week: 0,
            leap_s: 0,
            num_meas: 0,
            rec_stat: 0,
            version: 0,
            reserved1: [0; 2],
            meas: Vec::new(),
        }
    }
}

impl ros2_client::Message for RxmRAWX {}
