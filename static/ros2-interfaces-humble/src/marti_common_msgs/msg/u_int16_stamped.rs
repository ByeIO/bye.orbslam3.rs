use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UInt16Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: u16,
}

impl Default for UInt16Stamped {
    fn default() -> Self {
        UInt16Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}

impl ros2_client::Message for UInt16Stamped {}
