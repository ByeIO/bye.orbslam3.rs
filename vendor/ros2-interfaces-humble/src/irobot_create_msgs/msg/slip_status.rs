use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlipStatus {
    pub header: crate::std_msgs::msg::Header,
    pub is_slipping: bool,
}

impl Default for SlipStatus {
    fn default() -> Self {
        SlipStatus {
            header: crate::std_msgs::msg::Header::default(),
            is_slipping: false,
        }
    }
}

impl ros2_client::Message for SlipStatus {}
