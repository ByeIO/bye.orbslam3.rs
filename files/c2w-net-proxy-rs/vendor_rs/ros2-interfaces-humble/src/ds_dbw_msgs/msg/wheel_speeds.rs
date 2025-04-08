use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelSpeeds {
    pub header: crate::std_msgs::msg::Header,
    pub front_left: f32,
    pub front_right: f32,
    pub rear_left: f32,
    pub rear_right: f32,
}

impl Default for WheelSpeeds {
    fn default() -> Self {
        WheelSpeeds {
            header: crate::std_msgs::msg::Header::default(),
            front_left: 0.0,
            front_right: 0.0,
            rear_left: 0.0,
            rear_right: 0.0,
        }
    }
}

impl ros2_client::Message for WheelSpeeds {}
