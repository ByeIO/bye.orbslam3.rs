use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlindSpotIndicators {
    pub header: crate::std_msgs::msg::Header,
    pub left: bool,
    pub right: bool,
}

impl Default for BlindSpotIndicators {
    fn default() -> Self {
        BlindSpotIndicators {
            header: crate::std_msgs::msg::Header::default(),
            left: false,
            right: false,
        }
    }
}

impl ros2_client::Message for BlindSpotIndicators {}
