use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavRELPOSNED9 {
    pub version: u8,
    pub reserved1: u8,
    pub ref_station_id: u16,
    pub i_tow: u32,
    pub rel_pos_n: i32,
    pub rel_pos_e: i32,
    pub rel_pos_d: i32,
    pub rel_pos_length: i32,
    pub rel_pos_heading: i32,
    pub reserved2: [u8; 4],
    pub rel_pos_hpn: i8,
    pub rel_pos_hpe: i8,
    pub rel_pos_hpd: i8,
    pub rel_pos_hp_length: i8,
    pub acc_n: u32,
    pub acc_e: u32,
    pub acc_d: u32,
    pub acc_length: u32,
    pub acc_heading: u32,
    pub reserved3: [u8; 4],
    pub flags: u32,
}

impl NavRELPOSNED9 {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 60;
    pub const FLAGS_GNSS_FIX_OK: u32 = 1;
    pub const FLAGS_DIFF_SOLN: u32 = 2;
    pub const FLAGS_REL_POS_VALID: u32 = 4;
    pub const FLAGS_CARR_SOLN_MASK: u32 = 24;
    pub const FLAGS_CARR_SOLN_NONE: u32 = 0;
    pub const FLAGS_CARR_SOLN_FLOAT: u32 = 8;
    pub const FLAGS_CARR_SOLN_FIXED: u32 = 16;
    pub const FLAGS_IS_MOVING: u32 = 32;
    pub const FLAGS_REF_POS_MISS: u32 = 64;
    pub const FLAGS_REF_OBS_MISS: u32 = 128;
    pub const FLAGS_REL_POS_HEAD_VALID: u32 = 256;
    pub const FLAGS_REL_POS_NORM: u32 = 512;
}

impl Default for NavRELPOSNED9 {
    fn default() -> Self {
        NavRELPOSNED9 {
            version: 0,
            reserved1: 0,
            ref_station_id: 0,
            i_tow: 0,
            rel_pos_n: 0,
            rel_pos_e: 0,
            rel_pos_d: 0,
            rel_pos_length: 0,
            rel_pos_heading: 0,
            reserved2: [0; 4],
            rel_pos_hpn: 0,
            rel_pos_hpe: 0,
            rel_pos_hpd: 0,
            rel_pos_hp_length: 0,
            acc_n: 0,
            acc_e: 0,
            acc_d: 0,
            acc_length: 0,
            acc_heading: 0,
            reserved3: [0; 4],
            flags: 0,
        }
    }
}

impl ros2_client::Message for NavRELPOSNED9 {}
