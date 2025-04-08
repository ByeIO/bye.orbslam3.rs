use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedObjectArray {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::marti_nav_msgs::msg::TrackedObject>,
}

impl Default for TrackedObjectArray {
    fn default() -> Self {
        TrackedObjectArray {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrackedObjectArray {}
