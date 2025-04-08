use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub header: crate::std_msgs::msg::Header,
    pub message: Vec<u8>,
}

impl Default for Message {
    fn default() -> Self {
        Message {
            header: crate::std_msgs::msg::Header::default(),
            message: Vec::new(),
        }
    }
}

impl ros2_client::Message for Message {}
