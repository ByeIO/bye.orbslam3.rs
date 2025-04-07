use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationTreeNode {
    pub parent: i64,
    pub key: crate::rmf_traffic_msgs::msg::NegotiationKey,
    pub rejected: bool,
    pub itinerary: Vec<crate::rmf_traffic_msgs::msg::Route>,
}

impl Default for NegotiationTreeNode {
    fn default() -> Self {
        NegotiationTreeNode {
            parent: 0,
            key: crate::rmf_traffic_msgs::msg::NegotiationKey::default(),
            rejected: false,
            itinerary: Vec::new(),
        }
    }
}

impl ros2_client::Message for NegotiationTreeNode {}
