use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BumperEvent {
    pub bumper: u8,
    pub state: u8,
}

impl BumperEvent {
    pub const LEFT: u8 = 0;
    pub const CENTER: u8 = 1;
    pub const RIGHT: u8 = 2;
    pub const RELEASED: u8 = 0;
    pub const PRESSED: u8 = 1;
}

impl Default for BumperEvent {
    fn default() -> Self {
        BumperEvent {
            bumper: 0,
            state: 0,
        }
    }
}

impl ros2_client::Message for BumperEvent {}
