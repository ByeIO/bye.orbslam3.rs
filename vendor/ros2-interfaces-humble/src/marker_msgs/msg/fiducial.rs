use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fiducial {
    pub ids: Vec<i32>,
    pub ids_confidence: Vec<f64>,
    pub object_points: Vec<crate::geometry_msgs::msg::Point>,
    pub image_points: Vec<crate::geometry_msgs::msg::Point>,
}

impl Default for Fiducial {
    fn default() -> Self {
        Fiducial {
            ids: Vec::new(),
            ids_confidence: Vec::new(),
            object_points: Vec::new(),
            image_points: Vec::new(),
        }
    }
}

impl ros2_client::Message for Fiducial {}
