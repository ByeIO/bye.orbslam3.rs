use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wiper {
    pub value: u8,
}

impl Wiper {
    pub const UNKNOWN: u8 = 0;
    pub const OFF: u8 = 1;
    pub const MOVING_OFF: u8 = 2;
    pub const MANUAL_OFF: u8 = 3;
    pub const MANUAL_ON: u8 = 4;
    pub const MANUAL_LOW: u8 = 5;
    pub const MANUAL_HIGH: u8 = 6;
    pub const AUTO_OFF: u8 = 7;
    pub const AUTO_LOW: u8 = 8;
    pub const AUTO_HIGH: u8 = 9;
    pub const AUTO_ADJUST: u8 = 10;
    pub const MIST_FLICK: u8 = 11;
    pub const WASH: u8 = 12;
    pub const COURTESY_WIPE: u8 = 13;
    pub const STALLED: u8 = 14;
}

impl Default for Wiper {
    fn default() -> Self {
        Wiper {
            value: 0,
        }
    }
}

impl ros2_client::Message for Wiper {}
