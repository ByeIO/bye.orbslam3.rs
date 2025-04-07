use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmRAW {
    pub rcv_tow: i32,
    pub week: i16,
    pub num_sv: u8,
    pub reserved1: u8,
    pub sv: Vec<crate::ublox_msgs::msg::RxmRAWSV>,
}

impl RxmRAW {
    pub const CLASS_ID: u8 = 2;
    pub const MESSAGE_ID: u8 = 16;
}

impl Default for RxmRAW {
    fn default() -> Self {
        RxmRAW {
            rcv_tow: 0,
            week: 0,
            num_sv: 0,
            reserved1: 0,
            sv: Vec::new(),
        }
    }
}

impl ros2_client::Message for RxmRAW {}
