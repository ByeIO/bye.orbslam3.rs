use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrottleFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub throttle_pedal: f32,
}

impl Default for ThrottleFeedback {
    fn default() -> Self {
        ThrottleFeedback {
            header: crate::std_msgs::msg::Header::default(),
            throttle_pedal: 0.0,
        }
    }
}

impl ros2_client::Message for ThrottleFeedback {}
