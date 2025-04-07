use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpartnKeyInfo {
    pub reserved1: u8,
    pub key_length_bytes: u8,
    pub valid_from_wno: u16,
    pub valid_from_tow: u32,
}

impl Default for SpartnKeyInfo {
    fn default() -> Self {
        SpartnKeyInfo {
            reserved1: 0,
            key_length_bytes: 0,
            valid_from_wno: 0,
            valid_from_tow: 0,
        }
    }
}

impl ros2_client::Message for SpartnKeyInfo {}
