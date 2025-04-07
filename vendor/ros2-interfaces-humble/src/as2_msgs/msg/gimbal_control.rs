use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalControl {
    pub control_mode: u8,
    pub target: crate::geometry_msgs::msg::Vector3Stamped,
}

impl GimbalControl {
    pub const POSITION_MODE: u8 = 0;
    pub const SPEED_MODE: u8 = 1;
}

impl Default for GimbalControl {
    fn default() -> Self {
        GimbalControl {
            control_mode: 0,
            target: crate::geometry_msgs::msg::Vector3Stamped::default(),
        }
    }
}

impl ros2_client::Message for GimbalControl {}
