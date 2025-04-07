use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseArray {
    pub header: crate::std_msgs::msg::Header,
    pub poses: Vec<crate::geometry_msgs::msg::Pose>,
}

impl Default for PoseArray {
    fn default() -> Self {
        PoseArray {
            header: crate::std_msgs::msg::Header::default(),
            poses: Vec::new(),
        }
    }
}

impl ros2_client::Message for PoseArray {}
