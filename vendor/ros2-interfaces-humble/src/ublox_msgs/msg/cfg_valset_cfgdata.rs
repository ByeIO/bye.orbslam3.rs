use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgVALSETCfgdata {
    pub key: u32,
    pub data: Vec<u8>,
}

impl CfgVALSETCfgdata {
    pub const GPS_ENABLE: u32 = 271646751;
    pub const GPS_L1CA_ENABLE: u32 = 271646721;
    pub const GPS_L2C_ENABLE: u32 = 271646723;
    pub const SBAS_ENABLE: u32 = 271646752;
    pub const SBAS_L1CA_ENABLE: u32 = 271646725;
    pub const GAL_ENABLE: u32 = 271646753;
    pub const GAL_E1_ENABLE: u32 = 271646727;
    pub const GAL_E5B_ENABLE: u32 = 271646730;
    pub const BDS_ENABLE: u32 = 271646754;
    pub const BDS_B1_ENABLE: u32 = 271646733;
    pub const BDS_B2_ENABLE: u32 = 271646734;
    pub const QZSS_ENABLE: u32 = 271646756;
    pub const QZSS_L1CA_ENABLE: u32 = 271646738;
    pub const QZSS_L1S_ENABLE: u32 = 271646740;
    pub const QZSS_L2C_ENABLE: u32 = 271646741;
    pub const GLO_ENABLE: u32 = 271646757;
    pub const GLO_L1_ENABLE: u32 = 271646744;
    pub const GLO_L2_ENABLE: u32 = 271646746;
}

impl Default for CfgVALSETCfgdata {
    fn default() -> Self {
        CfgVALSETCfgdata {
            key: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for CfgVALSETCfgdata {}
