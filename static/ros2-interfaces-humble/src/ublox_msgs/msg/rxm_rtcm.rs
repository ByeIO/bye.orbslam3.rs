use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmRTCM {
    pub version: u8,
    pub flags: u8,
    pub reserved0: [u8; 2],
    pub ref_station: u16,
    pub msg_type: u16,
}

impl RxmRTCM {
    pub const CLASS_ID: u8 = 2;
    pub const MESSAGE_ID: u8 = 50;
    pub const FLAGS_CRC_FAILED: u8 = 1;
}

impl Default for RxmRTCM {
    fn default() -> Self {
        RxmRTCM {
            version: 0,
            flags: 0,
            reserved0: [0; 2],
            ref_station: 0,
            msg_type: 0,
        }
    }
}

impl ros2_client::Message for RxmRTCM {}
