use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkingIntersection {
    pub center: crate::vision_msgs::msg::Point2D,
    pub num_rays: i32,
    pub heading_rays: Vec<f64>,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for MarkingIntersection {
    fn default() -> Self {
        MarkingIntersection {
            center: crate::vision_msgs::msg::Point2D::default(),
            num_rays: 0,
            heading_rays: Vec::new(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}

impl ros2_client::Message for MarkingIntersection {}
