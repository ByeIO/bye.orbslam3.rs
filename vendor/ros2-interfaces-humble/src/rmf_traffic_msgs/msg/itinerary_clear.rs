use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItineraryClear {
    pub participant: u64,
    pub itinerary_version: u64,
}

impl Default for ItineraryClear {
    fn default() -> Self {
        ItineraryClear {
            participant: 0,
            itinerary_version: 0,
        }
    }
}

impl ros2_client::Message for ItineraryClear {}
