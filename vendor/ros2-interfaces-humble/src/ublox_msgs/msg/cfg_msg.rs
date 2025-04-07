use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgMSG {
    pub msg_class: u8,
    pub msg_id: u8,
    pub rate: u8,
}

impl CfgMSG {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 1;
}

impl Default for CfgMSG {
    fn default() -> Self {
        CfgMSG {
            msg_class: 0,
            msg_id: 0,
            rate: 0,
        }
    }
}

impl ros2_client::Message for CfgMSG {}
