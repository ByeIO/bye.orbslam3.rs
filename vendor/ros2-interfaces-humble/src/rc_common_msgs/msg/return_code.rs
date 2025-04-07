use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnCode {
    pub value: i16,
    pub message: ::std::string::String,
}

impl Default for ReturnCode {
    fn default() -> Self {
        ReturnCode {
            value: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReturnCode {}
