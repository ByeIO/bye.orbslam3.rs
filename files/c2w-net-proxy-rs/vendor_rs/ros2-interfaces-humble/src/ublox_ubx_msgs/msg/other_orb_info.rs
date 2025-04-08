use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherOrbInfo {
    pub ano_aop_usability: u8,
    pub orb_type: u8,
}

impl OtherOrbInfo {
    pub const ANO_AOP_USABILITY_UNKNOWN: u8 = 31;
    pub const ANO_AOP_USABILITY_OVER_30_DAYS: u8 = 30;
    pub const ANO_AOP_USABILITY_EXPIRED: u8 = 0;
    pub const TYPE_NO_ORBIT_DATA: u8 = 0;
    pub const TYPE_ASSISTNOW_OFFLINE: u8 = 1;
    pub const TYPE_ASSISTNOW_AUTONOMOUS: u8 = 2;
}

impl Default for OtherOrbInfo {
    fn default() -> Self {
        OtherOrbInfo {
            ano_aop_usability: 0,
            orb_type: 0,
        }
    }
}

impl ros2_client::Message for OtherOrbInfo {}
