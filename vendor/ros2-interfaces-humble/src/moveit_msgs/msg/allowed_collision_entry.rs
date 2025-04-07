use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedCollisionEntry {
    pub enabled: Vec<bool>,
}

impl Default for AllowedCollisionEntry {
    fn default() -> Self {
        AllowedCollisionEntry {
            enabled: Vec::new(),
        }
    }
}

impl ros2_client::Message for AllowedCollisionEntry {}
