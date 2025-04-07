use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajGenInfo {
    pub header: crate::std_msgs::msg::Header,
    pub node_status: crate::as2_msgs::msg::NodeStatus,
    pub active_status: u8,
}

impl TrajGenInfo {
    pub const WAITING: u8 = 0;
    pub const EVALUATING: u8 = 1;
    pub const STOPPED: u8 = 2;
}

impl Default for TrajGenInfo {
    fn default() -> Self {
        TrajGenInfo {
            header: crate::std_msgs::msg::Header::default(),
            node_status: crate::as2_msgs::msg::NodeStatus::default(),
            active_status: 0,
        }
    }
}

impl ros2_client::Message for TrajGenInfo {}
