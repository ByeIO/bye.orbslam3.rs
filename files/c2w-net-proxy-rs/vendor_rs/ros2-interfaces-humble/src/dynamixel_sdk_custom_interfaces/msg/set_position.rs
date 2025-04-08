use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPosition {
    pub id: u8,
    pub position: i32,
}

impl Default for SetPosition {
    fn default() -> Self {
        SetPosition {
            id: 0,
            position: 0,
        }
    }
}

impl ros2_client::Message for SetPosition {}
