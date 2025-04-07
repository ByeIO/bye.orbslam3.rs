use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformStatus {
    pub state: i8,
}

impl PlatformStatus {
    pub const EMERGENCY: i8 = -1;
    pub const DISARMED: i8 = 0;
    pub const LANDED: i8 = 1;
    pub const TAKING_OFF: i8 = 2;
    pub const FLYING: i8 = 3;
    pub const LANDING: i8 = 4;
}

impl Default for PlatformStatus {
    fn default() -> Self {
        PlatformStatus {
            state: 0,
        }
    }
}

impl ros2_client::Message for PlatformStatus {}
