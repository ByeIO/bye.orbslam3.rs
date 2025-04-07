use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldBoundary {
    pub header: crate::std_msgs::msg::Header,
    pub points: Vec<crate::geometry_msgs::msg::Point>,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for FieldBoundary {
    fn default() -> Self {
        FieldBoundary {
            header: crate::std_msgs::msg::Header::default(),
            points: Vec::new(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}

impl ros2_client::Message for FieldBoundary {}
