use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct String {
    pub data: ::std::string::String,
}

impl Default for String {
    fn default() -> Self {
        String {
            data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for String {}
