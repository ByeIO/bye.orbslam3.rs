use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeftFootLed {
    pub color: crate::std_msgs::msg::ColorRGBA,
}

impl Default for LeftFootLed {
    fn default() -> Self {
        LeftFootLed {
            color: crate::std_msgs::msg::ColorRGBA::default(),
        }
    }
}

impl ros2_client::Message for LeftFootLed {}
