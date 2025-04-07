use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationConclusion {
    pub conflict_version: u64,
    pub resolved: bool,
    pub table: Vec<crate::rmf_traffic_msgs::msg::NegotiationKey>,
}

impl Default for NegotiationConclusion {
    fn default() -> Self {
        NegotiationConclusion {
            conflict_version: 0,
            resolved: false,
            table: Vec::new(),
        }
    }
}

impl ros2_client::Message for NegotiationConclusion {}
