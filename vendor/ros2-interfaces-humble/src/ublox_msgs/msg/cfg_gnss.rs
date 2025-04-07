use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgGNSS {
    pub msg_ver: u8,
    pub num_trk_ch_hw: u8,
    pub num_trk_ch_use: u8,
    pub num_config_blocks: u8,
    pub blocks: Vec<crate::ublox_msgs::msg::CfgGNSSBlock>,
}

impl CfgGNSS {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 62;
}

impl Default for CfgGNSS {
    fn default() -> Self {
        CfgGNSS {
            msg_ver: 0,
            num_trk_ch_hw: 0,
            num_trk_ch_use: 0,
            num_config_blocks: 0,
            blocks: Vec::new(),
        }
    }
}

impl ros2_client::Message for CfgGNSS {}
