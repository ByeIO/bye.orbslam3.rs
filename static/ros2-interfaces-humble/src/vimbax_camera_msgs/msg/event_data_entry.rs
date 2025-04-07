use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventDataEntry {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for EventDataEntry {
    fn default() -> Self {
        EventDataEntry {
            name: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for EventDataEntry {}
