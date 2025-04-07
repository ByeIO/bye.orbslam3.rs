use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavSOL {
    pub i_tow: u32,
    pub f_tow: i32,
    pub week: i16,
    pub gps_fix: u8,
    pub flags: u8,
    pub ecef_x: i32,
    pub ecef_y: i32,
    pub ecef_z: i32,
    pub p_acc: u32,
    pub ecef_vx: i32,
    pub ecef_vy: i32,
    pub ecef_vz: i32,
    pub s_acc: u32,
    pub p_dop: u16,
    pub reserved1: u8,
    pub num_sv: u8,
    pub reserved2: u32,
}

impl NavSOL {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 6;
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
}

impl Default for NavSOL {
    fn default() -> Self {
        NavSOL {
            i_tow: 0,
            f_tow: 0,
            week: 0,
            gps_fix: 0,
            flags: 0,
            ecef_x: 0,
            ecef_y: 0,
            ecef_z: 0,
            p_acc: 0,
            ecef_vx: 0,
            ecef_vy: 0,
            ecef_vz: 0,
            s_acc: 0,
            p_dop: 0,
            reserved1: 0,
            num_sv: 0,
            reserved2: 0,
        }
    }
}

impl ros2_client::Message for NavSOL {}
