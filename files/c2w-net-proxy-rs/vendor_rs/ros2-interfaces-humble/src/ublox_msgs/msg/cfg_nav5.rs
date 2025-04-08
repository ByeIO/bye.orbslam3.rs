use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgNAV5 {
    pub mask: u16,
    pub dyn_model: u8,
    pub fix_mode: u8,
    pub fixed_alt: i32,
    pub fixed_alt_var: u32,
    pub min_elev: i8,
    pub dr_limit: u8,
    pub p_dop: u16,
    pub t_dop: u16,
    pub p_acc: u16,
    pub t_acc: u16,
    pub static_hold_thresh: u8,
    pub dgnss_time_out: u8,
    pub cno_thresh_num_svs: u8,
    pub cno_thresh: u8,
    pub reserved1: [u8; 2],
    pub static_hold_max_dist: u16,
    pub utc_standard: u8,
    pub reserved2: [u8; 5],
}

impl CfgNAV5 {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 36;
    pub const MASK_DYN: u16 = 1;
    pub const MASK_MIN_EL: u16 = 2;
    pub const MASK_FIX_MODE: u16 = 4;
    pub const MASK_DR_LIM: u16 = 8;
    pub const MASK_POS_MASK: u16 = 16;
    pub const MASK_TIME_MASK: u16 = 32;
    pub const MASK_STATIC_HOLD_MASK: u16 = 64;
    pub const MASK_DGPS_MASK: u16 = 128;
    pub const MASK_CNO: u16 = 256;
    pub const MASK_UTC: u16 = 1024;
    pub const DYN_MODEL_PORTABLE: u8 = 0;
    pub const DYN_MODEL_STATIONARY: u8 = 2;
    pub const DYN_MODEL_PEDESTRIAN: u8 = 3;
    pub const DYN_MODEL_AUTOMOTIVE: u8 = 4;
    pub const DYN_MODEL_SEA: u8 = 5;
    pub const DYN_MODEL_AIRBORNE_1G: u8 = 6;
    pub const DYN_MODEL_AIRBORNE_2G: u8 = 7;
    pub const DYN_MODEL_AIRBORNE_4G: u8 = 8;
    pub const DYN_MODEL_WRIST_WATCH: u8 = 9;
    pub const FIX_MODE_2D_ONLY: u8 = 1;
    pub const FIX_MODE_3D_ONLY: u8 = 2;
    pub const FIX_MODE_AUTO: u8 = 3;
    pub const UTC_STANDARD_AUTOMATIC: u8 = 0;
    pub const UTC_STANDARD_GPS: u8 = 3;
    pub const UTC_STANDARD_GLONASS: u8 = 6;
    pub const UTC_STANDARD_BEIDOU: u8 = 7;
}

impl Default for CfgNAV5 {
    fn default() -> Self {
        CfgNAV5 {
            mask: 0,
            dyn_model: 0,
            fix_mode: 0,
            fixed_alt: 0,
            fixed_alt_var: 0,
            min_elev: 0,
            dr_limit: 0,
            p_dop: 0,
            t_dop: 0,
            p_acc: 0,
            t_acc: 0,
            static_hold_thresh: 0,
            dgnss_time_out: 0,
            cno_thresh_num_svs: 0,
            cno_thresh: 0,
            reserved1: [0; 2],
            static_hold_max_dist: 0,
            utc_standard: 0,
            reserved2: [0; 5],
        }
    }
}

impl ros2_client::Message for CfgNAV5 {}
