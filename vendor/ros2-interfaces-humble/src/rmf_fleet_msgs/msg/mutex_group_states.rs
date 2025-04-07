use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutexGroupStates {
    pub assignments: Vec<crate::rmf_fleet_msgs::msg::MutexGroupAssignment>,
}

impl Default for MutexGroupStates {
    fn default() -> Self {
        MutexGroupStates {
            assignments: Vec::new(),
        }
    }
}

impl ros2_client::Message for MutexGroupStates {}
