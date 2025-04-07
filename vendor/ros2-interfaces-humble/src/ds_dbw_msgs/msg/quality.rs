use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quality {
    pub value: u8,
}

impl Quality {
    pub const OK: u8 = 0;
    pub const PARTIAL: u8 = 1;
    pub const NO_DATA: u8 = 2;
    pub const FAULT: u8 = 3;
}

impl Default for Quality {
    fn default() -> Self {
        Quality {
            value: 0,
        }
    }
}

impl ros2_client::Message for Quality {}
