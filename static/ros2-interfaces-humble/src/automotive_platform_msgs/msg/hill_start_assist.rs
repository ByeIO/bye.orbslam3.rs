use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HillStartAssist {
    pub header: crate::std_msgs::msg::Header,
    pub active: bool,
}

impl Default for HillStartAssist {
    fn default() -> Self {
        HillStartAssist {
            header: crate::std_msgs::msg::Header::default(),
            active: false,
        }
    }
}

impl ros2_client::Message for HillStartAssist {}
