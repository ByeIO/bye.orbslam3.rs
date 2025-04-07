use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiStatus {
    pub header: crate::std_msgs::msg::Header,
    pub drivers: Vec<crate::puma_motor_msgs::msg::Status>,
}

impl Default for MultiStatus {
    fn default() -> Self {
        MultiStatus {
            header: crate::std_msgs::msg::Header::default(),
            drivers: Vec::new(),
        }
    }
}

impl ros2_client::Message for MultiStatus {}
