use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoolStamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: bool,
}

impl Default for BoolStamped {
    fn default() -> Self {
        BoolStamped {
            header: crate::std_msgs::msg::Header::default(),
            value: false,
        }
    }
}

impl ros2_client::Message for BoolStamped {}
