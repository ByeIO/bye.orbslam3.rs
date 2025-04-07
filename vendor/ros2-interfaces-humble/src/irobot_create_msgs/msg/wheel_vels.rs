use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelVels {
    pub header: crate::std_msgs::msg::Header,
    pub velocity_left: f32,
    pub velocity_right: f32,
}

impl Default for WheelVels {
    fn default() -> Self {
        WheelVels {
            header: crate::std_msgs::msg::Header::default(),
            velocity_left: 0.0,
            velocity_right: 0.0,
        }
    }
}

impl ros2_client::Message for WheelVels {}
