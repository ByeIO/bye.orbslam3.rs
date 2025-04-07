use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    pub ids: Vec<i32>,
    pub ids_confidence: Vec<f64>,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for Marker {
    fn default() -> Self {
        Marker {
            ids: Vec::new(),
            ids_confidence: Vec::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for Marker {}
