use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriverCommands {
    pub msg_counter: u8,
    pub engage: u16,
    pub disengage: u16,
    pub speed_up: u16,
    pub slow_down: u16,
    pub further: u16,
    pub closer: u16,
    pub right_turn: u16,
    pub left_turn: u16,
}

impl Default for DriverCommands {
    fn default() -> Self {
        DriverCommands {
            msg_counter: 0,
            engage: 0,
            disengage: 0,
            speed_up: 0,
            slow_down: 0,
            further: 0,
            closer: 0,
            right_turn: 0,
            left_turn: 0,
        }
    }
}

impl ros2_client::Message for DriverCommands {}
