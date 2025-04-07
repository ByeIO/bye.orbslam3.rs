use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Speed {
    pub header: crate::std_msgs::msg::Header,
    pub speed: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Speed {
            header: crate::std_msgs::msg::Header::default(),
            speed: 0.0,
        }
    }
}

impl ros2_client::Message for Speed {}
