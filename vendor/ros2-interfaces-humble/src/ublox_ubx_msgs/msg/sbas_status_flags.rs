use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SBASStatusFlags {
    pub integrity_used: u8,
}

impl SBASStatusFlags {
    pub const INTEGRITY_UNKNOWN: u8 = 0;
    pub const INTEGRITY_NOT_AVAILABLE: u8 = 1;
    pub const INTEGRITY_USED: u8 = 2;
}

impl Default for SBASStatusFlags {
    fn default() -> Self {
        SBASStatusFlags {
            integrity_used: 0,
        }
    }
}

impl ros2_client::Message for SBASStatusFlags {}
