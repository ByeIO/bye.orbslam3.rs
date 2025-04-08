use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ESCInfoItem {
    pub header: crate::std_msgs::msg::Header,
    pub failure_flags: u16,
    pub error_count: u32,
    pub temperature: i32,
}

impl Default for ESCInfoItem {
    fn default() -> Self {
        ESCInfoItem {
            header: crate::std_msgs::msg::Header::default(),
            failure_flags: 0,
            error_count: 0,
            temperature: 0,
        }
    }
}

impl ros2_client::Message for ESCInfoItem {}
