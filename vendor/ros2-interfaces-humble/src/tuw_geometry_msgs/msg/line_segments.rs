use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineSegments {
    pub header: crate::std_msgs::msg::Header,
    pub segments: Vec<crate::tuw_geometry_msgs::msg::LineSegment>,
}

impl Default for LineSegments {
    fn default() -> Self {
        LineSegments {
            header: crate::std_msgs::msg::Header::default(),
            segments: Vec::new(),
        }
    }
}

impl ros2_client::Message for LineSegments {}
