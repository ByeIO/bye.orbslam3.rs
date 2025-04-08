use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AirskinColors {
    pub header: crate::std_msgs::msg::Header,
    pub idx: Vec<u16>,
    pub colors: Vec<crate::std_msgs::msg::ColorRGBA>,
}

impl Default for AirskinColors {
    fn default() -> Self {
        AirskinColors {
            header: crate::std_msgs::msg::Header::default(),
            idx: Vec::new(),
            colors: Vec::new(),
        }
    }
}

impl ros2_client::Message for AirskinColors {}
