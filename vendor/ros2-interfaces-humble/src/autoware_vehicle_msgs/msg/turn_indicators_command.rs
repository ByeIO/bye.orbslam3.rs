use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnIndicatorsCommand {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub command: u8,
}

impl TurnIndicatorsCommand {
    pub const NO_COMMAND: u8 = 0;
    pub const DISABLE: u8 = 1;
    pub const ENABLE_LEFT: u8 = 2;
    pub const ENABLE_RIGHT: u8 = 3;
}

impl Default for TurnIndicatorsCommand {
    fn default() -> Self {
        TurnIndicatorsCommand {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            command: 0,
        }
    }
}

impl ros2_client::Message for TurnIndicatorsCommand {}
