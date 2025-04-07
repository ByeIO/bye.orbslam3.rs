use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalOutput {
    pub values: [bool; 4],
    pub mask: [bool; 4],
}

impl Default for DigitalOutput {
    fn default() -> Self {
        DigitalOutput {
            values: [false; 4],
            mask: [false; 4],
        }
    }
}

impl ros2_client::Message for DigitalOutput {}
