use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgHNR {
    pub high_nav_rate: u8,
    pub reserved0: [u8; 3],
}

impl CfgHNR {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 92;
}

impl Default for CfgHNR {
    fn default() -> Self {
        CfgHNR {
            high_nav_rate: 0,
            reserved0: [0; 3],
        }
    }
}

impl ros2_client::Message for CfgHNR {}
