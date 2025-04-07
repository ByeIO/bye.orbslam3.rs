use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirrorUpdate {
    pub node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity,
    pub database_version: u64,
    pub patch: crate::rmf_traffic_msgs::msg::SchedulePatch,
    pub is_remedial_update: bool,
}

impl Default for MirrorUpdate {
    fn default() -> Self {
        MirrorUpdate {
            node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity::default(),
            database_version: 0,
            patch: crate::rmf_traffic_msgs::msg::SchedulePatch::default(),
            is_remedial_update: false,
        }
    }
}

impl ros2_client::Message for MirrorUpdate {}
