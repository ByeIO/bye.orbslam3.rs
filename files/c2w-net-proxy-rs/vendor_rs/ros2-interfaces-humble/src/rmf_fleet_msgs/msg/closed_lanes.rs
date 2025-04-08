use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosedLanes {
    pub fleet_name: ::std::string::String,
    pub closed_lanes: Vec<u64>,
}

impl Default for ClosedLanes {
    fn default() -> Self {
        ClosedLanes {
            fleet_name: ::std::string::String::new(),
            closed_lanes: Vec::new(),
        }
    }
}

impl ros2_client::Message for ClosedLanes {}
