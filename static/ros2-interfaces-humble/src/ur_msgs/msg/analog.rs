use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Analog {
    pub pin: u8,
    pub domain: u8,
    pub state: f32,
}

impl Analog {
    pub const CURRENT: u8 = 0;
    pub const VOLTAGE: u8 = 1;
}

impl Default for Analog {
    fn default() -> Self {
        Analog {
            pin: 0,
            domain: 0,
            state: 0.0,
        }
    }
}

impl ros2_client::Message for Analog {}
