use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationProposal {
    pub conflict_version: u64,
    pub proposal_version: u64,
    pub for_participant: u64,
    pub to_accommodate: Vec<crate::rmf_traffic_msgs::msg::NegotiationKey>,
    pub plan_id: u64,
    pub itinerary: Vec<crate::rmf_traffic_msgs::msg::Route>,
}

impl Default for NegotiationProposal {
    fn default() -> Self {
        NegotiationProposal {
            conflict_version: 0,
            proposal_version: 0,
            for_participant: 0,
            to_accommodate: Vec::new(),
            plan_id: 0,
            itinerary: Vec::new(),
        }
    }
}

impl ros2_client::Message for NegotiationProposal {}
