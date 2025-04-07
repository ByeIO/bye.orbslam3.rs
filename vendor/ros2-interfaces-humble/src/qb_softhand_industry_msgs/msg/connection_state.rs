use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionState {
    pub device: crate::qb_softhand_industry_msgs::msg::DeviceConnectionInfo,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState {
            device: crate::qb_softhand_industry_msgs::msg::DeviceConnectionInfo::default(),
        }
    }
}

impl ros2_client::Message for ConnectionState {}
