use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionExecutionInfo {
    pub status: i8,
    pub start_stamp: crate::builtin_interfaces::msg::Time,
    pub status_stamp: crate::builtin_interfaces::msg::Time,
    pub action_full_name: ::std::string::String,
    pub action: ::std::string::String,
    pub arguments: Vec<::std::string::String>,
    pub duration: crate::builtin_interfaces::msg::Duration,
    pub completion: f32,
    pub message_status: ::std::string::String,
}

impl ActionExecutionInfo {
    pub const NOT_EXECUTED: i8 = 1;
    pub const EXECUTING: i8 = 2;
    pub const FAILED: i8 = 3;
    pub const SUCCEEDED: i8 = 4;
    pub const CANCELLED: i8 = 5;
}

impl Default for ActionExecutionInfo {
    fn default() -> Self {
        ActionExecutionInfo {
            status: 0,
            start_stamp: crate::builtin_interfaces::msg::Time::default(),
            status_stamp: crate::builtin_interfaces::msg::Time::default(),
            action_full_name: ::std::string::String::new(),
            action: ::std::string::String::new(),
            arguments: Vec::new(),
            duration: crate::builtin_interfaces::msg::Duration::default(),
            completion: 0.0,
            message_status: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ActionExecutionInfo {}
