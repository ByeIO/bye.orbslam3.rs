use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandmarkEntry {
    pub id: ::std::string::String,
    pub tracking_from_landmark_transform: crate::geometry_msgs::msg::Pose,
    pub translation_weight: f64,
    pub rotation_weight: f64,
}

impl Default for LandmarkEntry {
    fn default() -> Self {
        LandmarkEntry {
            id: ::std::string::String::new(),
            tracking_from_landmark_transform: crate::geometry_msgs::msg::Pose::default(),
            translation_weight: 0.0,
            rotation_weight: 0.0,
        }
    }
}

impl ros2_client::Message for LandmarkEntry {}
