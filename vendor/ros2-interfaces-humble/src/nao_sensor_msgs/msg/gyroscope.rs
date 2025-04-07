use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gyroscope {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Gyroscope {
    fn default() -> Self {
        Gyroscope {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ros2_client::Message for Gyroscope {}
