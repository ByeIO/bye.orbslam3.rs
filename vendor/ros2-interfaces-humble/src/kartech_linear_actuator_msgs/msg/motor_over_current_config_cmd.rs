use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotorOverCurrentConfigCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub over_current: u16,
}

impl Default for MotorOverCurrentConfigCmd {
    fn default() -> Self {
        MotorOverCurrentConfigCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            over_current: 0,
        }
    }
}

impl ros2_client::Message for MotorOverCurrentConfigCmd {}
