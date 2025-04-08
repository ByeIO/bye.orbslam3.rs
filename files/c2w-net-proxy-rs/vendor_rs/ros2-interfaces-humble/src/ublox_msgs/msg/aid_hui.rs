use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AidHUI {
    pub health: u32,
    pub utc_a0: f64,
    pub utc_a1: f64,
    pub utc_tow: i32,
    pub utc_wnt: i16,
    pub utc_ls: i16,
    pub utc_wnf: i16,
    pub utc_dn: i16,
    pub utc_lsf: i16,
    pub utc_spare: i16,
    pub klob_a0: f32,
    pub klob_a1: f32,
    pub klob_a2: f32,
    pub klob_a3: f32,
    pub klob_b0: f32,
    pub klob_b1: f32,
    pub klob_b2: f32,
    pub klob_b3: f32,
    pub flags: u32,
}

impl AidHUI {
    pub const CLASS_ID: u8 = 11;
    pub const MESSAGE_ID: u8 = 2;
    pub const FLAGS_HEALTH: u32 = 1;
    pub const FLAGS_UTC: u32 = 2;
    pub const FLAGS_KLOB: u32 = 4;
}

impl Default for AidHUI {
    fn default() -> Self {
        AidHUI {
            health: 0,
            utc_a0: 0.0,
            utc_a1: 0.0,
            utc_tow: 0,
            utc_wnt: 0,
            utc_ls: 0,
            utc_wnf: 0,
            utc_dn: 0,
            utc_lsf: 0,
            utc_spare: 0,
            klob_a0: 0.0,
            klob_a1: 0.0,
            klob_a2: 0.0,
            klob_a3: 0.0,
            klob_b0: 0.0,
            klob_b1: 0.0,
            klob_b2: 0.0,
            klob_b3: 0.0,
            flags: 0,
        }
    }
}

impl ros2_client::Message for AidHUI {}
