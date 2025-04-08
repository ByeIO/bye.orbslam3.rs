use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioData {
    pub data: Vec<u8>,
}

impl Default for AudioData {
    fn default() -> Self {
        AudioData {
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for AudioData {}
