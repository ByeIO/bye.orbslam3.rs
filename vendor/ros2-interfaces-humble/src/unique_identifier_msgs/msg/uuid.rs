use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UUID {
    pub uuid: [u8; 16],
}

impl Default for UUID {
    fn default() -> Self {
        UUID {
            uuid: [0; 16],
        }
    }
}

impl ros2_client::Message for UUID {}
