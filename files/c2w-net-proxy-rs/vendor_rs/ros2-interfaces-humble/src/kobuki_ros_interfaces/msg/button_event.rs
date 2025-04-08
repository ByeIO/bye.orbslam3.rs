use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ButtonEvent {
    pub button: u8,
    pub state: u8,
}

impl ButtonEvent {
    pub const BUTTON0: u8 = 0;
    pub const BUTTON1: u8 = 1;
    pub const BUTTON2: u8 = 2;
    pub const RELEASED: u8 = 0;
    pub const PRESSED: u8 = 1;
}

impl Default for ButtonEvent {
    fn default() -> Self {
        ButtonEvent {
            button: 0,
            state: 0,
        }
    }
}

impl ros2_client::Message for ButtonEvent {}
