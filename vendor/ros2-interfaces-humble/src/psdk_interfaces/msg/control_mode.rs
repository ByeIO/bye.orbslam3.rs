use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlMode {
    pub header: crate::std_msgs::msg::Header,
    pub control_mode: u8,
    pub device_mode: u8,
    pub control_auth: u8,
}

impl Default for ControlMode {
    fn default() -> Self {
        ControlMode {
            header: crate::std_msgs::msg::Header::default(),
            control_mode: 0,
            device_mode: 0,
            control_auth: 0,
        }
    }
}

impl ros2_client::Message for ControlMode {}
