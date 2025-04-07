use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureOutputsPwmFreqCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub min_pwm_pct: u8,
    pub max_pwm_pct: u8,
    pub pwm_freq: u16,
}

impl Default for ConfigureOutputsPwmFreqCmd {
    fn default() -> Self {
        ConfigureOutputsPwmFreqCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            min_pwm_pct: 0,
            max_pwm_pct: 0,
            pwm_freq: 0,
        }
    }
}

impl ros2_client::Message for ConfigureOutputsPwmFreqCmd {}
