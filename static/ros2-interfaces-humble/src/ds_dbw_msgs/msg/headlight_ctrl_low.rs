use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadlightCtrlLow {
    pub value: u8,
}

impl HeadlightCtrlLow {
    pub const UNKNOWN: u8 = 0;
    pub const OFF: u8 = 1;
    pub const ON: u8 = 2;
    pub const AUTO: u8 = 3;
    pub const PARK: u8 = 4;
}

impl Default for HeadlightCtrlLow {
    fn default() -> Self {
        HeadlightCtrlLow {
            value: 0,
        }
    }
}

impl ros2_client::Message for HeadlightCtrlLow {}
