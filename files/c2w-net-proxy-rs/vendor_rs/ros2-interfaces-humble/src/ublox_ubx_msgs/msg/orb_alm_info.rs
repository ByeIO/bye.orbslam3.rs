use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrbAlmInfo {
    pub alm_usability: u8,
    pub alm_source: u8,
}

impl OrbAlmInfo {
    pub const ALM_USABILITY_UNKNOWN: u8 = 31;
    pub const ALM_USABILITY_OVER_30_DAYS: u8 = 30;
    pub const ALM_USABILITY_EXPIRED: u8 = 0;
    pub const ALM_SOURCE_NOT_AVAILABLE: u8 = 0;
    pub const ALM_SOURCE_GNSS_TRANSMISSION: u8 = 1;
    pub const ALM_SOURCE_EXTERNAL_AIDING: u8 = 2;
}

impl Default for OrbAlmInfo {
    fn default() -> Self {
        OrbAlmInfo {
            alm_usability: 0,
            alm_source: 0,
        }
    }
}

impl ros2_client::Message for OrbAlmInfo {}
