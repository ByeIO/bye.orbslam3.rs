use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Itinerary {
    pub routes: Vec<crate::rmf_traffic_msgs::msg::Route>,
}

impl Default for Itinerary {
    fn default() -> Self {
        Itinerary {
            routes: Vec::new(),
        }
    }
}

impl ros2_client::Message for Itinerary {}
