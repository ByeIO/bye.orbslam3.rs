use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusText {
    pub header: crate::std_msgs::msg::Header,
    pub severity: u8,
    pub text: ::std::string::String,
}

impl StatusText {
    pub const EMERGENCY: u8 = 0;
    pub const ALERT: u8 = 1;
    pub const CRITICAL: u8 = 2;
    pub const ERROR: u8 = 3;
    pub const WARNING: u8 = 4;
    pub const NOTICE: u8 = 5;
    pub const INFO: u8 = 6;
    pub const DEBUG: u8 = 7;
}

impl Default for StatusText {
    fn default() -> Self {
        StatusText {
            header: crate::std_msgs::msg::Header::default(),
            severity: 0,
            text: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StatusText {}
