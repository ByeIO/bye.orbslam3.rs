use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleIdentity {
    pub node_uuid: ::std::string::String,
    pub timestamp: crate::builtin_interfaces::msg::Time,
}

impl Default for ScheduleIdentity {
    fn default() -> Self {
        ScheduleIdentity {
            node_uuid: ::std::string::String::new(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for ScheduleIdentity {}
