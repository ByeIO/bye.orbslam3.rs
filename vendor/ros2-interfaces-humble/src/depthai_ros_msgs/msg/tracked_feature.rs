use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedFeature {
    pub header: crate::std_msgs::msg::Header,
    pub position: crate::geometry_msgs::msg::Point,
    pub id: u32,
    pub age: u32,
    pub harris_score: f32,
    pub tracking_error: f32,
}

impl Default for TrackedFeature {
    fn default() -> Self {
        TrackedFeature {
            header: crate::std_msgs::msg::Header::default(),
            position: crate::geometry_msgs::msg::Point::default(),
            id: 0,
            age: 0,
            harris_score: 0.0,
            tracking_error: 0.0,
        }
    }
}

impl ros2_client::Message for TrackedFeature {}
