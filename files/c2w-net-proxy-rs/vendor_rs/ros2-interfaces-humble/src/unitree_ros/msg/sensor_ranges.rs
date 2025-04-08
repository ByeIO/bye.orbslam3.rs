use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorRanges {
    pub front: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for SensorRanges {
    fn default() -> Self {
        SensorRanges {
            front: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }
}

impl ros2_client::Message for SensorRanges {}
