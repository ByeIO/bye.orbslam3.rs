use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavSBASSV {
    pub svid: u8,
    pub flags: u8,
    pub udre: u8,
    pub sv_sys: u8,
    pub sv_service: u8,
    pub reserved1: u8,
    pub prc: i16,
    pub reserved2: u16,
    pub ic: i16,
}

impl Default for NavSBASSV {
    fn default() -> Self {
        NavSBASSV {
            svid: 0,
            flags: 0,
            udre: 0,
            sv_sys: 0,
            sv_service: 0,
            reserved1: 0,
            prc: 0,
            reserved2: 0,
            ic: 0,
        }
    }
}

impl ros2_client::Message for NavSBASSV {}
