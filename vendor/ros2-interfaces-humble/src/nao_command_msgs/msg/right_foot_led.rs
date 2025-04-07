use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RightFootLed {
    pub color: crate::std_msgs::msg::ColorRGBA,
}

impl Default for RightFootLed {
    fn default() -> Self {
        RightFootLed {
            color: crate::std_msgs::msg::ColorRGBA::default(),
        }
    }
}

impl ros2_client::Message for RightFootLed {}
