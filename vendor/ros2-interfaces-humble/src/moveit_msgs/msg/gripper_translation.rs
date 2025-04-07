use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GripperTranslation {
    pub direction: crate::geometry_msgs::msg::Vector3Stamped,
    pub desired_distance: f32,
    pub min_distance: f32,
}

impl Default for GripperTranslation {
    fn default() -> Self {
        GripperTranslation {
            direction: crate::geometry_msgs::msg::Vector3Stamped::default(),
            desired_distance: 0.0,
            min_distance: 0.0,
        }
    }
}

impl ros2_client::Message for GripperTranslation {}
