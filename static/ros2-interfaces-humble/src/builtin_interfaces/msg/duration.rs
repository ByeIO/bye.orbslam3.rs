use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Duration {
    pub sec: i32,
    pub nanosec: u32,
}

impl Default for Duration {
    fn default() -> Self {
        Duration {
            sec: 0,
            nanosec: 0,
        }
    }
}

impl ros2_client::Message for Duration {}
