use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgINFBlock {
    pub protocol_id: u8,
    pub reserved1: [u8; 3],
    pub inf_msg_mask: [u8; 6],
}

impl CfgINFBlock {
    pub const PROTOCOL_ID_UBX: u8 = 0;
    pub const PROTOCOL_ID_NMEA: u8 = 1;
    pub const INF_MSG_ERROR: u8 = 1;
    pub const INF_MSG_WARNING: u8 = 2;
    pub const INF_MSG_NOTICE: u8 = 4;
    pub const INF_MSG_TEST: u8 = 8;
    pub const INF_MSG_DEBUG: u8 = 16;
}

impl Default for CfgINFBlock {
    fn default() -> Self {
        CfgINFBlock {
            protocol_id: 0,
            reserved1: [0; 3],
            inf_msg_mask: [0; 6],
        }
    }
}

impl ros2_client::Message for CfgINFBlock {}
