use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgNMEA6 {
    pub filter: u8,
    pub version: u8,
    pub num_sv: u8,
    pub flags: u8,
}

impl CfgNMEA6 {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 23;
    pub const FILTER_POS: u8 = 1;
    pub const FILTER_MSK_POS: u8 = 2;
    pub const FILTER_TIME: u8 = 4;
    pub const FILTER_DATE: u8 = 8;
    pub const FILTER_SBAS_FILT: u8 = 16;
    pub const FILTER_TRACK: u8 = 32;
    pub const NMEA_VERSION_2_3: u8 = 35;
    pub const NMEA_VERSION_2_1: u8 = 33;
    pub const FLAGS_COMPAT: u8 = 1;
    pub const FLAGS_CONSIDER: u8 = 2;
}

impl Default for CfgNMEA6 {
    fn default() -> Self {
        CfgNMEA6 {
            filter: 0,
            version: 0,
            num_sv: 0,
            flags: 0,
        }
    }
}

impl ros2_client::Message for CfgNMEA6 {}
