use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gear {
    pub value: u8,
}

impl Gear {
    pub const NONE: u8 = 0;
    pub const PARK: u8 = 1;
    pub const REVERSE: u8 = 2;
    pub const NEUTRAL: u8 = 3;
    pub const DRIVE: u8 = 4;
    pub const LOW: u8 = 5;
    pub const MANUAL: u8 = 6;
    pub const SPORT: u8 = 7;
    pub const CALIBRATE: u8 = 15;
}

impl Default for Gear {
    fn default() -> Self {
        Gear {
            value: 0,
        }
    }
}

impl ros2_client::Message for Gear {}
