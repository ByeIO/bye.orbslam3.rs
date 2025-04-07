use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Log {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub level: u8,
    pub name: ::std::string::String,
    pub msg: ::std::string::String,
    pub file: ::std::string::String,
    pub function: ::std::string::String,
    pub line: u32,
}

impl Log {
    pub const DEBUG: u8 = 10;
    pub const INFO: u8 = 20;
    pub const WARN: u8 = 30;
    pub const ERROR: u8 = 40;
    pub const FATAL: u8 = 50;
}

impl Default for Log {
    fn default() -> Self {
        Log {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            level: 0,
            name: ::std::string::String::new(),
            msg: ::std::string::String::new(),
            file: ::std::string::String::new(),
            function: ::std::string::String::new(),
            line: 0,
        }
    }
}

impl ros2_client::Message for Log {}
