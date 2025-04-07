use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "type")]    pub type_: u8,
    pub json_msg: ::std::string::String,
    pub request_id: ::std::string::String,
}

impl ApiResponse {
    pub const TYPE_UNINITIALIZED: u8 = 0;
    pub const TYPE_ACKNOWLEDGE: u8 = 1;
    pub const TYPE_RESPONDING: u8 = 2;
}

impl Default for ApiResponse {
    fn default() -> Self {
        ApiResponse {
            type_: 0,
            json_msg: ::std::string::String::new(),
            request_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ApiResponse {}
