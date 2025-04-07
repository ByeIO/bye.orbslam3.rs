use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmdSrc {
    pub value: u8,
}

impl CmdSrc {
    pub const USER: u8 = 0;
    pub const ULC: u8 = 1;
    pub const REMOTE: u8 = 2;
    pub const BUTTON: u8 = 3;
    pub const COMMS_LOSS: u8 = 4;
    pub const LOCKOUT: u8 = 5;
}

impl Default for CmdSrc {
    fn default() -> Self {
        CmdSrc {
            value: 0,
        }
    }
}

impl ros2_client::Message for CmdSrc {}
