use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub missed_number: u16, // default: 0
}

impl Default for Status {
    fn default() -> Self {
        Status {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            missed_number: 0,
        }
    }
}

impl ros2_client::Message for Status {}
