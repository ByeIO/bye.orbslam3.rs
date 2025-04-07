use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerSystemEvent {
    pub event: u8,
}

impl PowerSystemEvent {
    pub const UNPLUGGED: u8 = 0;
    pub const PLUGGED_TO_ADAPTER: u8 = 1;
    pub const PLUGGED_TO_DOCKBASE: u8 = 2;
    pub const CHARGE_COMPLETED: u8 = 3;
    pub const BATTERY_LOW: u8 = 4;
    pub const BATTERY_CRITICAL: u8 = 5;
}

impl Default for PowerSystemEvent {
    fn default() -> Self {
        PowerSystemEvent {
            event: 0,
        }
    }
}

impl ros2_client::Message for PowerSystemEvent {}
