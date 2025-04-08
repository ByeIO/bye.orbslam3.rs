use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseStampedWithIDArray {
    pub poses: Vec<crate::as2_msgs::msg::PoseStampedWithID>,
}

impl Default for PoseStampedWithIDArray {
    fn default() -> Self {
        PoseStampedWithIDArray {
            poses: Vec::new(),
        }
    }
}

impl ros2_client::Message for PoseStampedWithIDArray {}
