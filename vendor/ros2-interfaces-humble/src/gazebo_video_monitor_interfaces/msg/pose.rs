use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl ros2_client::Message for Pose {}
