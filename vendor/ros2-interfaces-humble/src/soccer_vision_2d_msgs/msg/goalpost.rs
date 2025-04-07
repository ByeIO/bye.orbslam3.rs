use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goalpost {
    pub bb: crate::vision_msgs::msg::BoundingBox2D,
    pub attributes: crate::soccer_vision_attribute_msgs::msg::Goalpost,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for Goalpost {
    fn default() -> Self {
        Goalpost {
            bb: crate::vision_msgs::msg::BoundingBox2D::default(),
            attributes: crate::soccer_vision_attribute_msgs::msg::Goalpost::default(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}

impl ros2_client::Message for Goalpost {}
