use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavSAT {
    pub i_tow: u32,
    pub version: u8,
    pub num_svs: u8,
    pub reserved0: [u8; 2],
    pub sv: Vec<crate::ublox_msgs::msg::NavSATSV>,
}

impl NavSAT {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 53;
}

impl Default for NavSAT {
    fn default() -> Self {
        NavSAT {
            i_tow: 0,
            version: 0,
            num_svs: 0,
            reserved0: [0; 2],
            sv: Vec::new(),
        }
    }
}

impl ros2_client::Message for NavSAT {}
