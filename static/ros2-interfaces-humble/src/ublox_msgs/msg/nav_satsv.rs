use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavSATSV {
    pub gnss_id: u8,
    pub sv_id: u8,
    pub cno: u8,
    pub elev: i8,
    pub azim: i16,
    pub pr_res: i16,
    pub flags: u32,
}

impl NavSATSV {
    pub const FLAGS_QUALITY_IND_MASK: u32 = 7;
    pub const QUALITY_IND_NO_SIGNAL: u8 = 0;
    pub const QUALITY_IND_SEARCHING_SIGNAL: u8 = 1;
    pub const QUALITY_IND_SIGNAL_ACQUIRED: u8 = 2;
    pub const QUALITY_IND_SIGNAL_DETECTED_BUT_UNUSABLE: u8 = 3;
    pub const QUALITY_IND_CODE_LOCKED_AND_TIME_SYNC: u8 = 4;
    pub const QUALITY_IND_CODE_AND_CARR_LOCK_AND_TIME_SYNC1: u8 = 5;
    pub const QUALITY_IND_CODE_AND_CARR_LOCK_AND_TIME_SYNC2: u8 = 6;
    pub const QUALITY_IND_CODE_AND_CARR_LOCK_AND_TIME_SYNC3: u8 = 7;
    pub const FLAGS_SV_USED: u32 = 8;
    pub const FLAGS_HEALTH_MASK: u32 = 48;
    pub const HEALTH_UNKNOWN: u32 = 0;
    pub const HEALTH_HEALTHY: u32 = 1;
    pub const HEALTH_UNHEALTHY: u32 = 2;
    pub const FLAGS_DIFF_CORR: u32 = 64;
    pub const FLAGS_SMOOTHED: u32 = 128;
    pub const FLAGS_ORBIT_SOURCE_MASK: u32 = 1792;
    pub const ORBIT_SOURCE_UNAVAILABLE: u32 = 0;
    pub const ORBIT_SOURCE_EPH: u32 = 256;
    pub const ORBIT_SOURCE_ALM: u32 = 512;
    pub const ORBIT_SOURCE_ASSIST_OFFLINE: u32 = 768;
    pub const ORBIT_SOURCE_ASSIST_AUTONOMOUS: u32 = 1024;
    pub const ORBIT_SOURCE_OTHER1: u32 = 1280;
    pub const ORBIT_SOURCE_OTHER2: u32 = 1536;
    pub const ORBIT_SOURCE_OTHER3: u32 = 1792;
    pub const FLAGS_EPH_AVAIL: u32 = 2048;
    pub const FLAGS_ALM_AVAIL: u32 = 4096;
    pub const FLAGS_ANO_AVAIL: u32 = 8192;
    pub const FLAGS_AOP_AVAIL: u32 = 16384;
    pub const FLAGS_SBAS_CORR_USED: u32 = 65536;
    pub const FLAGS_RTCM_CORR_USED: u32 = 131072;
    pub const FLAGS_PR_CORR_USED: u32 = 1048576;
    pub const FLAGS_CR_CORR_USED: u32 = 2097152;
    pub const FLAGS_DO_CORR_USED: u32 = 4194304;
}

impl Default for NavSATSV {
    fn default() -> Self {
        NavSATSV {
            gnss_id: 0,
            sv_id: 0,
            cno: 0,
            elev: 0,
            azim: 0,
            pr_res: 0,
            flags: 0,
        }
    }
}

impl ros2_client::Message for NavSATSV {}
