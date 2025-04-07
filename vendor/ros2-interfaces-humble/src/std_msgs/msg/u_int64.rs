use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UInt64 {
    pub data: u64,
}

impl Default for UInt64 {
    fn default() -> Self {
        UInt64 {
            data: 0,
        }
    }
}

impl ros2_client::Message for UInt64 {}
