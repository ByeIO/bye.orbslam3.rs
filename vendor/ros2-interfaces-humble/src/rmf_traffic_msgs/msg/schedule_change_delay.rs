use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleChangeDelay {
    pub delay: i64,
}

impl Default for ScheduleChangeDelay {
    fn default() -> Self {
        ScheduleChangeDelay {
            delay: 0,
        }
    }
}

impl ros2_client::Message for ScheduleChangeDelay {}
