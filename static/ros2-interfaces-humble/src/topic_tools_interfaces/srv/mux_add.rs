use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxAddRequest {
    pub topic: ::std::string::String,
}

impl Default for MuxAddRequest {
    fn default() -> Self {
        MuxAddRequest {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MuxAddRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxAddResponse {
    pub success: bool,
}

impl Default for MuxAddResponse {
    fn default() -> Self {
        MuxAddResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for MuxAddResponse {}


pub struct MuxAdd;
impl ros2_client::Service for MuxAdd {
    type Request = MuxAddRequest;
    type Response = MuxAddResponse;

    fn request_type_name(&self) -> &str { "MuxAddRequest" }
    fn response_type_name(&self) -> &str { "MuxAddResponse" }
}
