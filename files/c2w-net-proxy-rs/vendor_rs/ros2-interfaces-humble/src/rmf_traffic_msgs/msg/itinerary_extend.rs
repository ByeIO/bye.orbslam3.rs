use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItineraryExtend {
    pub participant: u64,
    pub routes: Vec<crate::rmf_traffic_msgs::msg::Route>,
    pub itinerary_version: u64,
}

impl Default for ItineraryExtend {
    fn default() -> Self {
        ItineraryExtend {
            participant: 0,
            routes: Vec::new(),
            itinerary_version: 0,
        }
    }
}

impl ros2_client::Message for ItineraryExtend {}
