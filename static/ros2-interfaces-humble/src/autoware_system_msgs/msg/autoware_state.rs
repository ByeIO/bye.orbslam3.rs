use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutowareState {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub state: u8,
}

impl AutowareState {
    pub const INITIALIZING: u8 = 1;
    pub const WAITING_FOR_ROUTE: u8 = 2;
    pub const PLANNING: u8 = 3;
    pub const WAITING_FOR_ENGAGE: u8 = 4;
    pub const DRIVING: u8 = 5;
    pub const ARRIVED_GOAL: u8 = 6;
    pub const FINALIZING: u8 = 7;
}

impl Default for AutowareState {
    fn default() -> Self {
        AutowareState {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            state: 0,
        }
    }
}

impl ros2_client::Message for AutowareState {}
