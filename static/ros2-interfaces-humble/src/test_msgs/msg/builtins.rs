use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Builtins {
    pub duration_value: crate::builtin_interfaces::msg::Duration,
    pub time_value: crate::builtin_interfaces::msg::Time,
}

impl Default for Builtins {
    fn default() -> Self {
        Builtins {
            duration_value: crate::builtin_interfaces::msg::Duration::default(),
            time_value: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for Builtins {}
