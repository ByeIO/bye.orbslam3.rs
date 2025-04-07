use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkingEllipse {
    pub diameter: f64,
    pub center: crate::geometry_msgs::msg::Pose,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for MarkingEllipse {
    fn default() -> Self {
        MarkingEllipse {
            diameter: 0.0,
            center: crate::geometry_msgs::msg::Pose::default(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}

impl ros2_client::Message for MarkingEllipse {}
