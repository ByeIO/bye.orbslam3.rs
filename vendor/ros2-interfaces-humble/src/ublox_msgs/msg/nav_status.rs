use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavSTATUS {
    pub i_tow: u32,
    pub gps_fix: u8,
    pub flags: u8,
    pub fix_stat: u8,
    pub flags2: u8,
    pub ttff: u32,
    pub msss: u32,
}

impl NavSTATUS {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 3;
    pub const GPS_NO_FIX: u8 = 0;
    pub const GPS_DEAD_RECKONING_ONLY: u8 = 1;
    pub const GPS_2D_FIX: u8 = 2;
    pub const GPS_3D_FIX: u8 = 3;
    pub const GPS_GPS_DEAD_RECKONING_COMBINED: u8 = 4;
    pub const GPS_TIME_ONLY_FIX: u8 = 5;
    pub const FLAGS_GPS_FIX_OK: u8 = 1;
    pub const FLAGS_DIFF_SOLN: u8 = 2;
    pub const FLAGS_WKNSET: u8 = 4;
    pub const FLAGS_TOWSET: u8 = 8;
    pub const FIX_STAT_DIFF_CORR_MASK: u8 = 1;
    pub const FIX_STAT_MAP_MATCHING_MASK: u8 = 192;
    pub const MAP_MATCHING_NONE: u8 = 0;
    pub const MAP_MATCHING_VALID: u8 = 64;
    pub const MAP_MATCHING_USED: u8 = 128;
    pub const MAP_MATCHING_DR: u8 = 192;
    pub const FLAGS2_PSM_STATE_MASK: u8 = 3;
    pub const PSM_STATE_ACQUISITION: u8 = 0;
    pub const PSM_STATE_TRACKING: u8 = 1;
    pub const PSM_STATE_POWER_OPTIMIZED_TRACKING: u8 = 2;
    pub const PSM_STATE_INACTIVE: u8 = 3;
    pub const FLAGS2_SPOOF_DET_STATE_MASK: u8 = 24;
    pub const SPOOF_DET_STATE_UNKNOWN: u8 = 0;
    pub const SPOOF_DET_STATE_NONE: u8 = 8;
    pub const SPOOF_DET_STATE_SPOOFING: u8 = 16;
    pub const SPOOF_DET_STATE_MULTIPLE: u8 = 24;
}

impl Default for NavSTATUS {
    fn default() -> Self {
        NavSTATUS {
            i_tow: 0,
            gps_fix: 0,
            flags: 0,
            fix_stat: 0,
            flags2: 0,
            ttff: 0,
            msss: 0,
        }
    }
}

impl ros2_client::Message for NavSTATUS {}
