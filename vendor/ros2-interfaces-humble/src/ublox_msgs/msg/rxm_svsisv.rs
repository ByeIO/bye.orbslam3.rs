use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmSVSISV {
    pub svid: u8,
    pub sv_flag: u8,
    pub azim: i16,
    pub elev: i8,
    pub age: u8,
}

impl RxmSVSISV {
    pub const FLAG_URA_MASK: u8 = 15;
    pub const FLAG_HEALTHY: u8 = 16;
    pub const FLAG_EPH_VAL: u8 = 32;
    pub const FLAG_ALM_VAL: u8 = 64;
    pub const FLAG_NOT_AVAIL: u8 = 128;
    pub const AGE_ALM_MASK: u8 = 15;
    pub const AGE_EPH_MASK: u8 = 240;
}

impl Default for RxmSVSISV {
    fn default() -> Self {
        RxmSVSISV {
            svid: 0,
            sv_flag: 0,
            azim: 0,
            elev: 0,
            age: 0,
        }
    }
}

impl ros2_client::Message for RxmSVSISV {}
