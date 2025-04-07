use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    pub header: crate::std_msgs::msg::Header,
    pub id: u32,
    pub pose: crate::geometry_msgs::msg::PoseWithCovariance,
    pub confidence: f64,
}

impl Default for Marker {
    fn default() -> Self {
        Marker {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            pose: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            confidence: 0.0,
        }
    }
}

impl ros2_client::Message for Marker {}
