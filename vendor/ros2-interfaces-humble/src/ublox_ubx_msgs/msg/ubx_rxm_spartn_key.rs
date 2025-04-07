use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXRxmSpartnKey {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub num_keys: u8,
    pub reserved0: [u8; 2],
    pub key_info: Vec<crate::ublox_ubx_msgs::msg::SpartnKeyInfo>,
    pub key_payload: Vec<u8>,
}

impl Default for UBXRxmSpartnKey {
    fn default() -> Self {
        UBXRxmSpartnKey {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            num_keys: 0,
            reserved0: [0; 2],
            key_info: Vec::new(),
            key_payload: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXRxmSpartnKey {}
