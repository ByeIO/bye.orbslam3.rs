use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Led {
    pub value: u8,
}

impl Led {
    pub const BLACK: u8 = 0;
    pub const GREEN: u8 = 1;
    pub const ORANGE: u8 = 2;
    pub const RED: u8 = 3;
}

impl Default for Led {
    fn default() -> Self {
        Led {
            value: 0,
        }
    }
}

impl ros2_client::Message for Led {}
