use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UInt8 {
    pub data: u8,
}

impl Default for UInt8 {
    fn default() -> Self {
        UInt8 {
            data: 0,
        }
    }
}

impl ros2_client::Message for UInt8 {}
