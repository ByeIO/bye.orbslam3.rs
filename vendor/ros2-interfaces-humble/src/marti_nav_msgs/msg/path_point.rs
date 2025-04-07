use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathPoint {
    pub x: f64,
    pub y: f64,
    pub yaw: f32,
    pub speed: f32,
}

impl Default for PathPoint {
    fn default() -> Self {
        PathPoint {
            x: 0.0,
            y: 0.0,
            yaw: 0.0,
            speed: 0.0,
        }
    }
}

impl ros2_client::Message for PathPoint {}
