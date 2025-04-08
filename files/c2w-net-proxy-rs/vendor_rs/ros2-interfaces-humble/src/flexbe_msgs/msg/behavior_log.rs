use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorLog {
    pub text: ::std::string::String,
    pub status_code: u8,
}

impl BehaviorLog {
    pub const INFO: u8 = 0;
    pub const WARN: u8 = 1;
    pub const HINT: u8 = 2;
    pub const ERROR: u8 = 3;
    pub const DEBUG: u8 = 10;
}

impl Default for BehaviorLog {
    fn default() -> Self {
        BehaviorLog {
            text: ::std::string::String::new(),
            status_code: 0,
        }
    }
}

impl ros2_client::Message for BehaviorLog {}
