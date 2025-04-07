use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanFrame {
    pub header: crate::std_msgs::msg::Header,
    pub id: u32,
    pub msg_type: u8,
    pub data_length: u8,
    pub data: [u8; 8],
}

impl CanFrame {
    pub const MSGTYPE_STANDARD: u8 = 0;
    pub const MSGTYPE_RTR: u8 = 1;
    pub const MSGTYPE_EXTENDED: u8 = 2;
    pub const MSGTYPE_STATUS: u8 = 128;
}

impl Default for CanFrame {
    fn default() -> Self {
        CanFrame {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            msg_type: 0,
            data_length: 0,
            data: [0; 8],
        }
    }
}

impl ros2_client::Message for CanFrame {}
