use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItinerarySet {
    pub participant: u64,
    pub plan: u64,
    pub itinerary: Vec<crate::rmf_traffic_msgs::msg::Route>,
    pub storage_base: u64,
    pub itinerary_version: u64,
}

impl Default for ItinerarySet {
    fn default() -> Self {
        ItinerarySet {
            participant: 0,
            plan: 0,
            itinerary: Vec::new(),
            storage_base: 0,
            itinerary_version: 0,
        }
    }
}

impl ros2_client::Message for ItinerarySet {}
