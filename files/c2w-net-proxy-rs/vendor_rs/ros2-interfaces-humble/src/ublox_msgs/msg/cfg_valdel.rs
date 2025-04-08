use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgVALDEL {
    pub version: u8,
    pub layers: u8,
    pub reserved0: [u8; 2],
    pub keys: Vec<u32>,
}

impl CfgVALDEL {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 140;
    pub const LAYER_BBR: u8 = 1;
    pub const LAYER_FLASH: u8 = 2;
}

impl Default for CfgVALDEL {
    fn default() -> Self {
        CfgVALDEL {
            version: 0,
            layers: 0,
            reserved0: [0; 2],
            keys: Vec::new(),
        }
    }
}

impl ros2_client::Message for CfgVALDEL {}
