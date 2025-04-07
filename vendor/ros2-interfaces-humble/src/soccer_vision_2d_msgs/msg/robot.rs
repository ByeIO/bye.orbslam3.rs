use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Robot {
    pub bb: crate::vision_msgs::msg::BoundingBox2D,
    pub attributes: crate::soccer_vision_attribute_msgs::msg::Robot,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for Robot {
    fn default() -> Self {
        Robot {
            bb: crate::vision_msgs::msg::BoundingBox2D::default(),
            attributes: crate::soccer_vision_attribute_msgs::msg::Robot::default(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}

impl ros2_client::Message for Robot {}
