use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleInconsistency {
    pub participant: u64,
    pub ranges: Vec<crate::rmf_traffic_msgs::msg::ScheduleInconsistencyRange>,
    pub last_known_itinerary: u64,
    pub last_known_progress: u64,
}

impl Default for ScheduleInconsistency {
    fn default() -> Self {
        ScheduleInconsistency {
            participant: 0,
            ranges: Vec::new(),
            last_known_itinerary: 0,
            last_known_progress: 0,
        }
    }
}

impl ros2_client::Message for ScheduleInconsistency {}
