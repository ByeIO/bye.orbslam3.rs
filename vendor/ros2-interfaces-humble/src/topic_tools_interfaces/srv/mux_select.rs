use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxSelectRequest {
    pub topic: ::std::string::String,
}

impl Default for MuxSelectRequest {
    fn default() -> Self {
        MuxSelectRequest {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MuxSelectRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxSelectResponse {
    pub prev_topic: ::std::string::String,
    pub success: bool,
}

impl Default for MuxSelectResponse {
    fn default() -> Self {
        MuxSelectResponse {
            prev_topic: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for MuxSelectResponse {}


pub struct MuxSelect;
impl ros2_client::Service for MuxSelect {
    type Request = MuxSelectRequest;
    type Response = MuxSelectResponse;

    fn request_type_name(&self) -> &str { "MuxSelectRequest" }
    fn response_type_name(&self) -> &str { "MuxSelectResponse" }
}
