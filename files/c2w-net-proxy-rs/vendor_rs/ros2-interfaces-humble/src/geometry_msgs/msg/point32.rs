use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Point32 {
    fn default() -> Self {
        Point32 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ros2_client::Message for Point32 {}
