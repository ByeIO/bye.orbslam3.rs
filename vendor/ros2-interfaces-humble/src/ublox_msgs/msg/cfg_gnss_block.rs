use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgGNSSBlock {
    pub gnss_id: u8,
    pub res_trk_ch: u8,
    pub max_trk_ch: u8,
    pub reserved1: u8,
    pub flags: u32,
}

impl CfgGNSSBlock {
    pub const GNSS_ID_GPS: u8 = 0;
    pub const GNSS_ID_SBAS: u8 = 1;
    pub const GNSS_ID_GALILEO: u8 = 2;
    pub const GNSS_ID_BEIDOU: u8 = 3;
    pub const GNSS_ID_IMES: u8 = 4;
    pub const GNSS_ID_QZSS: u8 = 5;
    pub const GNSS_ID_GLONASS: u8 = 6;
    pub const RES_TRK_CH_GPS: u8 = 8;
    pub const RES_TRK_CH_QZSS: u8 = 0;
    pub const RES_TRK_CH_SBAS: u8 = 0;
    pub const RES_TRK_CH_GLONASS: u8 = 8;
    pub const MAX_TRK_CH_MAJOR_MIN: u8 = 4;
    pub const MAX_TRK_CH_GPS: u8 = 16;
    pub const MAX_TRK_CH_GLONASS: u8 = 14;
    pub const MAX_TRK_CH_QZSS: u8 = 3;
    pub const MAX_TRK_CH_SBAS: u8 = 3;
    pub const FLAGS_ENABLE: u32 = 1;
    pub const FLAGS_SIG_CFG_MASK: u32 = 16711680;
    pub const SIG_CFG_GPS_L1CA: u32 = 65536;
    pub const SIG_CFG_SBAS_L1CA: u32 = 65536;
    pub const SIG_CFG_GALILEO_E1OS: u32 = 65536;
    pub const SIG_CFG_BEIDOU_B1I: u32 = 65536;
    pub const SIG_CFG_IMES_L1: u32 = 65536;
    pub const SIG_CFG_QZSS_L1CA: u32 = 65536;
    pub const SIG_CFG_QZSS_L1SAIF: u32 = 262144;
    pub const SIG_CFG_GLONASS_L1OF: u32 = 65536;
}

impl Default for CfgGNSSBlock {
    fn default() -> Self {
        CfgGNSSBlock {
            gnss_id: 0,
            res_trk_ch: 0,
            max_trk_ch: 0,
            reserved1: 0,
            flags: 0,
        }
    }
}

impl ros2_client::Message for CfgGNSSBlock {}
