use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavSig {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub version: u8,
    pub num_sigs: u8,
    pub reserved_0: [u8; 2],
    pub sig_data: Vec<crate::ublox_ubx_msgs::msg::SigData>,
}

impl Default for UBXNavSig {
    fn default() -> Self {
        UBXNavSig {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            version: 0,
            num_sigs: 0,
            reserved_0: [0; 2],
            sig_data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXNavSig {}
