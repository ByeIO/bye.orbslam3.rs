use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXRxmRTCM {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub crc_failed: bool,
    pub msg_used: u8,
    pub sub_type: u16,
    pub ref_station: u16,
    pub msg_type: u16,
}

impl Default for UBXRxmRTCM {
    fn default() -> Self {
        UBXRxmRTCM {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            crc_failed: false,
            msg_used: 0,
            sub_type: 0,
            ref_station: 0,
            msg_type: 0,
        }
    }
}

impl ros2_client::Message for UBXRxmRTCM {}
