use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SBASSvData {
    pub svid: u8,
    pub reserved_1: u8,
    pub udre: u8,
    pub sv_sys: u8,
    pub sv_service: u8,
    pub reserved_2: u8,
    pub prc: i16,
    pub reserved_3: [u8; 2],
    pub ic: i16,
}

impl Default for SBASSvData {
    fn default() -> Self {
        SBASSvData {
            svid: 0,
            reserved_1: 0,
            udre: 0,
            sv_sys: 0,
            sv_service: 0,
            reserved_2: 0,
            prc: 0,
            reserved_3: [0; 2],
            ic: 0,
        }
    }
}

impl ros2_client::Message for SBASSvData {}
