use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Path {
    pub header: crate::std_msgs::msg::Header,
    pub poses: Vec<crate::geometry_msgs::msg::PoseStamped>,
}

impl Default for Path {
    fn default() -> Self {
        Path {
            header: crate::std_msgs::msg::Header::default(),
            poses: Vec::new(),
        }
    }
}

impl ros2_client::Message for Path {}
