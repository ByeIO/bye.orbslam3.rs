use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtcStd {
    pub id: u8,
}

impl UtcStd {
    pub const NOT_AVAILABLE: u8 = 0;
    pub const CRL: u8 = 1;
    pub const NIST: u8 = 2;
    pub const USNO: u8 = 3;
    pub const BIPM: u8 = 4;
    pub const EURO: u8 = 5;
    pub const SU: u8 = 6;
    pub const NTSC: u8 = 7;
    pub const UTC_UNKNOWN: u8 = 15;
}

impl Default for UtcStd {
    fn default() -> Self {
        UtcStd {
            id: 0,
        }
    }
}

impl ros2_client::Message for UtcStd {}
