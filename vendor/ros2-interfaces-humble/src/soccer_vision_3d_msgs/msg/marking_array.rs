use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkingArray {
    pub header: crate::std_msgs::msg::Header,
    pub ellipses: Vec<crate::soccer_vision_3d_msgs::msg::MarkingEllipse>,
    pub intersections: Vec<crate::soccer_vision_3d_msgs::msg::MarkingIntersection>,
    pub segments: Vec<crate::soccer_vision_3d_msgs::msg::MarkingSegment>,
}

impl Default for MarkingArray {
    fn default() -> Self {
        MarkingArray {
            header: crate::std_msgs::msg::Header::default(),
            ellipses: Vec::new(),
            intersections: Vec::new(),
            segments: Vec::new(),
        }
    }
}

impl ros2_client::Message for MarkingArray {}
