use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavSVINFOSV {
    pub chn: u8,
    pub svid: u8,
    pub flags: u8,
    pub quality: u8,
    pub cno: u8,
    pub elev: i8,
    pub azim: i16,
    pub pr_res: i32,
}

impl NavSVINFOSV {
    pub const FLAGS_SV_USED: u8 = 1;
    pub const FLAGS_DIFF_CORR: u8 = 2;
    pub const FLAGS_ORBIT_AVAIL: u8 = 4;
    pub const FLAGS_ORBIT_EPH: u8 = 8;
    pub const FLAGS_UNHEALTHY: u8 = 16;
    pub const FLAGS_ORBIT_ALM: u8 = 32;
    pub const FLAGS_ORBIT_AOP: u8 = 64;
    pub const FLAGS_SMOOTHED: u8 = 128;
    pub const QUALITY_IDLE: u8 = 0;
    pub const QUALITY_SEARCHING: u8 = 1;
    pub const QUALITY_ACQUIRED: u8 = 2;
    pub const QUALITY_DETECTED: u8 = 3;
    pub const QUALITY_CODE_LOCK: u8 = 4;
    pub const QUALITY_CODE_AND_CARRIER_LOCKED1: u8 = 5;
    pub const QUALITY_CODE_AND_CARRIER_LOCKED2: u8 = 6;
    pub const QUALITY_CODE_AND_CARRIER_LOCKED3: u8 = 7;
}

impl Default for NavSVINFOSV {
    fn default() -> Self {
        NavSVINFOSV {
            chn: 0,
            svid: 0,
            flags: 0,
            quality: 0,
            cno: 0,
            elev: 0,
            azim: 0,
            pr_res: 0,
        }
    }
}

impl ros2_client::Message for NavSVINFOSV {}
