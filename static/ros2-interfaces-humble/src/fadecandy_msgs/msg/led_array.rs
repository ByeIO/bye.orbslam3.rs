use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LEDArray {
    pub strips: Vec<crate::fadecandy_msgs::msg::LEDStrip>,
}

impl Default for LEDArray {
    fn default() -> Self {
        LEDArray {
            strips: Vec::new(),
        }
    }
}

impl ros2_client::Message for LEDArray {}
