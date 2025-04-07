use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItineraryErase {
    pub participant: u64,
    pub routes: Vec<u64>,
    pub itinerary_version: u64,
}

impl Default for ItineraryErase {
    fn default() -> Self {
        ItineraryErase {
            participant: 0,
            routes: Vec::new(),
            itinerary_version: 0,
        }
    }
}

impl ros2_client::Message for ItineraryErase {}
