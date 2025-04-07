use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gear {
    pub gear: u8,
}

impl Gear {
    pub const NONE: u8 = 0;
    pub const PARK: u8 = 1;
    pub const REVERSE: u8 = 2;
    pub const NEUTRAL: u8 = 3;
    pub const DRIVE: u8 = 4;
    pub const LOW: u8 = 5;
}

impl Default for Gear {
    fn default() -> Self {
        Gear {
            gear: 0,
        }
    }
}

impl ros2_client::Message for Gear {}
