use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleParticipantPatch {
    pub participant_id: u64,
    pub itinerary_version: u64,
    pub erasures: Vec<u64>,
    pub delays: Vec<crate::rmf_traffic_msgs::msg::ScheduleChangeDelay>,
    pub additions: crate::rmf_traffic_msgs::msg::ScheduleChangeAdd,
    pub progress: crate::rmf_traffic_msgs::msg::ScheduleChangeProgress,
}

impl Default for ScheduleParticipantPatch {
    fn default() -> Self {
        ScheduleParticipantPatch {
            participant_id: 0,
            itinerary_version: 0,
            erasures: Vec::new(),
            delays: Vec::new(),
            additions: crate::rmf_traffic_msgs::msg::ScheduleChangeAdd::default(),
            progress: crate::rmf_traffic_msgs::msg::ScheduleChangeProgress::default(),
        }
    }
}

impl ros2_client::Message for ScheduleParticipantPatch {}
