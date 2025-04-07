use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COData {
    pub index: u16,
    pub subindex: u8,
    pub data: u32,
}

impl Default for COData {
    fn default() -> Self {
        COData {
            index: 0,
            subindex: 0,
            data: 0,
        }
    }
}

impl ros2_client::Message for COData {}
