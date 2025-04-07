use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolutionInfo {
    pub resolution_start_angle: f32,
    pub resolution: f32,
}

impl Default for ResolutionInfo {
    fn default() -> Self {
        ResolutionInfo {
            resolution_start_angle: 0.0,
            resolution: 0.0,
        }
    }
}

impl ros2_client::Message for ResolutionInfo {}
