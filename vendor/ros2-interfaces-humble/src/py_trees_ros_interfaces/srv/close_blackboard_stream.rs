use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseBlackboardStreamRequest {
    pub topic_name: ::std::string::String,
}

impl Default for CloseBlackboardStreamRequest {
    fn default() -> Self {
        CloseBlackboardStreamRequest {
            topic_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CloseBlackboardStreamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseBlackboardStreamResponse {
    pub result: bool,
}

impl Default for CloseBlackboardStreamResponse {
    fn default() -> Self {
        CloseBlackboardStreamResponse {
            result: false,
        }
    }
}

impl ros2_client::Message for CloseBlackboardStreamResponse {}


pub struct CloseBlackboardStream;
impl ros2_client::Service for CloseBlackboardStream {
    type Request = CloseBlackboardStreamRequest;
    type Response = CloseBlackboardStreamResponse;

    fn request_type_name(&self) -> &str { "CloseBlackboardStreamRequest" }
    fn response_type_name(&self) -> &str { "CloseBlackboardStreamResponse" }
}
