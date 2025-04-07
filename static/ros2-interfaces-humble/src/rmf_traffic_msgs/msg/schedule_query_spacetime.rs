use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleQuerySpacetime {
    #[serde(rename = "type")]    pub type_: u16,
    pub regions: Vec<crate::rmf_traffic_msgs::msg::Region>,
    pub shape_context: crate::rmf_traffic_msgs::msg::ShapeContext,
    pub timespan: crate::rmf_traffic_msgs::msg::Timespan,
}

impl ScheduleQuerySpacetime {
    pub const ALL: u16 = 1;
    pub const REGIONS: u16 = 2;
    pub const TIMESPAN: u16 = 3;
}

impl Default for ScheduleQuerySpacetime {
    fn default() -> Self {
        ScheduleQuerySpacetime {
            type_: 0,
            regions: Vec::new(),
            shape_context: crate::rmf_traffic_msgs::msg::ShapeContext::default(),
            timespan: crate::rmf_traffic_msgs::msg::Timespan::default(),
        }
    }
}

impl ros2_client::Message for ScheduleQuerySpacetime {}
