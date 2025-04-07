use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spherical {
    pub r: f32,
    pub phi: f32,
    pub theta: f32,
}

impl Default for Spherical {
    fn default() -> Self {
        Spherical {
            r: 0.0,
            phi: 0.0,
            theta: 0.0,
        }
    }
}

impl ros2_client::Message for Spherical {}
