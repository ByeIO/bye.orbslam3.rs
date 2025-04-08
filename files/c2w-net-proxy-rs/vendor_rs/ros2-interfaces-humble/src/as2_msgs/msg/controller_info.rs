use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControllerInfo {
    pub header: crate::std_msgs::msg::Header,
    pub input_control_mode: crate::as2_msgs::msg::ControlMode,
    pub output_control_mode: crate::as2_msgs::msg::ControlMode,
}

impl Default for ControllerInfo {
    fn default() -> Self {
        ControllerInfo {
            header: crate::std_msgs::msg::Header::default(),
            input_control_mode: crate::as2_msgs::msg::ControlMode::default(),
            output_control_mode: crate::as2_msgs::msg::ControlMode::default(),
        }
    }
}

impl ros2_client::Message for ControllerInfo {}
