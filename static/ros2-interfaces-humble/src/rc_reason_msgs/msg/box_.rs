use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Box {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Box {
    fn default() -> Self {
        Box {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ros2_client::Message for Box {}
