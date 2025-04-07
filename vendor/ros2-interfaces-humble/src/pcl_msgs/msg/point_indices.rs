use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointIndices {
    pub header: crate::std_msgs::msg::Header,
    pub indices: Vec<i32>,
}

impl Default for PointIndices {
    fn default() -> Self {
        PointIndices {
            header: crate::std_msgs::msg::Header::default(),
            indices: Vec::new(),
        }
    }
}

impl ros2_client::Message for PointIndices {}
