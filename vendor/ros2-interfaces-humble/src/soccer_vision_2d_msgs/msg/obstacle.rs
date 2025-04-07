use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Obstacle {
    pub bb: crate::vision_msgs::msg::BoundingBox2D,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for Obstacle {
    fn default() -> Self {
        Obstacle {
            bb: crate::vision_msgs::msg::BoundingBox2D::default(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}

impl ros2_client::Message for Obstacle {}
