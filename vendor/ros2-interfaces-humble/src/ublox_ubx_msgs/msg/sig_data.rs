use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SigData {
    pub gnss_id: u8,
    pub sv_id: u8,
    pub sig_id: u8,
    pub freq_id: u8,
    pub pr_res: i16,
    pub cno: u8,
    pub quality_ind: u8,
    pub corr_source: u8,
    pub iono_model: u8,
    pub sig_flags: crate::ublox_ubx_msgs::msg::SigFlags,
}

impl SigData {
    pub const QUALITY_NO_SIGNAL: u8 = 0;
    pub const QUALITY_SEARCHING_SIGNAL: u8 = 1;
    pub const QUALITY_SIGNAL_ACQUIRED: u8 = 2;
    pub const QUALITY_SIGNAL_UNUSABLE: u8 = 3;
    pub const QUALITY_CODE_LOCKED: u8 = 4;
    pub const QUALITY_CODE_CARRIER_LOCKED: u8 = 5;
    pub const CORR_NONE: u8 = 0;
    pub const CORR_SBAS: u8 = 1;
    pub const CORR_BEIDOU: u8 = 2;
    pub const CORR_RTCM2: u8 = 3;
    pub const CORR_RTCM3_OSR: u8 = 4;
    pub const CORR_RTCM3_SSR: u8 = 5;
    pub const CORR_QZSS_SLAS: u8 = 6;
    pub const CORR_SPARTN: u8 = 7;
    pub const CORR_CLAS: u8 = 8;
    pub const IONO_NONE: u8 = 0;
    pub const IONO_KLOB_GPS: u8 = 1;
    pub const IONO_SBAS: u8 = 2;
    pub const IONO_KLOB_BEIDOU: u8 = 3;
}

impl Default for SigData {
    fn default() -> Self {
        SigData {
            gnss_id: 0,
            sv_id: 0,
            sig_id: 0,
            freq_id: 0,
            pr_res: 0,
            cno: 0,
            quality_ind: 0,
            corr_source: 0,
            iono_model: 0,
            sig_flags: crate::ublox_ubx_msgs::msg::SigFlags::default(),
        }
    }
}

impl ros2_client::Message for SigData {}
