use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Byte {
    pub data: u8,
}

impl Default for Byte {
    fn default() -> Self {
        Byte {
            data: 0,
        }
    }
}

impl ros2_client::Message for Byte {}
