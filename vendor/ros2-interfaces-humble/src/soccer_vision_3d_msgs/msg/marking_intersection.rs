use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkingIntersection {
    pub center: crate::geometry_msgs::msg::Point,
    pub num_rays: i32,
    pub rays: Vec<crate::geometry_msgs::msg::Vector3>,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for MarkingIntersection {
    fn default() -> Self {
        MarkingIntersection {
            center: crate::geometry_msgs::msg::Point::default(),
            num_rays: 0,
            rays: Vec::new(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}

impl ros2_client::Message for MarkingIntersection {}
