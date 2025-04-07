use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vibration {
    pub header: crate::std_msgs::msg::Header,
    pub vibration: crate::geometry_msgs::msg::Vector3,
    pub clipping: [f32; 3],
}

impl Default for Vibration {
    fn default() -> Self {
        Vibration {
            header: crate::std_msgs::msg::Header::default(),
            vibration: crate::geometry_msgs::msg::Vector3::default(),
            clipping: [0.0; 3],
        }
    }
}

impl ros2_client::Message for Vibration {}
