use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavPVT {
    pub i_tow: u32,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub valid: u8,
    pub t_acc: u32,
    pub nano: i32,
    pub fix_type: u8,
    pub flags: u8,
    pub flags2: u8,
    pub num_sv: u8,
    pub lon: i32,
    pub lat: i32,
    pub height: i32,
    pub h_msl: i32,
    pub h_acc: u32,
    pub v_acc: u32,
    pub vel_n: i32,
    pub vel_e: i32,
    pub vel_d: i32,
    pub g_speed: i32,
    pub heading: i32,
    pub s_acc: u32,
    pub head_acc: u32,
    pub p_dop: u16,
    pub reserved1: [u8; 6],
    pub head_veh: i32,
    pub mag_dec: i16,
    pub mag_acc: u16,
}

impl NavPVT {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 7;
    pub const VALID_DATE: u8 = 1;
    pub const VALID_TIME: u8 = 2;
    pub const VALID_FULLY_RESOLVED: u8 = 4;
    pub const VALID_MAG: u8 = 8;
    pub const FIX_TYPE_NO_FIX: u8 = 0;
    pub const FIX_TYPE_DEAD_RECKONING_ONLY: u8 = 1;
    pub const FIX_TYPE_2D: u8 = 2;
    pub const FIX_TYPE_3D: u8 = 3;
    pub const FIX_TYPE_GNSS_DEAD_RECKONING_COMBINED: u8 = 4;
    pub const FIX_TYPE_TIME_ONLY: u8 = 5;
    pub const FLAGS_GNSS_FIX_OK: u8 = 1;
    pub const FLAGS_DIFF_SOLN: u8 = 2;
    pub const FLAGS_PSM_MASK: u8 = 28;
    pub const PSM_OFF: u8 = 0;
    pub const PSM_ENABLED: u8 = 4;
    pub const PSM_ACQUIRED: u8 = 8;
    pub const PSM_TRACKING: u8 = 12;
    pub const PSM_POWER_OPTIMIZED_TRACKING: u8 = 16;
    pub const PSM_INACTIVE: u8 = 20;
    pub const FLAGS_HEAD_VEH_VALID: u8 = 32;
    pub const FLAGS_CARRIER_PHASE_MASK: u8 = 192;
    pub const CARRIER_PHASE_NO_SOLUTION: u8 = 0;
    pub const CARRIER_PHASE_FLOAT: u8 = 64;
    pub const CARRIER_PHASE_FIXED: u8 = 128;
    pub const FLAGS2_CONFIRMED_AVAILABLE: u8 = 32;
    pub const FLAGS2_CONFIRMED_DATE: u8 = 64;
    pub const FLAGS2_CONFIRMED_TIME: u8 = 128;
}

impl Default for NavPVT {
    fn default() -> Self {
        NavPVT {
            i_tow: 0,
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            min: 0,
            sec: 0,
            valid: 0,
            t_acc: 0,
            nano: 0,
            fix_type: 0,
            flags: 0,
            flags2: 0,
            num_sv: 0,
            lon: 0,
            lat: 0,
            height: 0,
            h_msl: 0,
            h_acc: 0,
            v_acc: 0,
            vel_n: 0,
            vel_e: 0,
            vel_d: 0,
            g_speed: 0,
            heading: 0,
            s_acc: 0,
            head_acc: 0,
            p_dop: 0,
            reserved1: [0; 6],
            head_veh: 0,
            mag_dec: 0,
            mag_acc: 0,
        }
    }
}

impl ros2_client::Message for NavPVT {}
