use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub header: crate::std_msgs::msg::Header,
    pub connected: bool,
    pub armed: bool,
    pub offboard: bool,
    pub status: crate::as2_msgs::msg::PlatformStatus,
    pub current_control_mode: crate::as2_msgs::msg::ControlMode,
}

impl Default for PlatformInfo {
    fn default() -> Self {
        PlatformInfo {
            header: crate::std_msgs::msg::Header::default(),
            connected: false,
            armed: false,
            offboard: false,
            status: crate::as2_msgs::msg::PlatformStatus::default(),
            current_control_mode: crate::as2_msgs::msg::ControlMode::default(),
        }
    }
}

impl ros2_client::Message for PlatformInfo {}
