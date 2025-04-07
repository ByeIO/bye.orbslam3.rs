use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardInput {
    pub pressed_key: u8,
}

impl KeyboardInput {
    pub const KEYCODE_RIGHT: u8 = 67;
    pub const KEYCODE_LEFT: u8 = 68;
    pub const KEYCODE_UP: u8 = 65;
    pub const KEYCODE_DOWN: u8 = 66;
    pub const KEYCODE_SPACE: u8 = 32;
    pub const KEYCODE_ENABLE: u8 = 101;
    pub const KEYCODE_DISABLE: u8 = 100;
}

impl Default for KeyboardInput {
    fn default() -> Self {
        KeyboardInput {
            pressed_key: 0,
        }
    }
}

impl ros2_client::Message for KeyboardInput {}
