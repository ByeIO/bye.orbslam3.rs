use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgNMEA7 {
    pub filter: u8,
    pub nmea_version: u8,
    pub num_sv: u8,
    pub flags: u8,
    pub gnss_to_filter: u32,
    pub sv_numbering: u8,
    pub main_talker_id: u8,
    pub gsv_talker_id: u8,
    pub reserved: u8,
}

impl CfgNMEA7 {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 23;
    pub const FILTER_POS: u8 = 1;
    pub const FILTER_MSK_POS: u8 = 2;
    pub const FILTER_TIME: u8 = 4;
    pub const FILTER_DATE: u8 = 8;
    pub const FILTER_GPS_ONLY: u8 = 16;
    pub const FILTER_TRACK: u8 = 32;
    pub const NMEA_VERSION_2_3: u8 = 35;
    pub const NMEA_VERSION_2_1: u8 = 33;
    pub const NUM_SV_UNLIMITED: u8 = 0;
    pub const FLAGS_COMPAT: u8 = 1;
    pub const FLAGS_CONSIDER: u8 = 2;
    pub const GNSS_TO_FILTER_GPS: u32 = 1;
    pub const GNSS_TO_FILTER_SBAS: u32 = 2;
    pub const GNSS_TO_FILTER_QZSS: u32 = 16;
    pub const GNSS_TO_FILTER_GLONASS: u32 = 32;
    pub const SV_NUMBERING_STRICT: u8 = 0;
    pub const SV_NUMBERING_EXTENDED: u8 = 1;
    pub const MAIN_TALKER_ID_NOT_OVERRIDDEN: u8 = 0;
    pub const MAIN_TALKER_ID_GP: u8 = 1;
    pub const MAIN_TALKER_ID_GL: u8 = 2;
    pub const MAIN_TALKER_ID_GN: u8 = 3;
    pub const GSV_TALKER_ID_GNSS_SPECIFIC: u8 = 0;
    pub const GSV_TALKER_ID_MAIN: u8 = 1;
}

impl Default for CfgNMEA7 {
    fn default() -> Self {
        CfgNMEA7 {
            filter: 0,
            nmea_version: 0,
            num_sv: 0,
            flags: 0,
            gnss_to_filter: 0,
            sv_numbering: 0,
            main_talker_id: 0,
            gsv_talker_id: 0,
            reserved: 0,
        }
    }
}

impl ros2_client::Message for CfgNMEA7 {}
