use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CliffEvent {
    pub sensor: u8,
    pub state: u8,
    pub bottom: u16,
}

impl CliffEvent {
    pub const LEFT: u8 = 0;
    pub const CENTER: u8 = 1;
    pub const RIGHT: u8 = 2;
    pub const FLOOR: u8 = 0;
    pub const CLIFF: u8 = 1;
}

impl Default for CliffEvent {
    fn default() -> Self {
        CliffEvent {
            sensor: 0,
            state: 0,
            bottom: 0,
        }
    }
}

impl ros2_client::Message for CliffEvent {}
