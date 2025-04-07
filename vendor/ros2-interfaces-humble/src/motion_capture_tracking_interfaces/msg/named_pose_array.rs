use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedPoseArray {
    pub header: crate::std_msgs::msg::Header,
    pub poses: Vec<crate::motion_capture_tracking_interfaces::msg::NamedPose>,
}

impl Default for NamedPoseArray {
    fn default() -> Self {
        NamedPoseArray {
            header: crate::std_msgs::msg::Header::default(),
            poses: Vec::new(),
        }
    }
}

impl ros2_client::Message for NamedPoseArray {}
