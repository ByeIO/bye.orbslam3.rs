use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendBytesRequest {
    pub count: i64,
}

impl Default for SendBytesRequest {
    fn default() -> Self {
        SendBytesRequest {
            count: 0,
        }
    }
}

impl ros2_client::Message for SendBytesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendBytesResponse {
    pub data: ::std::string::String,
}

impl Default for SendBytesResponse {
    fn default() -> Self {
        SendBytesResponse {
            data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SendBytesResponse {}


pub struct SendBytes;
impl ros2_client::Service for SendBytes {
    type Request = SendBytesRequest;
    type Response = SendBytesResponse;

    fn request_type_name(&self) -> &str { "SendBytesRequest" }
    fn response_type_name(&self) -> &str { "SendBytesResponse" }
}
