use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationRefusal {
    pub conflict_version: u64,
}

impl Default for NegotiationRefusal {
    fn default() -> Self {
        NegotiationRefusal {
            conflict_version: 0,
        }
    }
}

impl ros2_client::Message for NegotiationRefusal {}
