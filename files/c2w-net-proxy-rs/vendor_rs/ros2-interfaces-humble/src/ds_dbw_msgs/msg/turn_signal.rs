use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnSignal {
    pub value: u8,
}

impl TurnSignal {
    pub const NONE: u8 = 0;
    pub const LEFT: u8 = 1;
    pub const RIGHT: u8 = 2;
    pub const HAZARD: u8 = 3;
}

impl Default for TurnSignal {
    fn default() -> Self {
        TurnSignal {
            value: 0,
        }
    }
}

impl ros2_client::Message for TurnSignal {}
