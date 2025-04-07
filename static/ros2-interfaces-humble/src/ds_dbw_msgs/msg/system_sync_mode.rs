use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemSyncMode {
    pub value: u8,
}

impl SystemSyncMode {
    pub const NONE: u8 = 0;
    pub const DISENGAGES: u8 = 1;
    pub const ALL_OR_NONE: u8 = 2;
    pub const ALL_OR_NONE_WITH_BTN: u8 = 3;
}

impl Default for SystemSyncMode {
    fn default() -> Self {
        SystemSyncMode {
            value: 0,
        }
    }
}

impl ros2_client::Message for SystemSyncMode {}
