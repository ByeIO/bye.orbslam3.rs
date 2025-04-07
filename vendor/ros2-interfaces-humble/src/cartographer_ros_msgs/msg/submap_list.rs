use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmapList {
    pub header: crate::std_msgs::msg::Header,
    pub submap: Vec<crate::cartographer_ros_msgs::msg::SubmapEntry>,
}

impl Default for SubmapList {
    fn default() -> Self {
        SubmapList {
            header: crate::std_msgs::msg::Header::default(),
            submap: Vec::new(),
        }
    }
}

impl ros2_client::Message for SubmapList {}
