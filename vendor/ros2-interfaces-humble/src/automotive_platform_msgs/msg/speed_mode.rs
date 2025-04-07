use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeedMode {
    pub header: crate::std_msgs::msg::Header,
    pub mode: u16,
    pub speed: f32,
    pub acceleration_limit: f32,
    pub deceleration_limit: f32,
}

impl Default for SpeedMode {
    fn default() -> Self {
        SpeedMode {
            header: crate::std_msgs::msg::Header::default(),
            mode: 0,
            speed: 0.0,
            acceleration_limit: 0.0,
            deceleration_limit: 0.0,
        }
    }
}

impl ros2_client::Message for SpeedMode {}
