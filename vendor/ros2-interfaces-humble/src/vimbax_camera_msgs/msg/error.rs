use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub text: ::std::string::String,
}

impl Default for Error {
    fn default() -> Self {
        Error {
            code: 0,
            text: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Error {}
