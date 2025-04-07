use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelStatus {
    pub header: crate::std_msgs::msg::Header,
    pub current_ma_left: i16,
    pub current_ma_right: i16,
    pub pwm_left: i16,
    pub pwm_right: i16,
    pub wheels_enabled: bool,
}

impl Default for WheelStatus {
    fn default() -> Self {
        WheelStatus {
            header: crate::std_msgs::msg::Header::default(),
            current_ma_left: 0,
            current_ma_right: 0,
            pwm_left: 0,
            pwm_right: 0,
            wheels_enabled: false,
        }
    }
}

impl ros2_client::Message for WheelStatus {}
