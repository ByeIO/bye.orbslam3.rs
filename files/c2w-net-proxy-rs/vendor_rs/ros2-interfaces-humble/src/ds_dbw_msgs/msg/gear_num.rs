use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GearNum {
    pub value: u8,
}

impl GearNum {
    pub const UNKNOWN: u8 = 0;
    pub const DRIVE01: u8 = 1;
    pub const DRIVE02: u8 = 2;
    pub const DRIVE03: u8 = 3;
    pub const DRIVE04: u8 = 4;
    pub const DRIVE05: u8 = 5;
    pub const DRIVE06: u8 = 6;
    pub const DRIVE07: u8 = 7;
    pub const DRIVE08: u8 = 8;
    pub const DRIVE09: u8 = 9;
    pub const DRIVE10: u8 = 10;
    pub const NEUTRAL: u8 = 16;
    pub const REVERSE1: u8 = 17;
    pub const REVERSE2: u8 = 18;
    pub const PARK: u8 = 31;
}

impl Default for GearNum {
    fn default() -> Self {
        GearNum {
            value: 0,
        }
    }
}

impl ros2_client::Message for GearNum {}
