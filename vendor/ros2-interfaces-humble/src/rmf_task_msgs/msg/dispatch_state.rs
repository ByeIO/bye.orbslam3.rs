use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispatchState {
    pub task_id: ::std::string::String,
    pub status: i8,
    pub assignment: crate::rmf_task_msgs::msg::Assignment,
    pub errors: Vec<::std::string::String>,
}

impl DispatchState {
    pub const STATUS_UNINITIALIZED: u8 = 0;
    pub const STATUS_QUEUED: u8 = 1;
    pub const STATUS_SELECTED: u8 = 2;
    pub const STATUS_DISPATCHED: u8 = 3;
    pub const STATUS_FAILED_TO_ASSIGN: u8 = 4;
    pub const STATUS_CANCELED_IN_FLIGHT: u8 = 5;
}

impl Default for DispatchState {
    fn default() -> Self {
        DispatchState {
            task_id: ::std::string::String::new(),
            status: 0,
            assignment: crate::rmf_task_msgs::msg::Assignment::default(),
            errors: Vec::new(),
        }
    }
}

impl ros2_client::Message for DispatchState {}
