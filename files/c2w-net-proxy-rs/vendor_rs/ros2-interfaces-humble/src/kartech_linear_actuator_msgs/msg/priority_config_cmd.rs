use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriorityConfigCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub handshake_priority: u8,
    pub auto_reply_priority: u8,
    pub scheduled_priority: u8,
    pub polled_priority: u8,
}

impl Default for PriorityConfigCmd {
    fn default() -> Self {
        PriorityConfigCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            handshake_priority: 0,
            auto_reply_priority: 0,
            scheduled_priority: 0,
            polled_priority: 0,
        }
    }
}

impl ros2_client::Message for PriorityConfigCmd {}
