use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleQueryParticipants {
    #[serde(rename = "type")]    pub type_: u16,
    pub ids: Vec<u64>,
}

impl ScheduleQueryParticipants {
    pub const ALL: u16 = 1;
    pub const INCLUDE: u16 = 2;
    pub const EXCLUDE: u16 = 3;
}

impl Default for ScheduleQueryParticipants {
    fn default() -> Self {
        ScheduleQueryParticipants {
            type_: 0,
            ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for ScheduleQueryParticipants {}
