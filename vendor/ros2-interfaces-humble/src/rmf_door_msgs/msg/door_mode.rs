use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoorMode {
    pub value: u32,
}

impl DoorMode {
    pub const MODE_CLOSED: u32 = 0;
    pub const MODE_MOVING: u32 = 1;
    pub const MODE_OPEN: u32 = 2;
    pub const MODE_OFFLINE: u32 = 3;
    pub const MODE_UNKNOWN: u32 = 4;
}

impl Default for DoorMode {
    fn default() -> Self {
        DoorMode {
            value: 0,
        }
    }
}

impl ros2_client::Message for DoorMode {}
