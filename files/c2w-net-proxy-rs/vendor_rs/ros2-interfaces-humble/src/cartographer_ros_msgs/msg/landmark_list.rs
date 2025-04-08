use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandmarkList {
    pub header: crate::std_msgs::msg::Header,
    pub landmarks: Vec<crate::cartographer_ros_msgs::msg::LandmarkEntry>,
}

impl Default for LandmarkList {
    fn default() -> Self {
        LandmarkList {
            header: crate::std_msgs::msg::Header::default(),
            landmarks: Vec::new(),
        }
    }
}

impl ros2_client::Message for LandmarkList {}
