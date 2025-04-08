use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clock {
    pub clock: crate::builtin_interfaces::msg::Time,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            clock: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for Clock {}
