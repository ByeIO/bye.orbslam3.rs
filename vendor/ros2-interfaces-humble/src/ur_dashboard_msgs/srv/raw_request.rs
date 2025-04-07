use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawRequestRequest {
    pub query: ::std::string::String,
}

impl Default for RawRequestRequest {
    fn default() -> Self {
        RawRequestRequest {
            query: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RawRequestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawRequestResponse {
    pub answer: ::std::string::String,
}

impl Default for RawRequestResponse {
    fn default() -> Self {
        RawRequestResponse {
            answer: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RawRequestResponse {}


pub struct RawRequest;
impl ros2_client::Service for RawRequest {
    type Request = RawRequestRequest;
    type Response = RawRequestResponse;

    fn request_type_name(&self) -> &str { "RawRequestRequest" }
    fn response_type_name(&self) -> &str { "RawRequestResponse" }
}
