use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockadeHeartbeat {
    pub statuses: Vec<crate::rmf_traffic_msgs::msg::BlockadeStatus>,
    pub has_gridlock: bool,
}

impl Default for BlockadeHeartbeat {
    fn default() -> Self {
        BlockadeHeartbeat {
            statuses: Vec::new(),
            has_gridlock: false,
        }
    }
}

impl ros2_client::Message for BlockadeHeartbeat {}
