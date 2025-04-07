use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsfSTATUS {
    pub i_tow: u32,
    pub version: u8,
    pub reserved1: [u8; 7],
    pub fusion_mode: u8,
    pub reserved2: [u8; 2],
    pub num_sens: u8,
    pub sens: Vec<crate::ublox_msgs::msg::EsfSTATUSSens>,
}

impl EsfSTATUS {
    pub const CLASS_ID: u8 = 16;
    pub const MESSAGE_ID: u8 = 16;
    pub const FUSION_MODE_INIT: u8 = 0;
    pub const FUSION_MODE_FUSION: u8 = 1;
    pub const FUSION_MODE_SUSPENDED: u8 = 2;
    pub const FUSION_MODE_DISABLED: u8 = 3;
}

impl Default for EsfSTATUS {
    fn default() -> Self {
        EsfSTATUS {
            i_tow: 0,
            version: 0,
            reserved1: [0; 7],
            fusion_mode: 0,
            reserved2: [0; 2],
            num_sens: 0,
            sens: Vec::new(),
        }
    }
}

impl ros2_client::Message for EsfSTATUS {}
