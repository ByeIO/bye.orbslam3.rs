use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Participants {
    pub node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity,
    pub participants: Vec<crate::rmf_traffic_msgs::msg::Participant>,
}

impl Default for Participants {
    fn default() -> Self {
        Participants {
            node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity::default(),
            participants: Vec::new(),
        }
    }
}

impl ros2_client::Message for Participants {}
