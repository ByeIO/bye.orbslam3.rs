use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgGpsVelStatus {
    pub vel_status: u8,
    pub vel_type: u8,
}

impl Default for SbgGpsVelStatus {
    fn default() -> Self {
        SbgGpsVelStatus {
            vel_status: 0,
            vel_type: 0,
        }
    }
}

impl ros2_client::Message for SbgGpsVelStatus {}
