use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UInt16 {
    pub data: u16,
}

impl Default for UInt16 {
    fn default() -> Self {
        UInt16 {
            data: 0,
        }
    }
}

impl ros2_client::Message for UInt16 {}
