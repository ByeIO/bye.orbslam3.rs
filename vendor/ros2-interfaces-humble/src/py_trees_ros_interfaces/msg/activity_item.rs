use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivityItem {
    pub key: ::std::string::String,
    pub client_name: ::std::string::String,
    pub activity_type: ::std::string::String,
    pub previous_value: ::std::string::String,
    pub current_value: ::std::string::String,
}

impl ActivityItem {
    pub const READ: &'static str = "READ";
    pub const INITIALISED: &'static str = "INITIALISED";
    pub const WRITE: &'static str = "WRITE";
    pub const ACCESSED: &'static str = "ACCESSED";
    pub const ACCESS_DENIED: &'static str = "ACCESS_DENIED";
    pub const NO_KEY: &'static str = "NO_KEY";
    pub const NO_OVERWRITE: &'static str = "NO_OVERWRITE";
    pub const UNSET: &'static str = "UNSET";
}

impl Default for ActivityItem {
    fn default() -> Self {
        ActivityItem {
            key: ::std::string::String::new(),
            client_name: ::std::string::String::new(),
            activity_type: ::std::string::String::new(),
            previous_value: ::std::string::String::new(),
            current_value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ActivityItem {}
