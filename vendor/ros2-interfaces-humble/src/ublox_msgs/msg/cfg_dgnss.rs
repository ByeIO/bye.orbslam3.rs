use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgDGNSS {
    pub dgnss_mode: u8,
    pub reserved0: [u8; 3],
}

impl CfgDGNSS {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 112;
    pub const DGNSS_MODE_RTK_FLOAT: u8 = 2;
    pub const DGNSS_MODE_RTK_FIXED: u8 = 3;
}

impl Default for CfgDGNSS {
    fn default() -> Self {
        CfgDGNSS {
            dgnss_mode: 0,
            reserved0: [0; 3],
        }
    }
}

impl ros2_client::Message for CfgDGNSS {}
