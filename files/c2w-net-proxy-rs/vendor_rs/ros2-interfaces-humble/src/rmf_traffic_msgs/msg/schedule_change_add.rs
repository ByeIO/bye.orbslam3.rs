use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleChangeAdd {
    pub plan_id: u64,
    pub items: Vec<crate::rmf_traffic_msgs::msg::ScheduleChangeAddItem>,
}

impl Default for ScheduleChangeAdd {
    fn default() -> Self {
        ScheduleChangeAdd {
            plan_id: 0,
            items: Vec::new(),
        }
    }
}

impl ros2_client::Message for ScheduleChangeAdd {}
