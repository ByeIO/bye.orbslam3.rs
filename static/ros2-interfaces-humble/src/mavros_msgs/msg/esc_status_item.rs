use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ESCStatusItem {
    pub header: crate::std_msgs::msg::Header,
    pub rpm: i32,
    pub voltage: f32,
    pub current: f32,
}

impl Default for ESCStatusItem {
    fn default() -> Self {
        ESCStatusItem {
            header: crate::std_msgs::msg::Header::default(),
            rpm: 0,
            voltage: 0.0,
            current: 0.0,
        }
    }
}

impl ros2_client::Message for ESCStatusItem {}
