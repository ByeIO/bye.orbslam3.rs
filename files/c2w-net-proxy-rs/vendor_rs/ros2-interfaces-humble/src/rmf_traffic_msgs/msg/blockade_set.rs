use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockadeSet {
    pub participant: u64,
    pub reservation: u64,
    pub radius: f64,
    pub path: Vec<crate::rmf_traffic_msgs::msg::BlockadeCheckpoint>,
}

impl Default for BlockadeSet {
    fn default() -> Self {
        BlockadeSet {
            participant: 0,
            reservation: 0,
            radius: 0.0,
            path: Vec::new(),
        }
    }
}

impl ros2_client::Message for BlockadeSet {}
