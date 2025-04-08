use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AirskinPressures {
    pub header: crate::std_msgs::msg::Header,
    pub pressures: Vec<u32>,
}

impl Default for AirskinPressures {
    fn default() -> Self {
        AirskinPressures {
            header: crate::std_msgs::msg::Header::default(),
            pressures: Vec::new(),
        }
    }
}

impl ros2_client::Message for AirskinPressures {}
