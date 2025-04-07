use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WString {
    pub data: ::std::string::String,
}

impl Default for WString {
    fn default() -> Self {
        WString {
            data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for WString {}
