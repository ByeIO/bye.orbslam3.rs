use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RCOut {
    pub header: crate::std_msgs::msg::Header,
    pub channels: Vec<u16>,
}

impl Default for RCOut {
    fn default() -> Self {
        RCOut {
            header: crate::std_msgs::msg::Header::default(),
            channels: Vec::new(),
        }
    }
}

impl ros2_client::Message for RCOut {}
