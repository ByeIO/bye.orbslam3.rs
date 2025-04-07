use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrbEphInfo {
    pub eph_usability: u8,
    pub eph_source: u8,
}

impl OrbEphInfo {
    pub const EPH_USABILITY_UNKNOWN: u8 = 31;
    pub const EPH_USABILITY_OVER_450_MIN: u8 = 30;
    pub const EPH_USABILITY_EXPIRED: u8 = 0;
    pub const EPH_SOURCE_NOT_AVAILABLE: u8 = 0;
    pub const EPH_SOURCE_GNSS_TRANSMISSION: u8 = 1;
    pub const EPH_SOURCE_EXTERNAL_AIDING: u8 = 2;
}

impl Default for OrbEphInfo {
    fn default() -> Self {
        OrbEphInfo {
            eph_usability: 0,
            eph_source: 0,
        }
    }
}

impl ros2_client::Message for OrbEphInfo {}
