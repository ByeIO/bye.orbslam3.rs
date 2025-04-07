use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavOrb {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub version: u8,
    pub num_sv: u8,
    pub reserved_0: [u8; 2],
    pub sv_info: Vec<crate::ublox_ubx_msgs::msg::OrbSVInfo>,
}

impl Default for UBXNavOrb {
    fn default() -> Self {
        UBXNavOrb {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            version: 0,
            num_sv: 0,
            reserved_0: [0; 2],
            sv_info: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXNavOrb {}
