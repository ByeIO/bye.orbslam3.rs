use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgSBAS {
    pub mode: u8,
    pub usage: u8,
    pub max_sbas: u8,
    pub scanmode2: u8,
    pub scanmode1: u32,
}

impl CfgSBAS {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 22;
    pub const MODE_ENABLED: u8 = 1;
    pub const MODE_TEST: u8 = 2;
    pub const USAGE_RANGE: u8 = 1;
    pub const USAGE_DIFF_CORR: u8 = 2;
    pub const USAGE_INTEGRITY: u8 = 4;
}

impl Default for CfgSBAS {
    fn default() -> Self {
        CfgSBAS {
            mode: 0,
            usage: 0,
            max_sbas: 0,
            scanmode2: 0,
            scanmode1: 0,
        }
    }
}

impl ros2_client::Message for CfgSBAS {}
