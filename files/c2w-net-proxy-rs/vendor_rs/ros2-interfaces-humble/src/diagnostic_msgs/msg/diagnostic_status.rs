use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticStatus {
    pub level: u8,
    pub name: ::std::string::String,
    pub message: ::std::string::String,
    pub hardware_id: ::std::string::String,
    pub values: Vec<crate::diagnostic_msgs::msg::KeyValue>,
}

impl DiagnosticStatus {
    pub const OK: u8 = 0;
    pub const WARN: u8 = 1;
    pub const ERROR: u8 = 2;
    pub const STALE: u8 = 3;
}

impl Default for DiagnosticStatus {
    fn default() -> Self {
        DiagnosticStatus {
            level: 0,
            name: ::std::string::String::new(),
            message: ::std::string::String::new(),
            hardware_id: ::std::string::String::new(),
            values: Vec::new(),
        }
    }
}

impl ros2_client::Message for DiagnosticStatus {}
