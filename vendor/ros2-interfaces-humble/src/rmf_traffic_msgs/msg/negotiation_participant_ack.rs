use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationParticipantAck {
    pub participant: u64,
    pub updating: bool, // default: false
    pub itinerary_version: u64,
}

impl Default for NegotiationParticipantAck {
    fn default() -> Self {
        NegotiationParticipantAck {
            participant: 0,
            updating: false,
            itinerary_version: 0,
        }
    }
}

impl ros2_client::Message for NegotiationParticipantAck {}
