use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleQueries {
    pub node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity,
    pub queries: Vec<crate::rmf_traffic_msgs::msg::ScheduleQuery>,
    pub query_ids: Vec<u64>,
}

impl Default for ScheduleQueries {
    fn default() -> Self {
        ScheduleQueries {
            node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity::default(),
            queries: Vec::new(),
            query_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for ScheduleQueries {}
