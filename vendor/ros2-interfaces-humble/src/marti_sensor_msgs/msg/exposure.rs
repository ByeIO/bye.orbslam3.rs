use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exposure {
    pub header: crate::std_msgs::msg::Header,
    pub value: u64,
}

impl Default for Exposure {
    fn default() -> Self {
        Exposure {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}

impl ros2_client::Message for Exposure {}
