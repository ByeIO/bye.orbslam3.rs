use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationRepeat {
    pub conflict_version: u64,
    pub table: Vec<u64>,
}

impl Default for NegotiationRepeat {
    fn default() -> Self {
        NegotiationRepeat {
            conflict_version: 0,
            table: Vec::new(),
        }
    }
}

impl ros2_client::Message for NegotiationRepeat {}
