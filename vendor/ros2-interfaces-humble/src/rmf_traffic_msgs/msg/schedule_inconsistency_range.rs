use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleInconsistencyRange {
    pub lower: u64,
    pub upper: u64,
}

impl Default for ScheduleInconsistencyRange {
    fn default() -> Self {
        ScheduleInconsistencyRange {
            lower: 0,
            upper: 0,
        }
    }
}

impl ros2_client::Message for ScheduleInconsistencyRange {}
