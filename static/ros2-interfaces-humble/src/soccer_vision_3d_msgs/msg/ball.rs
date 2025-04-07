use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ball {
    pub center: crate::geometry_msgs::msg::Point,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            center: crate::geometry_msgs::msg::Point::default(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}

impl ros2_client::Message for Ball {}
