use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HnrPVT {
    pub i_tow: u32,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub valid: u8,
    pub nano: i32,
    pub gps_fix: u8,
    pub flags: u8,
    pub reserved0: [u8; 2],
    pub lon: i32,
    pub lat: i32,
    pub height: i32,
    pub h_msl: i32,
    pub g_speed: i32,
    pub speed: i32,
    pub head_mot: i32,
    pub head_veh: i32,
    pub h_acc: u32,
    pub v_acc: u32,
    pub s_acc: u32,
    pub head_acc: u32,
    pub reserved1: [u8; 4],
}

impl HnrPVT {
    pub const CLASS_ID: u8 = 40;
    pub const MESSAGE_ID: u8 = 0;
    pub const VALID_DATE: u8 = 1;
    pub const VALID_TIME: u8 = 2;
    pub const VALID_FULLY_RESOLVED: u8 = 4;
    pub const VALID_MAG: u8 = 8;
    pub const FIX_TYPE_NO_FIX: u8 = 0;
    pub const FIX_TYPE_DEAD_RECKONING_ONLY: u8 = 1;
    pub const FIX_TYPE_2D: u8 = 2;
    pub const FIX_TYPE_3D: u8 = 3;
    pub const FIX_TYPE_GPS_DEAD_RECKONING_COMBINED: u8 = 4;
    pub const FIX_TYPE_TIME_ONLY: u8 = 5;
    pub const FLAGS_GNSS_FIX_OK: u8 = 1;
    pub const FLAGS_DIFF_SOLN: u8 = 2;
    pub const FLAGS_WKN_SET: u8 = 4;
    pub const FLAGS_TOW_SET: u8 = 8;
    pub const FLAGS_HEAD_VEH_VALID: u8 = 32;
}

impl Default for HnrPVT {
    fn default() -> Self {
        HnrPVT {
            i_tow: 0,
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            min: 0,
            sec: 0,
            valid: 0,
            nano: 0,
            gps_fix: 0,
            flags: 0,
            reserved0: [0; 2],
            lon: 0,
            lat: 0,
            height: 0,
            h_msl: 0,
            g_speed: 0,
            speed: 0,
            head_mot: 0,
            head_veh: 0,
            h_acc: 0,
            v_acc: 0,
            s_acc: 0,
            head_acc: 0,
            reserved1: [0; 4],
        }
    }
}

impl ros2_client::Message for HnrPVT {}
