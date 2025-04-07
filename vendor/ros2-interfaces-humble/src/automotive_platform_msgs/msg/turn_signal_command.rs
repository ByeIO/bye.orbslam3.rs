use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnSignalCommand {
    pub header: crate::std_msgs::msg::Header,
    pub mode: u16,
    pub turn_signal: u8,
}

impl TurnSignalCommand {
    pub const NONE: u8 = 0;
    pub const LEFT: u8 = 1;
    pub const RIGHT: u8 = 2;
}

impl Default for TurnSignalCommand {
    fn default() -> Self {
        TurnSignalCommand {
            header: crate::std_msgs::msg::Header::default(),
            mode: 0,
            turn_signal: 0,
        }
    }
}

impl ros2_client::Message for TurnSignalCommand {}
