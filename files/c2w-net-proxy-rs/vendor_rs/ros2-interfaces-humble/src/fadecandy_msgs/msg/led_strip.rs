use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LEDStrip {
    pub colors: Vec<crate::std_msgs::msg::ColorRGBA>,
}

impl Default for LEDStrip {
    fn default() -> Self {
        LEDStrip {
            colors: Vec::new(),
        }
    }
}

impl ros2_client::Message for LEDStrip {}
