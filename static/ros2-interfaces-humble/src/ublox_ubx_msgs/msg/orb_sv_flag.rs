use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrbSVFlag {
    pub health: u8,
    pub visibility: u8,
}

impl OrbSVFlag {
    pub const HEALTH_UNKNOWN: u8 = 0;
    pub const HEALTH_HEALTHY: u8 = 1;
    pub const HEALTH_NOT_HEALTHY: u8 = 2;
    pub const VISIBILITY_UNKNOWN: u8 = 0;
    pub const VISIBILITY_BELOW_HORIZON: u8 = 1;
    pub const VISIBILITY_ABOVE_HORIZON: u8 = 2;
    pub const VISIBILITY_ABOVE_ELEVATION_MASK: u8 = 3;
}

impl Default for OrbSVFlag {
    fn default() -> Self {
        OrbSVFlag {
            health: 0,
            visibility: 0,
        }
    }
}

impl ros2_client::Message for OrbSVFlag {}
