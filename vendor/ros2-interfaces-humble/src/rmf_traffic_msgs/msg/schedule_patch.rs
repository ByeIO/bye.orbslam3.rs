use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchedulePatch {
    pub participants: Vec<crate::rmf_traffic_msgs::msg::ScheduleParticipantPatch>,
    pub cull: Vec<crate::rmf_traffic_msgs::msg::ScheduleChangeCull>,
    pub has_base_version: bool,
    pub base_version: u64,
    pub latest_version: u64,
}

impl Default for SchedulePatch {
    fn default() -> Self {
        SchedulePatch {
            participants: Vec::new(),
            cull: Vec::new(),
            has_base_version: false,
            base_version: 0,
            latest_version: 0,
        }
    }
}

impl ros2_client::Message for SchedulePatch {}
