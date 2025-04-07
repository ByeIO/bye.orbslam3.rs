use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathVec {
    pub header: crate::std_msgs::msg::Header,
    pub paths: Vec<crate::nav_msgs::msg::Path>,
}

impl Default for PathVec {
    fn default() -> Self {
        PathVec {
            header: crate::std_msgs::msg::Header::default(),
            paths: Vec::new(),
        }
    }
}

impl ros2_client::Message for PathVec {}
