use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleChangeCull {
    pub time: i64,
}

impl Default for ScheduleChangeCull {
    fn default() -> Self {
        ScheduleChangeCull {
            time: 0,
        }
    }
}

impl ros2_client::Message for ScheduleChangeCull {}
