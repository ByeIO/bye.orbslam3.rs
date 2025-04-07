use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformStateMachineEvent {
    pub event: i8,
}

impl PlatformStateMachineEvent {
    pub const EMERGENCY: i8 = -1;
    pub const ARM: i8 = 0;
    pub const DISARM: i8 = 1;
    pub const TAKE_OFF: i8 = 2;
    pub const TOOK_OFF: i8 = 3;
    pub const LAND: i8 = 4;
    pub const LANDED: i8 = 5;
}

impl Default for PlatformStateMachineEvent {
    fn default() -> Self {
        PlatformStateMachineEvent {
            event: 0,
        }
    }
}

impl ros2_client::Message for PlatformStateMachineEvent {}
