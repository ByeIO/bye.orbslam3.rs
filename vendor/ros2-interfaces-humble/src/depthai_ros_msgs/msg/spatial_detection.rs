use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialDetection {
    pub results: Vec<crate::vision_msgs::msg::ObjectHypothesis>,
    pub bbox: crate::vision_msgs::msg::BoundingBox2D,
    pub position: crate::geometry_msgs::msg::Point,
    pub is_tracking: bool,
    pub tracking_id: ::std::string::String,
}

impl Default for SpatialDetection {
    fn default() -> Self {
        SpatialDetection {
            results: Vec::new(),
            bbox: crate::vision_msgs::msg::BoundingBox2D::default(),
            position: crate::geometry_msgs::msg::Point::default(),
            is_tracking: false,
            tracking_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpatialDetection {}
