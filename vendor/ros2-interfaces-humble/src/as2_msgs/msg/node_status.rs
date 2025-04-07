use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeStatus {
    pub status: i8,
}

impl NodeStatus {
    pub const UNCONFIGURED: i8 = 0;
    pub const INACTIVE: i8 = 1;
    pub const ACTIVE: i8 = 2;
    pub const FINALIZED: i8 = 3;
}

impl Default for NodeStatus {
    fn default() -> Self {
        NodeStatus {
            status: 0,
        }
    }
}

impl ros2_client::Message for NodeStatus {}
