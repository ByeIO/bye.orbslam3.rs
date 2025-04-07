use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedObjects {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::autoware_perception_msgs::msg::TrackedObject>,
}

impl Default for TrackedObjects {
    fn default() -> Self {
        TrackedObjects {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrackedObjects {}
