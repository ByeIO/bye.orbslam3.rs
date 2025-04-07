use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sound {
    pub value: u8,
}

impl Sound {
    pub const OFF: u8 = 0;
    pub const ON: u8 = 1;
    pub const LOW_BATTERY: u8 = 2;
    pub const ERROR: u8 = 3;
    pub const BUTTON1: u8 = 4;
    pub const BUTTON2: u8 = 5;
}

impl Default for Sound {
    fn default() -> Self {
        Sound {
            value: 0,
        }
    }
}

impl ros2_client::Message for Sound {}
