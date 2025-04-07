use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectEcho {
    pub header: crate::std_msgs::msg::Header,
    pub id: u8,
    pub first: crate::off_highway_uss_msgs::msg::Echo,
    pub first_filtered: crate::off_highway_uss_msgs::msg::Echo,
    pub second: crate::off_highway_uss_msgs::msg::Echo,
}

impl Default for DirectEcho {
    fn default() -> Self {
        DirectEcho {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            first: crate::off_highway_uss_msgs::msg::Echo::default(),
            first_filtered: crate::off_highway_uss_msgs::msg::Echo::default(),
            second: crate::off_highway_uss_msgs::msg::Echo::default(),
        }
    }
}

impl ros2_client::Message for DirectEcho {}
