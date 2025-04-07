use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleState {
    pub name: ::std::string::String,
    pub last_modified: i64,
    pub last_ran: i64,
    pub next_run: i64,
    pub status: i8,
}

impl ScheduleState {
    pub const CREATED: i8 = 1;
    pub const STARTED: i8 = 2;
    pub const FINISHED: i8 = 3;
    pub const CANCELLED: i8 = 4;
    pub const FAILED: i8 = -1;
}

impl Default for ScheduleState {
    fn default() -> Self {
        ScheduleState {
            name: ::std::string::String::new(),
            last_modified: 0,
            last_ran: 0,
            next_run: 0,
            status: 0,
        }
    }
}

impl ros2_client::Message for ScheduleState {}
