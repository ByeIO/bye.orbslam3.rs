use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedPointOfInterest2D {
    pub x: f32,
    pub y: f32,
    pub c: f32,
}

impl Default for NormalizedPointOfInterest2D {
    fn default() -> Self {
        NormalizedPointOfInterest2D {
            x: 0.0,
            y: 0.0,
            c: 0.0,
        }
    }
}

impl ros2_client::Message for NormalizedPointOfInterest2D {}
