use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteerWheel {
    pub header: crate::std_msgs::msg::Header,
    pub mode: u16,
    pub angle: f32,
    pub angle_velocity: f32,
}

impl Default for SteerWheel {
    fn default() -> Self {
        SteerWheel {
            header: crate::std_msgs::msg::Header::default(),
            mode: 0,
            angle: 0.0,
            angle_velocity: 0.0,
        }
    }
}

impl ros2_client::Message for SteerWheel {}
