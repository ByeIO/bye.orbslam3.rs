use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Echo {
    pub amplitude: u8,
    pub distance: u16,
}

impl Echo {
    pub const AMPLITUDE_NO_OBJECT: u8 = 63;
    pub const AMPLITUDE_SNA: u8 = 64;
    pub const DISTANCE_SNA: u16 = 1023;
    pub const DISTANCE_SENSOR_FAULTED: u16 = 1022;
    pub const DISTANCE_SENSOR_BLIND: u16 = 1021;
    pub const DISTANCE_PRESENCE_DETECTION: u16 = 1002;
    pub const DISTANCE_NO_OBJECT: u16 = 1003;
}

impl Default for Echo {
    fn default() -> Self {
        Echo {
            amplitude: 0,
            distance: 0,
        }
    }
}

impl ros2_client::Message for Echo {}
