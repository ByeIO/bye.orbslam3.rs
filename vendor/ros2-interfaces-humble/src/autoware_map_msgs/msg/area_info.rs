use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AreaInfo {
    pub center_x: f32,
    pub center_y: f32,
    pub radius: f32,
}

impl Default for AreaInfo {
    fn default() -> Self {
        AreaInfo {
            center_x: 0.0,
            center_y: 0.0,
            radius: 0.0,
        }
    }
}

impl ros2_client::Message for AreaInfo {}
