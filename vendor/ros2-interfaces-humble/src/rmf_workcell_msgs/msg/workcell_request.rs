use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkcellRequest {
    pub time: crate::builtin_interfaces::msg::Time,
    pub request_guid: ::std::string::String,
    pub target_guid: ::std::string::String,
}

impl Default for WorkcellRequest {
    fn default() -> Self {
        WorkcellRequest {
            time: crate::builtin_interfaces::msg::Time::default(),
            request_guid: ::std::string::String::new(),
            target_guid: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for WorkcellRequest {}
