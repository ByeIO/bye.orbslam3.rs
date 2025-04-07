use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for KeyValue {
    fn default() -> Self {
        KeyValue {
            key: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for KeyValue {}
