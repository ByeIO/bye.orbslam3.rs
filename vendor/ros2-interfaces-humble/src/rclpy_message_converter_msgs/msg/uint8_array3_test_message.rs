use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uint8Array3TestMessage {
    pub data: [u8; 3],
}

impl Default for Uint8Array3TestMessage {
    fn default() -> Self {
        Uint8Array3TestMessage {
            data: [0; 3],
        }
    }
}

impl ros2_client::Message for Uint8Array3TestMessage {}
