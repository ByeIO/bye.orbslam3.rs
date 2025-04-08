use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleChangeAddItem {
    pub route_id: u64,
    pub storage_id: u64,
    pub route: crate::rmf_traffic_msgs::msg::Route,
}

impl Default for ScheduleChangeAddItem {
    fn default() -> Self {
        ScheduleChangeAddItem {
            route_id: 0,
            storage_id: 0,
            route: crate::rmf_traffic_msgs::msg::Route::default(),
        }
    }
}

impl ros2_client::Message for ScheduleChangeAddItem {}
