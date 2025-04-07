use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenBlackboardStreamRequest {
    pub variables: Vec<::std::string::String>,
    pub filter_on_visited_path: bool,
    pub with_activity_stream: bool,
}

impl Default for OpenBlackboardStreamRequest {
    fn default() -> Self {
        OpenBlackboardStreamRequest {
            variables: Vec::new(),
            filter_on_visited_path: false,
            with_activity_stream: false,
        }
    }
}

impl ros2_client::Message for OpenBlackboardStreamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenBlackboardStreamResponse {
    pub topic: ::std::string::String,
}

impl Default for OpenBlackboardStreamResponse {
    fn default() -> Self {
        OpenBlackboardStreamResponse {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for OpenBlackboardStreamResponse {}


pub struct OpenBlackboardStream;
impl ros2_client::Service for OpenBlackboardStream {
    type Request = OpenBlackboardStreamRequest;
    type Response = OpenBlackboardStreamResponse;

    fn request_type_name(&self) -> &str { "OpenBlackboardStreamRequest" }
    fn response_type_name(&self) -> &str { "OpenBlackboardStreamResponse" }
}
