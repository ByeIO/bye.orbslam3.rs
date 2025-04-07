use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectionFlag {
    pub flag: u8,
}

impl DetectionFlag {
    pub const DETECT_OK: u8 = 0;
    pub const DETECT_BAD_SONAR: u8 = 1;
    pub const DETECT_BAD_FILTER: u8 = 2;
    pub const DETECT_BAD_USER: u8 = 4;
}

impl Default for DetectionFlag {
    fn default() -> Self {
        DetectionFlag {
            flag: 0,
        }
    }
}

impl ros2_client::Message for DetectionFlag {}
