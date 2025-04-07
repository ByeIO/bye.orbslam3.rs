use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationState {
    pub status: crate::rmf_traffic_msgs::msg::NegotiationStatus,
    pub tree: Vec<crate::rmf_traffic_msgs::msg::NegotiationTreeNode>,
    pub orphan_proposals: Vec<crate::rmf_traffic_msgs::msg::NegotiationProposal>,
    pub orphan_rejections: Vec<crate::rmf_traffic_msgs::msg::NegotiationRejection>,
    pub orphan_forfeits: Vec<crate::rmf_traffic_msgs::msg::NegotiationForfeit>,
}

impl Default for NegotiationState {
    fn default() -> Self {
        NegotiationState {
            status: crate::rmf_traffic_msgs::msg::NegotiationStatus::default(),
            tree: Vec::new(),
            orphan_proposals: Vec::new(),
            orphan_rejections: Vec::new(),
            orphan_forfeits: Vec::new(),
        }
    }
}

impl ros2_client::Message for NegotiationState {}
