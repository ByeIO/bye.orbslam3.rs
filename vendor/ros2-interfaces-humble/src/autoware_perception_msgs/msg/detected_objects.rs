use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectedObjects {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::autoware_perception_msgs::msg::DetectedObject>,
}

impl Default for DetectedObjects {
    fn default() -> Self {
        DetectedObjects {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}

impl ros2_client::Message for DetectedObjects {}
