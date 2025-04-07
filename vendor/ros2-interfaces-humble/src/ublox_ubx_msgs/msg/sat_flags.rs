use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SatFlags {
    pub quality_ind: u8,
    pub sv_used: bool,
    pub health: u8,
    pub diff_corr: bool,
    pub smoothed: bool,
    pub orbit_source: u8,
    pub eph_avail: bool,
    pub alm_avail: bool,
    pub ano_avail: bool,
    pub aop_avail: bool,
    pub sbas_corr_used: bool,
    pub rtcm_corr_used: bool,
    pub slas_corr_used: bool,
    pub spartn_corr_used: bool,
    pub pr_corr_used: bool,
    pub cr_corr_used: bool,
    pub do_corr_used: bool,
    pub clas_corr_used: bool,
}

impl SatFlags {
    pub const QUALITY_NO_SIGNAL: u8 = 0;
    pub const QUALITY_SEARCHING: u8 = 1;
    pub const QUALITY_ACQUIRED: u8 = 2;
    pub const QUALITY_UNUSABLE: u8 = 3;
    pub const QUALITY_CODE_LOCKED: u8 = 4;
    pub const QUALITY_CARRIER_LOCKED: u8 = 5;
    pub const HEALTH_UNKNOWN: u8 = 0;
    pub const HEALTH_HEALTHY: u8 = 1;
    pub const HEALTH_UNHEALTHY: u8 = 2;
    pub const ORBIT_NO_INFO: u8 = 0;
    pub const ORBIT_EPH_USED: u8 = 1;
    pub const ORBIT_ALM_USED: u8 = 2;
    pub const ORBIT_ASSISTNOW_OFFLINE: u8 = 3;
    pub const ORBIT_ASSISTNOW_AUTONOMOUS: u8 = 4;
}

impl Default for SatFlags {
    fn default() -> Self {
        SatFlags {
            quality_ind: 0,
            sv_used: false,
            health: 0,
            diff_corr: false,
            smoothed: false,
            orbit_source: 0,
            eph_avail: false,
            alm_avail: false,
            ano_avail: false,
            aop_avail: false,
            sbas_corr_used: false,
            rtcm_corr_used: false,
            slas_corr_used: false,
            spartn_corr_used: false,
            pr_corr_used: false,
            cr_corr_used: false,
            do_corr_used: false,
            clas_corr_used: false,
        }
    }
}

impl ros2_client::Message for SatFlags {}
