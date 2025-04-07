use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeedPedals {
    pub header: crate::std_msgs::msg::Header,
    pub mode: u16,
    pub throttle: f32,
    pub brake: f32,
}

impl Default for SpeedPedals {
    fn default() -> Self {
        SpeedPedals {
            header: crate::std_msgs::msg::Header::default(),
            mode: 0,
            throttle: 0.0,
            brake: 0.0,
        }
    }
}

impl ros2_client::Message for SpeedPedals {}
