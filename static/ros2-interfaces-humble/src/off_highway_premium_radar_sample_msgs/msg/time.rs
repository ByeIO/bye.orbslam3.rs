use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub sec: u32,
    pub nanosec: u32,
}

impl Default for Time {
    fn default() -> Self {
        Time {
            sec: 0,
            nanosec: 0,
        }
    }
}

impl ros2_client::Message for Time {}
