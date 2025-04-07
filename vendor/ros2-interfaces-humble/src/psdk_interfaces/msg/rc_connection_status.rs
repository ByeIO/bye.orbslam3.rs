use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RCConnectionStatus {
    pub header: crate::std_msgs::msg::Header,
    pub air_connection: u8,
    pub ground_connection: u8,
    pub app_connection: u8,
    pub air_or_ground_disconnected: u8,
}

impl Default for RCConnectionStatus {
    fn default() -> Self {
        RCConnectionStatus {
            header: crate::std_msgs::msg::Header::default(),
            air_connection: 0,
            ground_connection: 0,
            app_connection: 0,
            air_or_ground_disconnected: 0,
        }
    }
}

impl ros2_client::Message for RCConnectionStatus {}
