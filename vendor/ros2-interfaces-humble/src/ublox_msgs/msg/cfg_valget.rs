use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgVALGET {
    pub version: u8,
    pub layers: u8,
    pub position: u16,
    pub keys: Vec<u32>,
}

impl CfgVALGET {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 139;
    pub const LAYER_RAM: u8 = 0;
    pub const LAYER_BBR: u8 = 1;
    pub const LAYER_FLASH: u8 = 2;
    pub const LAYER_DEFAULT: u8 = 7;
}

impl Default for CfgVALGET {
    fn default() -> Self {
        CfgVALGET {
            version: 0,
            layers: 0,
            position: 0,
            keys: Vec::new(),
        }
    }
}

impl ros2_client::Message for CfgVALGET {}
