use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnIndicatorsReport {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub report: u8,
}

impl TurnIndicatorsReport {
    pub const DISABLE: u8 = 1;
    pub const ENABLE_LEFT: u8 = 2;
    pub const ENABLE_RIGHT: u8 = 3;
}

impl Default for TurnIndicatorsReport {
    fn default() -> Self {
        TurnIndicatorsReport {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            report: 0,
        }
    }
}

impl ros2_client::Message for TurnIndicatorsReport {}
