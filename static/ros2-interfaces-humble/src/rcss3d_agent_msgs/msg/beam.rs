use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beam {
    pub x: f64,
    pub y: f64,
    pub rot: f64,
}

impl Default for Beam {
    fn default() -> Self {
        Beam {
            x: 0.0,
            y: 0.0,
            rot: 0.0,
        }
    }
}

impl ros2_client::Message for Beam {}
