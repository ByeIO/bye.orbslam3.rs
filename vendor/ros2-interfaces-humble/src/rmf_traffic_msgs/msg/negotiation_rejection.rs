use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationRejection {
    pub conflict_version: u64,
    pub table: Vec<crate::rmf_traffic_msgs::msg::NegotiationKey>,
    pub rejected_by: u64,
    pub alternatives: Vec<crate::rmf_traffic_msgs::msg::Itinerary>,
}

impl Default for NegotiationRejection {
    fn default() -> Self {
        NegotiationRejection {
            conflict_version: 0,
            table: Vec::new(),
            rejected_by: 0,
            alternatives: Vec::new(),
        }
    }
}

impl ros2_client::Message for NegotiationRejection {}
