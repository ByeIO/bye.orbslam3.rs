use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sound {
    pub value: u8,
}

impl Sound {
    pub const ON: u8 = 0;
    pub const OFF: u8 = 1;
    pub const RECHARGE: u8 = 2;
    pub const BUTTON: u8 = 3;
    pub const ERROR: u8 = 4;
    pub const CLEANINGSTART: u8 = 5;
    pub const CLEANINGEND: u8 = 6;
}

impl Default for Sound {
    fn default() -> Self {
        Sound {
            value: 0,
        }
    }
}

impl ros2_client::Message for Sound {}
