use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub auto_reply: bool,
    pub position: f64,
    pub clutch_enable: bool,
    pub motor_enable: bool,
}

impl Default for PositionCmd {
    fn default() -> Self {
        PositionCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            auto_reply: false,
            position: 0.0,
            clutch_enable: false,
            motor_enable: false,
        }
    }
}

impl ros2_client::Message for PositionCmd {}
