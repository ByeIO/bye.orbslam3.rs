use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UVCoordinate {
    pub u: f32,
    pub v: f32,
}

impl Default for UVCoordinate {
    fn default() -> Self {
        UVCoordinate {
            u: 0.0,
            v: 0.0,
        }
    }
}

impl ros2_client::Message for UVCoordinate {}
