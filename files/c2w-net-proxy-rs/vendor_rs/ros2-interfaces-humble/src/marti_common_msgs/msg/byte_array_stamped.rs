use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByteArrayStamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: Vec<u8>,
}

impl Default for ByteArrayStamped {
    fn default() -> Self {
        ByteArrayStamped {
            header: crate::std_msgs::msg::Header::default(),
            value: Vec::new(),
        }
    }
}

impl ros2_client::Message for ByteArrayStamped {}
