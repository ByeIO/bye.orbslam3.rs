use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgUtcTimeStatus {
    pub clock_stable: bool,
    pub clock_status: u8,
    pub clock_utc_sync: bool,
    pub clock_utc_status: u8,
}

impl Default for SbgUtcTimeStatus {
    fn default() -> Self {
        SbgUtcTimeStatus {
            clock_stable: false,
            clock_status: 0,
            clock_utc_sync: false,
            clock_utc_status: 0,
        }
    }
}

impl ros2_client::Message for SbgUtcTimeStatus {}
