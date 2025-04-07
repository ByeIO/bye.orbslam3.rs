use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Engage {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub engage: bool,
}

impl Default for Engage {
    fn default() -> Self {
        Engage {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            engage: false,
        }
    }
}

impl ros2_client::Message for Engage {}
