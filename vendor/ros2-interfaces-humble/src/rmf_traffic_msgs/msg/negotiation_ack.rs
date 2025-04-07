use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationAck {
    pub conflict_version: u64,
    pub acknowledgments: Vec<crate::rmf_traffic_msgs::msg::NegotiationParticipantAck>,
}

impl Default for NegotiationAck {
    fn default() -> Self {
        NegotiationAck {
            conflict_version: 0,
            acknowledgments: Vec::new(),
        }
    }
}

impl ros2_client::Message for NegotiationAck {}
