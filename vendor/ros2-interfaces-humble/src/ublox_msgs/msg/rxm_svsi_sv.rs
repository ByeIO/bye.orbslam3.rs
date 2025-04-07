use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmSVSI_SV {
    pub svid: u8,
    pub svFlag: u8,
    pub azim: i16,
    pub elev: i8,
    pub age: u8,
}

impl RxmSVSI_SV {
    pub const FLAG_URA_MASK: u8 = 15;
    pub const FLAG_HEALTHY: u8 = 16;
    pub const FLAG_EPH_VAL: u8 = 32;
    pub const FLAG_ALM_VAL: u8 = 64;
    pub const FLAG_NOT_AVAIL: u8 = 128;
    pub const AGE_ALM_MASK: u8 = 15;
    pub const AGE_EPH_MASK: u8 = 240;
}

impl Default for RxmSVSI_SV {
    fn default() -> Self {
        RxmSVSI_SV {
            svid: 0,
            svFlag: 0,
            azim: 0,
            elev: 0,
            age: 0,
        }
    }
}

impl ros2_client::Message for RxmSVSI_SV {}
