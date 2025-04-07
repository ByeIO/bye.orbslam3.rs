use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgCFG {
    pub clear_mask: u32,
    pub save_mask: u32,
    pub load_mask: u32,
    pub device_mask: u8,
}

impl CfgCFG {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 9;
    pub const MASK_IO_PORT: u32 = 1;
    pub const MASK_MSG_CONF: u32 = 2;
    pub const MASK_INF_MSG: u32 = 4;
    pub const MASK_NAV_CONF: u32 = 8;
    pub const MASK_RXM_CONF: u32 = 16;
    pub const MASK_SEN_CONF: u32 = 256;
    pub const MASK_RINV_CONF: u32 = 512;
    pub const MASK_ANT_CONF: u32 = 1024;
    pub const MASK_LOG_CONF: u32 = 2048;
    pub const MASK_FTS_CONF: u32 = 4096;
    pub const DEV_BBR: u8 = 1;
    pub const DEV_FLASH: u8 = 2;
    pub const DEV_EEPROM: u8 = 4;
    pub const DEV_SPI_FLASH: u8 = 16;
}

impl Default for CfgCFG {
    fn default() -> Self {
        CfgCFG {
            clear_mask: 0,
            save_mask: 0,
            load_mask: 0,
            device_mask: 0,
        }
    }
}

impl ros2_client::Message for CfgCFG {}
