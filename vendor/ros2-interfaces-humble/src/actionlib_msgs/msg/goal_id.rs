use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalID {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub id: ::std::string::String,
}

impl Default for GoalID {
    fn default() -> Self {
        GoalID {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GoalID {}
