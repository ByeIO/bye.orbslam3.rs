use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItineraryReached {
    pub participant: u64,
    pub plan: u64,
    pub reached_checkpoints: Vec<u64>,
    pub progress_version: u64,
}

impl Default for ItineraryReached {
    fn default() -> Self {
        ItineraryReached {
            participant: 0,
            plan: 0,
            reached_checkpoints: Vec::new(),
            progress_version: 0,
        }
    }
}

impl ros2_client::Message for ItineraryReached {}
