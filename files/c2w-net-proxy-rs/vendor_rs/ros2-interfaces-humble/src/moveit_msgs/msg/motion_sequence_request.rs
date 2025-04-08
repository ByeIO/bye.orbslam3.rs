use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionSequenceRequest {
    pub items: Vec<crate::moveit_msgs::msg::MotionSequenceItem>,
}

impl Default for MotionSequenceRequest {
    fn default() -> Self {
        MotionSequenceRequest {
            items: Vec::new(),
        }
    }
}

impl ros2_client::Message for MotionSequenceRequest {}
