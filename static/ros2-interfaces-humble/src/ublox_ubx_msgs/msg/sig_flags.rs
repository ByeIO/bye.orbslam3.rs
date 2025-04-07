use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SigFlags {
    pub health: u8,
    pub pr_smoothed: bool,
    pub pr_used: bool,
    pub cr_used: bool,
    pub do_used: bool,
    pub pr_corr_used: bool,
    pub cr_corr_used: bool,
    pub do_corr_used: bool,
}

impl SigFlags {
    pub const HEALTH_UNKNOWN: u8 = 0;
    pub const HEALTH_HEALTHY: u8 = 1;
    pub const HEALTH_UNHEALTHY: u8 = 2;
}

impl Default for SigFlags {
    fn default() -> Self {
        SigFlags {
            health: 0,
            pr_smoothed: false,
            pr_used: false,
            cr_used: false,
            do_used: false,
            pr_corr_used: false,
            cr_corr_used: false,
            do_corr_used: false,
        }
    }
}

impl ros2_client::Message for SigFlags {}
