use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXRxmSpartn {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub msg_used: u8,
    pub sub_type: u16,
    pub msg_type: u16,
}

impl UBXRxmSpartn {
    pub const MSG_UNKNOWN: u8 = 0;
    pub const MSG_NOT_USED: u8 = 1;
    pub const MSG_USED: u8 = 2;
}

impl Default for UBXRxmSpartn {
    fn default() -> Self {
        UBXRxmSpartn {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            msg_used: 0,
            sub_type: 0,
            msg_type: 0,
        }
    }
}

impl ros2_client::Message for UBXRxmSpartn {}
