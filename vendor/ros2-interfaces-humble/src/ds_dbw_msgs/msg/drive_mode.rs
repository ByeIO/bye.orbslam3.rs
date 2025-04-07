use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveMode {
    pub value: u8,
}

impl DriveMode {
    pub const UNKNOWN: u8 = 0;
    pub const NORMAL: u8 = 1;
    pub const ECONOMY: u8 = 2;
    pub const COMFORT: u8 = 3;
    pub const SPORT: u8 = 4;
    pub const TOW_HAUL: u8 = 5;
    pub const SNOW: u8 = 6;
    pub const SAND: u8 = 7;
    pub const MUD: u8 = 8;
    pub const ROCK: u8 = 9;
    pub const BAJA: u8 = 10;
    pub const TRACK: u8 = 11;
}

impl Default for DriveMode {
    fn default() -> Self {
        DriveMode {
            value: 0,
        }
    }
}

impl ros2_client::Message for DriveMode {}
