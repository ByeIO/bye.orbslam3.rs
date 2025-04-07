use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpoofDet {
    pub state: u8,
}

impl SpoofDet {
    pub const SPOOF_DET_UNKNOWN: u8 = 0;
    pub const SPOOF_DET_NO_SPOOFING: u8 = 1;
    pub const SPOOF_DET_SPOOFING: u8 = 2;
    pub const SPOOF_DET_MULTIPLE_SPOOFING: u8 = 3;
}

impl Default for SpoofDet {
    fn default() -> Self {
        SpoofDet {
            state: 0,
        }
    }
}

impl ros2_client::Message for SpoofDet {}
