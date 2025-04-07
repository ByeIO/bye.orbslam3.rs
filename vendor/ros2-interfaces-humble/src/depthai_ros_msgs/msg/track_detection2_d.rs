use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackDetection2D {
    pub results: Vec<crate::vision_msgs::msg::ObjectHypothesisWithPose>,
    pub bbox: crate::vision_msgs::msg::BoundingBox2D,
    pub is_tracking: bool,
    pub tracking_id: ::std::string::String,
    pub tracking_age: i32,
    pub tracking_status: i32,
}

impl Default for TrackDetection2D {
    fn default() -> Self {
        TrackDetection2D {
            results: Vec::new(),
            bbox: crate::vision_msgs::msg::BoundingBox2D::default(),
            is_tracking: false,
            tracking_id: ::std::string::String::new(),
            tracking_age: 0,
            tracking_status: 0,
        }
    }
}

impl ros2_client::Message for TrackDetection2D {}
