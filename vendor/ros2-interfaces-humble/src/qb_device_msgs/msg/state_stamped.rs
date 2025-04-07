use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateStamped {
    pub header: crate::std_msgs::msg::Header,
    pub device_info: crate::qb_device_msgs::msg::Info,
    pub device_data: crate::qb_device_msgs::msg::State,
}

impl Default for StateStamped {
    fn default() -> Self {
        StateStamped {
            header: crate::std_msgs::msg::Header::default(),
            device_info: crate::qb_device_msgs::msg::Info::default(),
            device_data: crate::qb_device_msgs::msg::State::default(),
        }
    }
}

impl ros2_client::Message for StateStamped {}
