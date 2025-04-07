use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupervisorHeartbeat {
    pub all_sessions: Vec<crate::rmf_door_msgs::msg::DoorSessions>,
}

impl Default for SupervisorHeartbeat {
    fn default() -> Self {
        SupervisorHeartbeat {
            all_sessions: Vec::new(),
        }
    }
}

impl ros2_client::Message for SupervisorHeartbeat {}
