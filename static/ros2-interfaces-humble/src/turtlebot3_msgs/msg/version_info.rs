use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionInfo {
    pub hardware: ::std::string::String,
    pub firmware: ::std::string::String,
    pub software: ::std::string::String,
}

impl Default for VersionInfo {
    fn default() -> Self {
        VersionInfo {
            hardware: ::std::string::String::new(),
            firmware: ::std::string::String::new(),
            software: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for VersionInfo {}
