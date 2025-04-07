use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsfRAW {
    pub reserved0: [u8; 4],
    pub blocks: Vec<crate::ublox_msgs::msg::EsfRAWBlock>,
}

impl EsfRAW {
    pub const CLASS_ID: u8 = 16;
    pub const MESSAGE_ID: u8 = 3;
}

impl Default for EsfRAW {
    fn default() -> Self {
        EsfRAW {
            reserved0: [0; 4],
            blocks: Vec::new(),
        }
    }
}

impl ros2_client::Message for EsfRAW {}
