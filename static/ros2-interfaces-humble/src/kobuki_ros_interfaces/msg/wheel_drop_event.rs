use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelDropEvent {
    pub wheel: u8,
    pub state: u8,
}

impl WheelDropEvent {
    pub const LEFT: u8 = 0;
    pub const RIGHT: u8 = 1;
    pub const RAISED: u8 = 0;
    pub const DROPPED: u8 = 1;
}

impl Default for WheelDropEvent {
    fn default() -> Self {
        WheelDropEvent {
            wheel: 0,
            state: 0,
        }
    }
}

impl ros2_client::Message for WheelDropEvent {}
