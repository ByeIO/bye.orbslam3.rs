use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavDGPSSV {
    pub svid: u8,
    pub flags: u8,
    pub age_c: u16,
    pub prc: f32,
    pub prrc: f32,
}

impl NavDGPSSV {
    pub const FLAGS_CHANNEL_MASK: u8 = 15;
    pub const FLAGS_DGPS: u8 = 16;
}

impl Default for NavDGPSSV {
    fn default() -> Self {
        NavDGPSSV {
            svid: 0,
            flags: 0,
            age_c: 0,
            prc: 0.0,
            prrc: 0.0,
        }
    }
}

impl ros2_client::Message for NavDGPSSV {}
