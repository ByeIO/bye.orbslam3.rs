use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerState {
    pub name: ::std::string::String,
    pub last_modified: i64,
    pub last_ran: i64,
    pub status: i8,
}

impl TriggerState {
    pub const STARTED: i8 = 2;
    pub const FINISHED: i8 = 3;
    pub const CANCELLED: i8 = 4;
    pub const FAILED: i8 = -1;
}

impl Default for TriggerState {
    fn default() -> Self {
        TriggerState {
            name: ::std::string::String::new(),
            last_modified: 0,
            last_ran: 0,
            status: 0,
        }
    }
}

impl ros2_client::Message for TriggerState {}
