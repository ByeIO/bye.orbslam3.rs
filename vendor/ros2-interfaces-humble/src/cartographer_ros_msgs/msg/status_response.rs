use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusResponse {
    pub code: u8,
    pub message: ::std::string::String,
}

impl Default for StatusResponse {
    fn default() -> Self {
        StatusResponse {
            code: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StatusResponse {}
