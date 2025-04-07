use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavSat {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub version: u8,
    pub num_svs: u8,
    pub sv_info: Vec<crate::ublox_ubx_msgs::msg::SatInfo>,
}

impl Default for UBXNavSat {
    fn default() -> Self {
        UBXNavSat {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            version: 0,
            num_svs: 0,
            sv_info: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXNavSat {}
