use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BEStatus {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub behavior_id: i32,
    pub code: u8,
    pub args: Vec<::std::string::String>,
}

impl BEStatus {
    pub const STARTED: u8 = 0;
    pub const FINISHED: u8 = 1;
    pub const FAILED: u8 = 2;
    pub const LOCKED: u8 = 4;
    pub const WAITING: u8 = 5;
    pub const SWITCHING: u8 = 6;
    pub const WARNING: u8 = 10;
    pub const ERROR: u8 = 11;
    pub const READY: u8 = 20;
    pub const RUNNING: u8 = 30;
}

impl Default for BEStatus {
    fn default() -> Self {
        BEStatus {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            behavior_id: 0,
            code: 0,
            args: Vec::new(),
        }
    }
}

impl ros2_client::Message for BEStatus {}
