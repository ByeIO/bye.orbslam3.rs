use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkcellState {
    pub time: crate::builtin_interfaces::msg::Time,
    pub guid: ::std::string::String,
    pub mode: i32,
    pub request_guid_queue: Vec<::std::string::String>,
}

impl WorkcellState {
    pub const IDLE: i32 = 0;
    pub const BUSY: i32 = 1;
    pub const OFFLINE: i32 = 2;
}

impl Default for WorkcellState {
    fn default() -> Self {
        WorkcellState {
            time: crate::builtin_interfaces::msg::Time::default(),
            guid: ::std::string::String::new(),
            mode: 0,
            request_guid_queue: Vec::new(),
        }
    }
}

impl ros2_client::Message for WorkcellState {}
