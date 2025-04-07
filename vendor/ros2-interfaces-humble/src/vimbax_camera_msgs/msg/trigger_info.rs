use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerInfo {
    pub selector: ::std::string::String,
    pub mode: ::std::string::String,
    pub source: ::std::string::String,
}

impl Default for TriggerInfo {
    fn default() -> Self {
        TriggerInfo {
            selector: ::std::string::String::new(),
            mode: ::std::string::String::new(),
            source: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TriggerInfo {}
