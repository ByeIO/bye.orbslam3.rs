use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgVALSET {
    pub version: u8,
    pub layers: u8,
    pub reserved0: [u8; 2],
    pub cfgdata: Vec<crate::ublox_msgs::msg::CfgVALSETCfgdata>,
}

impl CfgVALSET {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 138;
    pub const LAYER_RAM: u8 = 1;
    pub const LAYER_BBR: u8 = 2;
    pub const LAYER_FLASH: u8 = 4;
}

impl Default for CfgVALSET {
    fn default() -> Self {
        CfgVALSET {
            version: 0,
            layers: 0,
            reserved0: [0; 2],
            cfgdata: Vec::new(),
        }
    }
}

impl ros2_client::Message for CfgVALSET {}
