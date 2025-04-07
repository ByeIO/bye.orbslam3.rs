use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedFeatures {
    pub header: crate::std_msgs::msg::Header,
    pub features: Vec<crate::depthai_ros_msgs::msg::TrackedFeature>,
}

impl Default for TrackedFeatures {
    fn default() -> Self {
        TrackedFeatures {
            header: crate::std_msgs::msg::Header::default(),
            features: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrackedFeatures {}
