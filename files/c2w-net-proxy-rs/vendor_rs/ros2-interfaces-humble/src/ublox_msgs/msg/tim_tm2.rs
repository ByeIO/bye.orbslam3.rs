use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimTM2 {
    pub ch: u8,
    pub flags: u8,
    pub rising_edge_count: u16,
    pub wn_r: u16,
    pub wn_f: u16,
    pub tow_ms_r: u32,
    pub tow_sub_ms_r: u32,
    pub tow_ms_f: u32,
    pub tow_sub_ms_f: u32,
    pub acc_est: u32,
}

impl TimTM2 {
    pub const CLASS_ID: u8 = 13;
    pub const MESSAGE_ID: u8 = 3;
    pub const FLAGS_MODE_RUNNING: u8 = 1;
    pub const FLAGS_RUN: u8 = 2;
    pub const FLAGS_NEWFALLINGEDGE: u8 = 4;
    pub const FLAGS_TIMEBASE_GNSS: u8 = 8;
    pub const FLAGS_TIMEBASE_UTC: u8 = 16;
    pub const FLAGS_UTC_AVAIL: u8 = 32;
    pub const FLAGS_TIME_VALID: u8 = 64;
    pub const FLAGS_NEWRISINGEDGE: u8 = 128;
}

impl Default for TimTM2 {
    fn default() -> Self {
        TimTM2 {
            ch: 0,
            flags: 0,
            rising_edge_count: 0,
            wn_r: 0,
            wn_f: 0,
            tow_ms_r: 0,
            tow_sub_ms_r: 0,
            tow_ms_f: 0,
            tow_sub_ms_f: 0,
            acc_est: 0,
        }
    }
}

impl ros2_client::Message for TimTM2 {}
