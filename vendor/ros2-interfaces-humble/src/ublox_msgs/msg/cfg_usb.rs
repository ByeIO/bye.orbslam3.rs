use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgUSB {
    pub vendor_id: u16,
    pub product_id: u16,
    pub reserved1: [u8; 2],
    pub reserved2: [u8; 2],
    pub power_consumption: u16,
    pub flags: u16,
    pub vendor_string: [i8; 32],
    pub product_string: [i8; 32],
    pub serial_number: [i8; 32],
}

impl CfgUSB {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 27;
    pub const FLAGS_RE_ENUM: u16 = 0;
    pub const FLAGS_POWER_MODE: u16 = 2;
}

impl Default for CfgUSB {
    fn default() -> Self {
        CfgUSB {
            vendor_id: 0,
            product_id: 0,
            reserved1: [0; 2],
            reserved2: [0; 2],
            power_consumption: 0,
            flags: 0,
            vendor_string: [0; 32],
            product_string: [0; 32],
            serial_number: [0; 32],
        }
    }
}

impl ros2_client::Message for CfgUSB {}
