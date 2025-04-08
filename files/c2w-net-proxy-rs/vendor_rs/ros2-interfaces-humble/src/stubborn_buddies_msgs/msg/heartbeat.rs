use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Heartbeat {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub checkpoint_id: u16, // default: 0
}

impl Default for Heartbeat {
    fn default() -> Self {
        Heartbeat {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            checkpoint_id: 0,
        }
    }
}

impl ros2_client::Message for Heartbeat {}
