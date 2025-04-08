use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispenserResult {
    pub time: crate::builtin_interfaces::msg::Time,
    pub request_guid: ::std::string::String,
    pub source_guid: ::std::string::String,
    pub status: u8,
}

impl DispenserResult {
    pub const ACKNOWLEDGED: u8 = 0;
    pub const SUCCESS: u8 = 1;
    pub const FAILED: u8 = 2;
}

impl Default for DispenserResult {
    fn default() -> Self {
        DispenserResult {
            time: crate::builtin_interfaces::msg::Time::default(),
            request_guid: ::std::string::String::new(),
            source_guid: ::std::string::String::new(),
            status: 0,
        }
    }
}

impl ros2_client::Message for DispenserResult {}
