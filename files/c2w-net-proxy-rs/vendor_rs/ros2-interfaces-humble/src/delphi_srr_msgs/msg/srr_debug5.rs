use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrrDebug5 {
    pub header: crate::std_msgs::msg::Header,
    pub can_tx_align_updates: u16,
}

impl Default for SrrDebug5 {
    fn default() -> Self {
        SrrDebug5 {
            header: crate::std_msgs::msg::Header::default(),
            can_tx_align_updates: 0,
        }
    }
}

impl ros2_client::Message for SrrDebug5 {}
