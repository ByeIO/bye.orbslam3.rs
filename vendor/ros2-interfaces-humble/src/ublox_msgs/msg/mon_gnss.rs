use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonGNSS {
    pub version: u8,
    pub supported: u8,
    pub default_gnss: u8,
    pub enabled: u8,
    pub simultaneous: u8,
    pub reserved1: [u8; 3],
}

impl MonGNSS {
    pub const CLASS_ID: u8 = 10;
    pub const MESSAGE_ID: u8 = 40;
    pub const BIT_MASK_GPS: u8 = 1;
    pub const BIT_MASK_GLONASS: u8 = 2;
    pub const BIT_MASK_BEIDOU: u8 = 4;
    pub const BIT_MASK_GALILEO: u8 = 8;
}

impl Default for MonGNSS {
    fn default() -> Self {
        MonGNSS {
            version: 0,
            supported: 0,
            default_gnss: 0,
            enabled: 0,
            simultaneous: 0,
            reserved1: [0; 3],
        }
    }
}

impl ros2_client::Message for MonGNSS {}
