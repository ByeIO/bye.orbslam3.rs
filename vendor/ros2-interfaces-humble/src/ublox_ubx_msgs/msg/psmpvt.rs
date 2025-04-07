use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PSMPVT {
    pub state: u8,
}

impl PSMPVT {
    pub const PSM_STATE_NOT_ACTIVE: u8 = 0;
    pub const PSM_STATE_ENABLED: u8 = 1;
    pub const PSM_STATE_ACQUISITION: u8 = 2;
    pub const PSM_STATE_TRACKING: u8 = 3;
    pub const PSM_STATE_POWER_OPTIMIZED_TRACKING: u8 = 4;
    pub const PSM_STATE_INACTIVE: u8 = 5;
}

impl Default for PSMPVT {
    fn default() -> Self {
        PSMPVT {
            state: 0,
        }
    }
}

impl ros2_client::Message for PSMPVT {}
