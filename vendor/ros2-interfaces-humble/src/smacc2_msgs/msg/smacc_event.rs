use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccEvent {
    pub event_type: ::std::string::String,
    pub event_object_tag: ::std::string::String,
    pub event_source: ::std::string::String,
    pub label: ::std::string::String,
}

impl Default for SmaccEvent {
    fn default() -> Self {
        SmaccEvent {
            event_type: ::std::string::String::new(),
            event_object_tag: ::std::string::String::new(),
            event_source: ::std::string::String::new(),
            label: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SmaccEvent {}
